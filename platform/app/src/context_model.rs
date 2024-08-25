use derive_more::{Deref, DerefMut};

use slotmap::{KeyData, SlotMap};
use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;

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

#[derive(Deref, DerefMut)]
pub(crate) struct Slot<T>(Model<T>);

struct EntityRefCounter {
    storage: SlotMap<EntityId, AtomicUsize>,
    dropped: Vec<EntityId>,
}

pub struct EntryMap(SlotMap<EntityId, Box<dyn Any>>);

impl EntryMap {
    pub fn new() -> Self {
        Self(SlotMap::with_key())
    }

    // pub(crate) fn insert::<T>(&mut self) ->
}

// pub struct EntryMap {
//     storage: SecondaryMap<EntityId, Box<dyn Any>>,
//     counter: Arc<RwLock<EntityRefCounter>>,
// }

// impl EntryMap {
//     pub(crate) fn new() -> Self {
//         let counter = EntityRefCounter {
//             storage: SlotMap::with_key(),
//             dropped: Vec::new(),
//         };

//         Self {
//             storage: SecondaryMap::new(),
//             counter: Arc::new(RwLock::new(counter)),
//         }
//     }

//     pub(crate) fn reserve<T: 'static>(&self) -> Slot<T> {
//         let id = self.counter.write().storage.insert(1.into());

//         Slot(Model::new(id))
//     }

//     pub(crate) fn insert<T: 'static>(&mut self, slot: Slot<T>, entity: T) -> Model<T> {
//         let model = slot.0;
//         self.storage.insert(model.id, Box::new(entity));
//         model
//     }

//     pub(crate) fn read<T: 'static>(&self, model: &Model<T>) -> &T {
//         self.storage[model.id]
//             .downcast_ref()
//             .unwrap_or_else(|| panic!("failed to read entity model"))
//     }
// }

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

pub trait Entity<T: 'static> {
    fn id(&self) -> EntityId;
}

impl<T: 'static> Entity<T> for Model<T> {
    fn id(&self) -> EntityId {
        self.id
    }
}

impl<T: 'static> Model<T> {
    pub fn new(id: EntityId) -> Self {
        Self {
            any: AnyModel {
                id,
                typ: TypeId::of::<T>(),
            },
            _type: PhantomData,
        }
    }
}

// pub struct ModelContext {}

// impl ModelContext {
//     pub(crate) fn new() -> Self {
//         Self {}
//     }
// }
