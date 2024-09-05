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

slotmap::new_key_type! {
    pub struct EntityId;
}

pub struct EntityRefCounter {
    pub counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
}

pub struct EntityMap {
    entities: SecondaryMap<EntityId, Box<dyn Any>>,
    ref_counter: Arc<RwLock<EntityRefCounter>>,
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
}
