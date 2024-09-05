use moss_std::collection::{BTreeMap, BTreeSet};
use parking_lot::Mutex;
use std::fmt::Debug;
use std::mem;
use std::ops::AddAssign;
use std::{cell::Cell, rc::Rc, sync::Arc};

pub struct Subscription<'a> {
    unsubscribe: Option<Box<dyn FnOnce() + 'a>>,
}

impl<'a> Subscription<'a> {
    pub fn new(unsubscribe: impl FnOnce() + 'a) -> Self {
        Self {
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }

    pub fn detach(mut self) {
        self.unsubscribe.take();
    }
}

impl<'a> Drop for Subscription<'a> {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.take() {
            unsubscribe();
        }
    }
}

pub struct Subscriber<Callback> {
    active: Rc<Cell<bool>>,
    callback: Callback,
}

pub struct SubscriberSetState<EmitterKey, Callback> {
    subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
    dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
    next_subscriber_id: usize,
}

pub struct SubscriberSet<EmitterKey, Callback>(
    Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
);

impl<EmitterKey, Callback> Clone for SubscriberSet<EmitterKey, Callback> {
    fn clone(&self) -> Self {
        SubscriberSet(self.0.clone())
    }
}

impl<'a, EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
where
    EmitterKey: Ord + Clone + Debug,
    Callback:,
{
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(SubscriberSetState {
            subscribers: Default::default(),
            dropped_subscribers: Default::default(),
            next_subscriber_id: 0,
        })))
    }

    pub fn insert(
        &'a self,
        emitter_key: EmitterKey,
        callback: Callback,
    ) -> (Subscription<'a>, impl FnOnce()) {
        let active = Rc::new(Cell::new(false));
        let mut lock = self.0.lock();
        let subscriber_id = post_inc(&mut lock.next_subscriber_id);
        lock.subscribers
            .entry(emitter_key.clone())
            .or_default()
            .get_or_insert_with(Default::default)
            .insert(
                subscriber_id,
                Subscriber {
                    active: active.clone(),
                    callback,
                },
            );

        let this = self.0.clone();

        let subscription = Subscription {
            unsubscribe: Some(Box::new(move || {
                let mut lock = this.lock();
                let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
                    return;
                };

                if let Some(subscribers) = subscribers {
                    subscribers.remove(&subscriber_id);
                    if subscribers.is_empty() {
                        lock.subscribers.remove(&emitter_key);
                    }

                    return;
                }

                lock.dropped_subscribers
                    .insert((emitter_key, subscriber_id));
            })),
        };

        (subscription, move || active.set(true))
    }

    pub fn remove(&self, emitter: &EmitterKey) -> impl IntoIterator<Item = Callback> {
        let subscribers = self.0.lock().subscribers.remove(emitter);
        subscribers
            .unwrap_or_default()
            .map(|s| s.into_values())
            .into_iter()
            .flatten()
            .filter_map(|subscriber| {
                if subscriber.active.get() {
                    Some(subscriber.callback)
                } else {
                    None
                }
            })
    }

    pub fn retain<F>(&self, emitter: &EmitterKey, mut f: F)
    where
        F: FnMut(&mut Callback) -> bool,
    {
        let Some(mut subscribers) = self
            .0
            .lock()
            .subscribers
            .get_mut(emitter)
            .and_then(|s| s.take())
        else {
            return;
        };

        subscribers.retain(|_, subscriber| {
            if subscriber.active.get() {
                f(&mut subscriber.callback)
            } else {
                true
            }
        });
        let mut lock = self.0.lock();

        if let Some(Some(new_subscribers)) = lock.subscribers.remove(emitter) {
            subscribers.extend(new_subscribers);
        }

        for (dropped_emitter, dropped_subscription_id) in mem::take(&mut lock.dropped_subscribers) {
            debug_assert_eq!(*emitter, dropped_emitter);
            subscribers.remove(&dropped_subscription_id);
        }

        if !subscribers.is_empty() {
            lock.subscribers.insert(emitter.clone(), Some(subscribers));
        }
    }
}

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}
