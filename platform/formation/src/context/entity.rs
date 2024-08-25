use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::Any,
    marker::PhantomData,
    mem,
    sync::{atomic::AtomicUsize, Arc, Weak},
};

use super::{
    context::{Effect, PlatformContext},
    model::{AnyModel, Model},
};

slotmap::new_key_type! {
    pub struct EntityId;
}

pub trait Entity<T> {
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
    ref_counts: Arc<RwLock<EntityRefCounts>>,
}

// TODO: rename EntityRefCounter
pub struct EntityRefCounts {
    pub counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
}

fn double_lease_panic<T>(operation: &str) -> ! {
    panic!(
        "cannot {operation} {} while it is already being updated",
        std::any::type_name::<T>()
    )
}

pub(crate) struct Lease<'a, T> {
    entity: Option<Box<dyn Any>>,
    pub model: &'a Model<T>,
    entity_type: PhantomData<T>,
}

impl<'a, T: 'static> core::ops::Deref for Lease<'a, T> {
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
            ref_counts: Arc::new(RwLock::new(EntityRefCounts {
                counts: SlotMap::with_key(),
                dropped_entity_ids: Vec::new(),
            })),
        }
    }

    pub fn reserve<T: 'static>(&self) -> Slot<T> {
        let id = self.ref_counts.write().counts.insert(1.into());
        Slot(Model::new(id, Arc::downgrade(&self.ref_counts)))
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
            Weak::ptr_eq(&model.entity_map, &Arc::downgrade(&self.ref_counts)),
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
        let mut ref_counts = self.ref_counts.write();
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
