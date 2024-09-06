use derive_more::{Deref, DerefMut};
use std::any::TypeId;

use super::{
    entity::{AnyEntity, Model, WeakModel},
    subscription::Subscription,
    AnyContext, ContextInner, Effect, EventEmitter, Reservation,
};

#[derive(Deref, DerefMut)]
pub struct ModelContext<'a, T> {
    #[deref]
    #[deref_mut]
    app: &'a mut ContextInner,
    model_state: WeakModel<T>,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn new(app: &'a mut ContextInner, model_state: WeakModel<T>) -> Self {
        Self { app, model_state }
    }

    pub fn weak_model(&self) -> WeakModel<T> {
        self.model_state.clone()
    }

    pub fn emit<Evt>(&mut self, event: Evt)
    where
        T: EventEmitter<Evt>,
        Evt: 'static,
    {
        self.app.pending_effects.push_back(Effect::Emit {
            emitter: self.model_state.entity_id,
            event_type: TypeId::of::<Evt>(),
            event: Box::new(event),
        });
    }

    pub fn subscribe<T2, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(&mut T, E, &Evt, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        T2: 'static + EventEmitter<Evt>,
        E: AnyEntity<T2>,
        Evt: 'static,
    {
        let this = self.weak_model();
        self.app.subscribe_internal(entity, move |e, event, cx| {
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| on_event(this, e, event, cx));
                true
            } else {
                false
            }
        })
    }

    // TODO: rename -> on_notify
    pub fn observe<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(&mut T, E, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        W: 'static,
        E: AnyEntity<W>,
    {
        let this = self.weak_model();
        self.app.observe_internal(entity, move |e, ctx| {
            if let Some(this) = this.upgrade() {
                this.update(ctx, |this, ctx| on_notify(this, e, ctx));
                true
            } else {
                false
            }
        })
    }

    pub fn notify(&mut self) {
        if self
            .app
            .pending_notifications
            .insert(self.model_state.entity_id)
        {
            self.app.pending_effects.push_back(Effect::Notify {
                emitter: self.model_state.entity_id,
            })
        }
    }
}

impl<'a, T> AnyContext for ModelContext<'a, T> {
    type Result<U> = U;

    fn reserve_model<U: 'static>(&mut self) -> Reservation<U> {
        self.app.reserve_model()
    }

    fn new_model<U: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, U>) -> U,
    ) -> Model<U> {
        self.app.new_model(build_model)
    }

    fn insert_model<U: 'static>(
        &mut self,
        reservation: Reservation<U>,
        build_model: impl FnOnce(&mut ModelContext<'_, U>) -> U,
    ) -> Model<U> {
        self.app.insert_model(reservation, build_model)
    }

    fn update_model<U: 'static, R>(
        &mut self,
        handle: &Model<U>,
        update: impl FnOnce(&mut U, &mut ModelContext<'_, U>) -> R,
    ) -> R {
        self.app.update_model(handle, update)
    }
}
