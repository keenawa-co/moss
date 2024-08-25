use anyhow::Result;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use std::{
    any::TypeId,
    marker::PhantomData,
    sync::{atomic::Ordering::SeqCst, Weak},
};

use super::{
    context::{Context, Effect, EventEmitter, PlatformContext, Reservation},
    entity::{Entity, EntityId, EntityRefCounts},
    subscriber::Subscription,
    Flatten,
};

pub struct AnyModel {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: TypeId,
    pub(crate) entity_map: Weak<RwLock<EntityRefCounts>>,
}

impl AnyModel {
    fn new(id: EntityId, typ: TypeId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self {
        Self {
            entity_id: id,
            entity_type: typ,
            entity_map: entity_map.clone(),
        }
    }

    fn downgrade(&self) -> AnyWeakModel {
        AnyWeakModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_ref_counts: self.entity_map.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AnyWeakModel {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

impl AnyWeakModel {
    pub fn upgrade(&self) -> Option<AnyModel> {
        let ref_counts = &self.entity_ref_counts.upgrade()?;
        let ref_counts = ref_counts.read();
        let ref_count = ref_counts.counts.get(self.entity_id)?;

        if ref_count.load(SeqCst) == 0 {
            return None;
        }

        ref_count.fetch_add(1, SeqCst);
        drop(ref_counts);

        Some(AnyModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_ref_counts.clone(),
        })
    }
}

#[derive(Deref, DerefMut)]
pub struct WeakModel<T> {
    #[deref]
    #[deref_mut]
    any_model: AnyWeakModel,
    entity_type: PhantomData<T>,
}

unsafe impl<T> Send for WeakModel<T> {}
unsafe impl<T> Sync for WeakModel<T> {}

impl<T> Clone for WeakModel<T> {
    fn clone(&self) -> Self {
        Self {
            any_model: self.any_model.clone(),
            entity_type: self.entity_type,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Model<T> {
    #[deref]
    #[deref_mut]
    pub(crate) any_model: AnyModel,
    pub(crate) entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Model<T> {}
unsafe impl<T> Sync for Model<T> {}

impl<T: 'static> Entity<T> for Model<T> {
    type Weak = WeakModel<T>;

    fn entity_id(&self) -> EntityId {
        self.any_model.entity_id
    }

    fn downgrade(&self) -> Self::Weak {
        WeakModel {
            any_model: self.any_model.downgrade(),
            entity_type: self.entity_type,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Model {
            any_model: weak.any_model.upgrade()?,
            entity_type: weak.entity_type,
        })
    }
}

impl<T: 'static> WeakModel<T> {
    pub fn upgrade(&self) -> Option<Model<T>> {
        Model::upgrade_from(self)
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Result<R>
    where
        C: Context,
        Result<C::Result<R>>: Flatten<R>,
    {
        Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| ctx.update_model(&this, update)),
        )
    }
}

impl<T: 'static> Model<T> {
    pub(crate) fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self
    where
        T: 'static,
    {
        Self {
            any_model: AnyModel::new(id, TypeId::of::<T>(), entity_map),
            entity_type: PhantomData,
        }
    }

    pub fn read<'a>(&self, ctx: &'a PlatformContext) -> &'a T {
        ctx.entities.read(self)
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> C::Result<R>
    where
        C: Context,
    {
        ctx.update_model(self, update)
    }
}

#[derive(Deref, DerefMut)]
pub struct ModelContext<'a, T> {
    #[deref]
    #[deref_mut]
    app: &'a mut PlatformContext,
    model_state: WeakModel<T>,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn new(app: &'a mut PlatformContext, model_state: WeakModel<T>) -> Self {
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
        E: Entity<T2>,
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
        E: Entity<W>,
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

impl<'a, T> Context for ModelContext<'a, T> {
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
