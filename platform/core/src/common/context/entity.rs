use anyhow::Result;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

use super::{model_context::ModelContext, utl::FlattenAnyhowResult, AnyContext, Context};

pub struct AnyModel {
    entity_id: EntityId,
    entity_type: TypeId,
    entity_map: Weak<RwLock<EntityRefCounter>>,
}

impl AnyModel {
    fn new(id: EntityId, typ: TypeId, entity_map: Weak<RwLock<EntityRefCounter>>) -> Self {
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
    pub(super) entity_id: EntityId,
    pub(super) entity_type: TypeId,
    pub(super) entity_ref_counts: Weak<RwLock<EntityRefCounter>>,
}

impl AnyWeakModel {
    pub fn upgrade(&self) -> Option<AnyModel> {
        let ref_counts = &self.entity_ref_counts.upgrade()?;
        let ref_counts = ref_counts.read();
        let ref_count = ref_counts.counts.get(self.entity_id)?;

        if ref_count.load(Ordering::SeqCst) == 0 {
            return None;
        }

        ref_count.fetch_add(1, Ordering::SeqCst);
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
    pub(super) any_model: AnyModel,
    pub(super) entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Model<T> {}
unsafe impl<T> Sync for Model<T> {}

impl<T: 'static> AnyEntity<T> for Model<T> {
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
        C: AnyContext,
        Result<C::Result<R>>: FlattenAnyhowResult<R>,
    {
        FlattenAnyhowResult::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| ctx.update_model(&this, update)),
        )
    }
}

impl<T: 'static> Model<T> {
    pub(crate) fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounter>>) -> Self
    where
        T: 'static,
    {
        Self {
            any_model: AnyModel::new(id, TypeId::of::<T>(), entity_map),
            entity_type: PhantomData,
        }
    }

    pub fn read<'a>(&self, ctx: &'a Context) -> &'a T {
        ctx.entities.read(self)
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> C::Result<R>
    where
        C: AnyContext,
    {
        ctx.update_model(self, update)
    }
}

slotmap::new_key_type! {
    pub struct EntityId;
}

pub trait AnyEntity<T> {
    type Weak: 'static;

    fn entity_id(&self) -> EntityId;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Deref, DerefMut)]
pub struct Slot<T>(Model<T>);

pub struct EntityMap {
    entities: SecondaryMap<EntityId, Box<dyn Any>>,
    ref_counter: Arc<RwLock<EntityRefCounter>>,
}

pub struct EntityRefCounter {
    pub counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
}

fn double_lease_panic<T>(operation: &str) -> ! {
    panic!(
        "cannot {operation} {} while it is already being updated",
        std::any::type_name::<T>()
    )
}

pub struct Lease<'a, T> {
    entity: Option<Box<dyn Any>>,
    pub model: &'a Model<T>,
    entity_type: PhantomData<T>,
}

impl<'a, T> core::ops::Deref for Lease<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.entity.as_ref().unwrap().downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> core::ops::DerefMut for Lease<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity.as_mut().unwrap().downcast_mut().unwrap()
    }
}

impl<'a, T> Drop for Lease<'a, T> {
    fn drop(&mut self) {
        if self.entity.is_some() && !std::thread::panicking() {
            panic!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            entities: SecondaryMap::new(),
            ref_counter: Arc::new(RwLock::new(EntityRefCounter {
                counts: SlotMap::with_key(),
                dropped_entity_ids: Vec::new(),
            })),
        }
    }

    pub fn reserve<T: 'static>(&self) -> Slot<T> {
        let id = self.ref_counter.write().counts.insert(1.into());
        Slot(Model::new(id, Arc::downgrade(&self.ref_counter)))
    }

    pub fn insert<T>(&mut self, slot: Slot<T>, entity: T) -> Model<T>
    where
        T: 'static,
    {
        let model = slot.0;
        self.entities.insert(model.entity_id, Box::new(entity));

        model
    }

    fn assert_valid_context(&self, model: &AnyModel) {
        debug_assert!(
            Weak::ptr_eq(&model.entity_map, &Arc::downgrade(&self.ref_counter)),
            "used a model with the wrong context"
        );
    }

    pub fn lease<'a, T>(&mut self, model: &'a Model<T>) -> Lease<'a, T> {
        self.assert_valid_context(model);

        let entity = Some(
            self.entities
                .remove(model.entity_id)
                .unwrap_or_else(|| double_lease_panic::<T>("update")),
        );

        Lease {
            entity,
            model,
            entity_type: PhantomData,
        }
    }

    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.model.entity_id, lease.entity.take().unwrap());
    }

    pub fn take_dropped(&mut self) -> Vec<(EntityId, Box<dyn Any>)> {
        let mut ref_counts = self.ref_counter.write();
        let dropped_entity_ids = mem::take(&mut ref_counts.dropped_entity_ids);

        dropped_entity_ids
            .into_iter()
            .filter_map(|entity_id| {
                let count = ref_counts.counts.remove(entity_id).unwrap();
                debug_assert_eq!(
                    count.load(std::sync::atomic::Ordering::SeqCst),
                    0,
                    "dropped an entity that was referenced"
                );

                Some((entity_id, self.entities.remove(entity_id)?))
            })
            .collect()
    }

    pub fn read<T: 'static>(&self, model: &Model<T>) -> &T {
        self.assert_valid_context(model);

        self.entities[model.entity_id]
            .downcast_ref()
            .unwrap_or_else(|| double_lease_panic::<T>("read"))
    }
}
