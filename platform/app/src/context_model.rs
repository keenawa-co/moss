use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::{KeyData, SecondaryMap, SlotMap};
use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

slotmap::new_key_type! {
    pub struct EntityId;
}

impl From<u64> for EntityId {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

impl EntityId {
    /// Convert entity id to a [u64]
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

struct EntityRefCounter {
    counter: SlotMap<EntityId, AtomicUsize>,
    dropped: Vec<EntityId>,
}

pub struct EntryMap {
    storage: SecondaryMap<EntityId, Box<dyn Any>>,
    counter: Arc<RwLock<EntityRefCounter>>,
}

impl EntryMap {
    pub fn new() -> Self {
        let counter = EntityRefCounter {
            counter: SlotMap::with_key(),
            dropped: Vec::new(),
        };

        Self {
            storage: SecondaryMap::new(),
            counter: Arc::new(RwLock::new(counter)),
        }
    }
}

#[derive(Clone)]
pub struct AnyModel {
    pub(crate) id: EntityId,
    pub(crate) typ: TypeId,
}

impl Hash for AnyModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for AnyModel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AnyModel {}

impl AnyModel {
    fn new(id: EntityId, typ: TypeId) -> Self {
        Self { id, typ }
    }

    pub fn entity_id(&self) -> EntityId {
        self.id
    }

    pub fn entity_type(&self) -> TypeId {
        self.typ
    }
}

#[derive(Deref, DerefMut)]
pub struct Model<T> {
    #[deref]
    #[deref_mut]
    pub(crate) any: AnyModel,
    pub(crate) _type: PhantomData<T>,
}

impl<T> Hash for Model<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any.hash(state);
    }
}

impl<T> PartialEq for Model<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any == other.any
    }
}

unsafe impl<T> Send for Model<T> {}
unsafe impl<T> Sync for Model<T> {}

pub trait Entity<T> {
    fn id(&self) -> EntityId;
}

impl<T> Entity<T> for Model<T> {
    fn id(&self) -> EntityId {
        self.id
    }
}
