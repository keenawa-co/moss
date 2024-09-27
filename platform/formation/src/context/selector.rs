use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::TypeId,
    marker::PhantomData,
    sync::{Arc, Weak},
};

use super::{
    node::{
        AnyNode, AnyNodeValue, Lease, NodeKey, NodeRefCounter, NodeValue, ProtoNode, Slot, WeakNode,
    },
    selector_context::SelectorContext,
    AnyContext, Context,
};

#[derive(Deref, DerefMut)]
pub struct Selector<T: NodeValue> {
    #[deref]
    #[deref_mut]
    node: ProtoNode,
    typ: PhantomData<T>,
    compute: Box<dyn Fn(&mut SelectorContext<'_, T>) -> T + 'static>,
}

impl<T: NodeValue> AnyNode<T> for Selector<T> {
    type Weak = WeakNode<T, Selector<T>>;

    fn key(&self) -> NodeKey {
        self.node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakNode {
            weak_proto_atom: self.node.downgrade(),
            typ: self.typ,
            node_typ: PhantomData::<Selector<T>>,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<T: NodeValue> Selector<T> {
    pub(super) fn new(
        key: NodeKey,
        compute: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
        rc: Weak<RwLock<NodeRefCounter>>,
    ) -> Self {
        Self {
            node: ProtoNode::new(key, TypeId::of::<T>(), rc),
            compute: Box::new(compute),
            typ: PhantomData,
        }
    }

    pub fn read<'a, C>(&self, ctx: &'a mut C) -> &'a T
    where
        C: AnyContext + AsMut<Context>,
    {
        ctx.read_selector(self)
    }

    pub fn compute(&self, ctx: &mut SelectorContext<'_, T>) -> T {
        (&self.compute)(ctx)
    }
}

#[derive(Clone)]
pub(super) struct SelectorMap {
    pub(super) computed_values: im::HashMap<NodeKey, Box<dyn AnyNodeValue>>,
    pub(super) rc: Arc<RwLock<NodeRefCounter>>,
}

impl SelectorMap {
    pub fn new() -> Self {
        Self {
            computed_values: im::HashMap::new(),
            rc: Arc::new(RwLock::new(NodeRefCounter {
                counts: SlotMap::with_key(),
                dropped: Vec::new(),
            })),
        }
    }

    pub(super) fn lookup(&self, key: &NodeKey) -> bool {
        self.computed_values.contains_key(key)
    }

    pub(super) fn remove(&mut self, key: &NodeKey) {
        self.computed_values
            .remove(key)
            // Panic at this point most likely signals a bug in the program.
            // The reason why the key may not be in the map:
            // - The value has already been deleted
            // - The value is currently leased and is being updated
            .unwrap_or_else(|| panic!("cannot delete a node value that does not exist"));
    }

    pub(super) fn reserve<T, N>(&self, create_slot: impl FnOnce(&Self, NodeKey) -> N) -> Slot<T, N>
    where
        T: NodeValue,
        N: AnyNode<T>,
    {
        let key = self.rc.write().counts.insert(1.into());
        Slot(create_slot(self, key), PhantomData)
    }

    pub(super) fn insert<T>(&mut self, key: NodeKey, value: T)
    where
        T: NodeValue,
    {
        self.computed_values = self.computed_values.update(key, Box::new(value));
    }

    pub(super) fn read<T>(&self, key: &NodeKey) -> &T
    where
        T: NodeValue,
    {
        // TODO: add check for valid context

        self.computed_values[key]
            .as_any_ref()
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            })
    }

    pub(super) fn begin_lease<'a, T>(&mut self, node: &'a Selector<T>) -> Lease<'a, T, Selector<T>>
    where
        T: NodeValue,
    {
        // TODO: add check for valid context

        let value = Some(self.computed_values.remove(&node.key).unwrap_or_else(|| {
            panic!(
                "cannot update {} node that is already being updated",
                std::any::type_name::<T>()
            )
        }));

        Lease {
            node,
            value,
            typ: PhantomData,
        }
    }

    pub(super) fn end_lease<T>(&mut self, mut lease: Lease<T, Selector<T>>)
    where
        T: NodeValue,
    {
        self.computed_values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}
