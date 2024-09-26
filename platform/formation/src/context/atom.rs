use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_std::collection::ImHashMap;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::TypeId,
    marker::PhantomData,
    sync::{atomic::Ordering, Arc, Weak},
};

use crate::FlattenAnyhowResult;

use super::{
    atom_context::AtomContext,
    node::{AnyNode, AnyNodeValue, NodeKey, NodeRefCounter, NodeValue},
};
use super::{node::Slot, AnyContext};

pub trait AnyAtom<T>: AnyNode<T> {}

// AnyModel
pub struct ProtoAtom {
    pub(super) key: NodeKey,                     // TODO: remove pub(super)
    pub(super) typ: TypeId,                      // TODO: remove pub(super)
    pub(super) rc: Weak<RwLock<NodeRefCounter>>, // TODO: remove pub(super)
}

impl ProtoAtom {
    // TODO: remove pub(super)
    pub(super) fn new(key: NodeKey, typ: TypeId, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            key,
            typ,
            rc: rc.clone(),
        }
    }

    // TODO: remove pub(super)
    pub(super) fn downgrade(&self) -> WeakProtoAtom {
        WeakProtoAtom {
            key: self.key,
            typ: self.typ,
            rc: self.rc.clone(),
        }
    }
}

#[derive(Clone)]
// AnyWeakModel
pub struct WeakProtoAtom {
    pub(super) key: NodeKey,
    pub(super) typ: TypeId,
    pub(super) rc: Weak<RwLock<NodeRefCounter>>,
}

impl WeakProtoAtom {
    fn upgrade(&self) -> Option<ProtoAtom> {
        let ref_counts = &self.rc.upgrade()?;
        let ref_counts_lock = ref_counts.read();
        let ref_count = ref_counts_lock.counts.get(self.key)?;

        if ref_count.load(Ordering::SeqCst) == 0 {
            return None;
        }

        ref_count.fetch_add(1, Ordering::SeqCst);
        drop(ref_counts_lock);

        Some(ProtoAtom {
            key: self.key,
            typ: self.typ,
            rc: self.rc.clone(),
        })
    }
}

#[derive(Deref, DerefMut)]
pub struct WeakAtom<T> {
    #[deref]
    #[deref_mut]
    pub(super) weak_proto_atom: WeakProtoAtom,
    pub(super) typ: PhantomData<T>,
}

unsafe impl<T> Send for WeakAtom<T> {}
unsafe impl<T> Sync for WeakAtom<T> {}

impl<T> Clone for WeakAtom<T> {
    fn clone(&self) -> Self {
        Self {
            weak_proto_atom: self.weak_proto_atom.clone(),
            typ: self.typ,
        }
    }
}

impl<T: NodeValue> WeakAtom<T> {
    pub fn upgrade(&self) -> Option<Atom<T>> {
        Atom::upgrade_from(self)
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Result<R>
    where
        C: AnyContext,
        Result<C::Result<R>>: FlattenAnyhowResult<R>,
    {
        FlattenAnyhowResult::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("node release"))
                .map(|this| ctx.update_atom(&this, update)),
        )
    }
}

pub struct OnChangeAtomEvent {}

#[derive(Deref, DerefMut)]
pub struct Atom<T> {
    #[deref]
    #[deref_mut]
    node: ProtoAtom,
    typ: PhantomData<T>,
}

impl<T: NodeValue> AnyAtom<T> for Atom<T> {}

impl<T: NodeValue> AnyNode<T> for Atom<T> {
    type Weak = WeakAtom<T>;

    fn key(&self) -> NodeKey {
        self.node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakAtom {
            weak_proto_atom: self.node.downgrade(),
            typ: self.typ,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Atom {
            node: weak.weak_proto_atom.upgrade()?,
            typ: weak.typ,
        })
    }
}

impl<T: NodeValue> Atom<T> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            node: ProtoAtom::new(key, TypeId::of::<T>(), rc),
            typ: PhantomData,
        }
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> C::Result<R>
    where
        C: AnyContext,
    {
        ctx.update_atom(self, update)
    }

    pub fn read<'a, C: AnyContext>(&self, ctx: &'a mut C) -> &'a T {
        ctx.read_atom(self)
    }
}

type AtomLease<'a, T> = super::node::Lease<'a, T, Atom<T>>;

#[derive(Clone)]
pub(super) struct AtomMap {
    pub values: ImHashMap<NodeKey, Box<dyn AnyNodeValue>>,
    pub rc: Arc<RwLock<NodeRefCounter>>,
}

impl AtomMap {
    pub fn new() -> Self {
        Self {
            values: im::HashMap::new(),
            rc: Arc::new(RwLock::new(NodeRefCounter {
                counts: SlotMap::with_key(),
                dropped: Vec::new(),
            })),
        }
    }

    pub fn reserve<T, N>(&self, create_slot: impl FnOnce(&Self, NodeKey) -> N) -> Slot<T, N>
    where
        T: NodeValue,
        N: AnyNode<T>,
    {
        let key = self.rc.write().counts.insert(1.into());
        Slot(create_slot(self, key), PhantomData)
    }

    pub fn insert<T, N>(&mut self, slot: Slot<T, N>, entity: T) -> N
    where
        T: NodeValue,
        N: AnyNode<T>,
    {
        let atom = slot.0;
        self.values = self.values.update(atom.key(), Box::new(entity));

        atom
    }

    pub fn read<T>(&self, key: &NodeKey) -> &T
    where
        T: NodeValue,
    {
        // TODO: add check for valid context

        self.values[key]
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            })
    }

    pub fn begin_lease<'a, T>(&mut self, node: &'a Atom<T>) -> AtomLease<'a, T>
    where
        T: NodeValue,
    {
        // TODO: add check for valid context

        let value = Some(self.values.remove(&node.key()).unwrap_or_else(|| {
            panic!(
                "cannot update {} node that is already being updated",
                std::any::type_name::<T>()
            )
        }));

        AtomLease {
            value,
            node,
            typ: PhantomData,
        }
    }

    pub fn end_lease<T>(&mut self, mut lease: AtomLease<T>)
    where
        T: NodeValue,
    {
        self.values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}
