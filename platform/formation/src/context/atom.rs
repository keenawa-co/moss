use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_std::collection::ImHashMap;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::TypeId,
    marker::PhantomData,
    sync::{Arc, Weak},
};

use crate::FlattenAnyhowResult;

use super::{
    atom_context::AtomContext,
    node::{AnyNode, AnyNodeValue, Lease, NodeKey, NodeRefCounter, NodeValue, ProtoNode, WeakNode},
};
use super::{node::Slot, AnyContext};

pub trait AnyAtom<T>: AnyNode<T> {}

impl<T: NodeValue> WeakNode<T, Atom<T>> {
    pub fn upgrade(&self) -> Option<Atom<T>> {
        Atom::upgrade_from(self)
    }

    pub fn update<'a, C, R>(
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
    node: ProtoNode,
    typ: PhantomData<T>,
}

impl<T: NodeValue> AnyAtom<T> for Atom<T> {}

impl<T: NodeValue> AnyNode<T> for Atom<T> {
    type Weak = WeakNode<T, Atom<T>>;

    fn key(&self) -> NodeKey {
        self.node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakNode {
            weak_proto_atom: self.node.downgrade(),
            typ: self.typ,
            node_typ: PhantomData::<Atom<T>>,
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
            node: ProtoNode::new(key, TypeId::of::<T>(), rc),
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

    pub fn begin_lease<'a, T>(&mut self, node: &'a Atom<T>) -> Lease<'a, T, Atom<T>>
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

        Lease {
            value,
            node,
            typ: PhantomData,
        }
    }

    pub fn end_lease<T>(&mut self, mut lease: Lease<T, Atom<T>>)
    where
        T: NodeValue,
    {
        self.values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}
