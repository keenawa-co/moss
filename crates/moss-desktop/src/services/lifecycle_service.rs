// use once_cell::sync::OnceCell;
// use parking_lot::Mutex;
// use std::collections::{BTreeMap, BTreeSet};
// use std::fmt::Debug;
// use std::mem;
// use std::ops::AddAssign;
// use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
// use std::sync::Arc;
// use tauri::AppHandle;

// pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
//     let prev = *value;
//     *value += T::from(1);
//     prev
// }

// pub struct Subscriber<Callback> {
//     active: Arc<AtomicBool>,
//     callback: Callback,
// }

// pub struct SubscriberSet<EmitterKey, Callback>(
//     Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
// );

// impl<EmitterKey, Callback> Clone for SubscriberSet<EmitterKey, Callback> {
//     fn clone(&self) -> Self {
//         SubscriberSet(self.0.clone())
//     }
// }

// impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
// where
//     EmitterKey: 'static + Ord + Clone + Debug,
//     Callback: 'static,
// {
//     pub fn new() -> Self {
//         Self(Arc::new(Mutex::new(SubscriberSetState {
//             subscribers: Default::default(),
//             dropped_subscribers: Default::default(),
//             next_subscriber_id: 0,
//         })))
//     }

//     pub fn insert(&self, emitter_key: EmitterKey, callback: Callback) -> (Subscription) {
//         let active = Arc::new(AtomicBool::new(false));
//         let mut lock = self.0.lock();
//         let subscriber_id = post_inc(&mut lock.next_subscriber_id);
//         lock.subscribers
//             .entry(emitter_key.clone())
//             .or_default()
//             .get_or_insert_with(Default::default)
//             .insert(
//                 subscriber_id,
//                 Subscriber {
//                     active: active.clone(),
//                     callback,
//                 },
//             );

//         let this = self.0.clone();

//         let subscription = Subscription {
//             unsubscribe: Some(Box::new(move || {
//                 let mut lock = this.lock();
//                 let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
//                     return;
//                 };

//                 if let Some(subscribers) = subscribers {
//                     subscribers.remove(&subscriber_id);
//                     if subscribers.is_empty() {
//                         lock.subscribers.remove(&emitter_key);
//                     }

//                     return;
//                 }

//                 lock.dropped_subscribers
//                     .insert((emitter_key, subscriber_id));
//             })),
//         };

//         subscription
//     }

//     pub fn remove(&self, emitter: &EmitterKey) -> impl IntoIterator<Item = Callback> {
//         let subscribers = self.0.lock().subscribers.remove(emitter);
//         subscribers
//             .unwrap_or_default()
//             .map(|s| s.into_values())
//             .into_iter()
//             .flatten()
//             .filter_map(|subscriber| {
//                 if subscriber.active.load(Ordering::SeqCst) {
//                     Some(subscriber.callback)
//                 } else {
//                     None
//                 }
//             })
//     }

//     pub fn retain<F>(&self, emitter: &EmitterKey, mut f: F)
//     where
//         F: FnMut(&mut Callback) -> bool,
//     {
//         let Some(mut subscribers) = self
//             .0
//             .lock()
//             .subscribers
//             .get_mut(emitter)
//             .and_then(|s| s.take())
//         else {
//             return;
//         };

//         subscribers.retain(|_, subscriber| {
//             if subscriber.active.load(Ordering::SeqCst) {
//                 f(&mut subscriber.callback)
//             } else {
//                 true
//             }
//         });
//         let mut lock = self.0.lock();

//         if let Some(Some(new_subscribers)) = lock.subscribers.remove(emitter) {
//             subscribers.extend(new_subscribers);
//         }

//         for (dropped_emitter, dropped_subscription_id) in mem::take(&mut lock.dropped_subscribers) {
//             debug_assert_eq!(*emitter, dropped_emitter);
//             subscribers.remove(&dropped_subscription_id);
//         }

//         if !subscribers.is_empty() {
//             lock.subscribers.insert(emitter.clone(), Some(subscribers));
//         }
//     }
// }

// pub struct SubscriberSetState<EmitterKey, Callback> {
//     subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
//     dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
//     next_subscriber_id: usize,
// }

// pub struct Subscription {
//     unsubscribe: Option<Box<dyn FnOnce() + 'static>>,
// }

// impl Subscription {
//     pub fn new(unsubscribe: impl 'static + FnOnce()) -> Self {
//         Self {
//             unsubscribe: Some(Box::new(unsubscribe)),
//         }
//     }

//     pub fn detach(mut self) {
//         self.unsubscribe.take();
//     }
// }

// impl Drop for Subscription {
//     fn drop(&mut self) {
//         if let Some(unsubscribe) = self.unsubscribe.take() {
//             unsubscribe();
//         }
//     }
// }

// #[repr(i8)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum LifecyclePhase {
//     Starting = 0,
// }

// pub struct LifecycleService {
//     phase: OnceCell<AtomicI8>,
//     listeners: SubscriberSet<LifecyclePhase, Listener>,
// }

// type Listener = Box<dyn FnOnce(&AppHandle) + Send + 'static>;

// impl LifecycleService {
//     pub fn new() -> Self {
//         Self {
//             phase: OnceCell::new(),
//             listeners: SubscriberSet::new(),
//         }
//     }

//     pub fn register_phase_listener(
//         &self,
//         phase: LifecyclePhase,
//         callback: impl FnOnce(&AppHandle) + Send + 'static,
//     ) -> Subscription {
//         self.listeners.insert(phase, Box::new(callback))
//     }

//     pub fn notify_phase(&self, phase: LifecyclePhase, app_handle: AppHandle) {
//         let mut lock = self.listeners.0.lock();

//         let Some(subs_map) = lock.subscribers.get_mut(&phase).map(|m| mem::take(m)) else {
//             return;
//         };

//         dbg!(0);

//         if let Some(subs) = subs_map {
//             dbg!(1);
//             for (_, subscriber) in subs {
//                 dbg!(2);
//                 (subscriber.callback)(&app_handle);
//             }
//         }

//         if let Some(value) = self.phase.get() {
//             value.store(phase as i8, Ordering::SeqCst);
//         } else {
//             let _ = self.phase.set(AtomicI8::from(phase as i8));
//         }
//     }
// }
