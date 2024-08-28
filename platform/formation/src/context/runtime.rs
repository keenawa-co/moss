use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
    sync::Arc,
};

use moss_std::collection::FxHashSet;
use parking_lot::Mutex;

use super::{
    context::{Context, ContextCell},
    entity::EntityMap,
    subscriber::SubscriberSet,
};

pub struct PlatformRuntime(Arc<Mutex<Context>>);

impl PlatformRuntime {
    pub fn new() -> Self {
        // Self(Arc::new_cyclic(|this| {
        //     Mutex::new(Context {
        //         this: this.clone(),
        //         observers: SubscriberSet::new(),
        //         pending_notifications: FxHashSet::default(),
        //         pending_effects: VecDeque::new(),
        //         pending_updates: 0,
        //         entities: EntityMap::new(),
        //         flushing_effects: false,
        //         event_listeners: SubscriberSet::new(),
        //         release_listeners: SubscriberSet::new(),
        //     })
        // }))

        Self(Arc::new_cyclic(|this| {
            // let r: Mutex<std::sync::Weak<Context>> = this.clone().into();

            Mutex::new(Context {
                // this: todo!(),
                observers: SubscriberSet::new(),
                pending_notifications: FxHashSet::default(),
                pending_effects: VecDeque::new(),
                pending_updates: 0,
                entities: EntityMap::new(),
                flushing_effects: false,
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
            })
        }))
    }

    // pub fn exec<F>(self, f: F)
    // where
    //     F: 'static + FnOnce(&mut Context),
    // {
    //     let this = self.0.clone();
    //     f(&mut *this.borrow_mut());
    // }

    pub fn exec<F>(self, f: F)
    where
        F: 'static + FnOnce(Arc<Mutex<Context>>),
    {
        let this = self.0.clone();
        f(this);
    }
}
