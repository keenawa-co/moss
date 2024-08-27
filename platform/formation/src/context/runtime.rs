use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
};

use moss_std::collection::FxHashSet;

use super::{context::PlatformContext, entity::EntityMap, subscriber::SubscriberSet};

pub struct PlatformRuntime(Rc<RefCell<PlatformContext>>);

impl PlatformRuntime {
    pub fn new() -> Self {
        Self(Rc::new_cyclic(|this| {
            RefCell::new(PlatformContext {
                this: this.clone(),
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

    pub fn exec<F>(self, f: F)
    where
        F: 'static + FnOnce(&mut PlatformContext),
    {
        let this = self.0.clone();
        let mut ctx: RefMut<PlatformContext> = this.borrow_mut();
        f(&mut ctx);
    }
}
