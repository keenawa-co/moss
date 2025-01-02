use parking_lot::Mutex;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::mem;
use std::ops::AddAssign;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

pub struct Subscriber<Callback> {
    active: Arc<AtomicBool>,
    callback: Callback,
}

impl<Callback: 'static> Subscriber<Callback> {
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}

struct SubscriberSetState<EmitterKey, Callback> {
    subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
    dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
    next_subscriber_id: usize,
}

pub struct Subscription {
    unsubscribe: Option<Box<dyn FnOnce() + Send + Sync + 'static>>,
}

impl Subscription {
    pub fn new(unsubscribe: impl FnOnce() + Send + Sync + 'static) -> Self {
        Self {
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }

    pub fn detach(mut self) {
        self.unsubscribe.take();
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.take() {
            unsubscribe();
        }
    }
}

pub(crate) struct SubscriberSet<EmitterKey, Callback> {
    state: Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
}

impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
where
    EmitterKey: 'static + Send + Sync + Ord + Clone + Debug,
    Callback: 'static + Send + Sync,
{
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SubscriberSetState {
                subscribers: Default::default(),
                dropped_subscribers: Default::default(),
                next_subscriber_id: 0,
            })),
        }
    }

    pub(crate) fn insert(
        &self,
        emitter_key: EmitterKey,
        callback: Callback,
    ) -> (Subscription, impl FnOnce()) {
        let mut state_lock = self.state.lock();
        let active = Arc::new(AtomicBool::new(false));
        let subscriber_id = {
            let current = state_lock.next_subscriber_id;
            state_lock.next_subscriber_id += 1;
            current
        };

        state_lock
            .subscribers
            .entry(emitter_key.clone())
            .or_default()
            .get_or_insert_with(Default::default)
            .insert(
                subscriber_id,
                Subscriber {
                    active: Arc::clone(&active),
                    callback,
                },
            );

        let state_clone = Arc::clone(&self.state);
        let subscription = Subscription {
            unsubscribe: Some(Box::new(move || {
                let mut state_lock = state_clone.lock();
                let Some(subscribers) = state_lock.subscribers.get_mut(&emitter_key) else {
                    // The emitter key has already been removed, nothing to do.
                    return;
                };

                if let Some(subscribers) = subscribers {
                    subscribers.remove(&subscriber_id);

                    // If there are no more subscribers for this emitter key,
                    // remove the key itself.
                    if subscribers.is_empty() {
                        state_lock.subscribers.remove(&emitter_key);
                    }

                    return;
                }

                // If we couldn't remove the subscription, it was likely
                // dropped while invoking the callback. Mark it as dropped
                // so it can be cleaned up later.
                state_lock
                    .dropped_subscribers
                    .insert((emitter_key, subscriber_id));
            })),
        };

        (subscription, move || active.store(true, Ordering::SeqCst))
    }

    pub(crate) fn remove(&self, ref emitter_key: EmitterKey) -> impl IntoIterator<Item = Callback> {
        let subscribers = self.state.lock().subscribers.remove(emitter_key);
        subscribers
            .unwrap_or_default()
            .map(|subscribers| subscribers.into_values())
            .into_iter()
            .flatten()
            .filter_map(|subscriber| {
                if subscriber.is_active() {
                    Some(subscriber.callback)
                } else {
                    None
                }
            })
    }

    /// Iterates over each subscriber for the given emitter and applies
    /// the provided callback function. If the callback returns `false`,
    /// the subscriber is removed.
    pub(crate) fn retain<F>(&self, emitter_key: &EmitterKey, mut f: F)
    where
        F: Fn(&Callback) -> bool,
    {
        let Some(mut subscribers) = self
            .state
            .lock()
            .subscribers
            .get_mut(emitter_key)
            .and_then(|data| data.take())
        else {
            return;
        };

        // Retain only those subscribers for which the callback returns
        // true or are inactive.
        subscribers.retain(|_, subscriber| {
            if subscriber.is_active() {
                f(&subscriber.callback)
            } else {
                true
            }
        });

        let mut state_lock = self.state.lock();

        // Merge any new subscribers that were added while the callback
        // was being executed.
        if let Some(Some(new_subscribers)) = state_lock.subscribers.remove(emitter_key) {
            subscribers.extend(new_subscribers);
        }

        // Remove any subscriptions that were marked as dropped during
        // the callback execution.
        for (dropped_emitter, dropped_subscription_id) in
            mem::take(&mut state_lock.dropped_subscribers)
        {
            debug_assert_eq!(*emitter_key, dropped_emitter);
            subscribers.remove(&dropped_subscription_id);
        }

        if !subscribers.is_empty() {
            state_lock
                .subscribers
                .insert(emitter_key.clone(), Some(subscribers));
        }
    }
}
