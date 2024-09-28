use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
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
pub struct Selector<V: NodeValue> {
    #[deref]
    #[deref_mut]
    pub(super) p_node: ProtoNode,
    result_typ: PhantomData<V>,
    compute: Box<dyn Fn(&mut SelectorContext<'_, V>) -> V + 'static>,
}

impl<V: NodeValue> AnyNode<V> for Selector<V> {
    type Weak = WeakNode<V, Selector<V>>;

    fn key(&self) -> NodeKey {
        self.p_node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakNode {
            wp_node: self.p_node.downgrade(),
            value_typ: self.result_typ,
            node_typ: PhantomData::<Selector<V>>,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<V: NodeValue> Selector<V> {
    pub(super) fn new(
        key: NodeKey,
        compute: impl Fn(&mut SelectorContext<'_, V>) -> V + 'static,
        rc: Weak<RwLock<NodeRefCounter>>,
    ) -> Self {
        Self {
            p_node: ProtoNode::new(key, rc),
            compute: Box::new(compute),
            result_typ: PhantomData,
        }
    }

    pub fn read<'a>(&self, ctx: &'a mut Context) -> &'a V {
        ctx.read_selector(self)
    }

    pub fn compute(&self, ctx: &mut SelectorContext<'_, V>) -> V {
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

    pub(super) fn reserve<V>(
        &self,
        create_slot: impl FnOnce(&Self, NodeKey) -> Selector<V>,
    ) -> Slot<V, Selector<V>>
    where
        V: NodeValue,
    {
        let key = self.rc.write().counts.insert(1.into());
        Slot(create_slot(self, key), PhantomData)
    }

    pub(super) fn insert<V>(&mut self, key: NodeKey, value: V)
    where
        V: NodeValue,
    {
        self.computed_values = self.computed_values.update(key, Box::new(value));
    }

    pub(super) fn read<V>(&self, key: &NodeKey) -> &V
    where
        V: NodeValue,
    {
        // TODO: add check for valid context

        self.computed_values[key]
            .as_any_ref()
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<V>()
                )
            })
    }

    pub(super) fn begin_lease<'a, V>(&mut self, node: &'a Selector<V>) -> Lease<'a, V, Selector<V>>
    where
        V: NodeValue,
    {
        // TODO: add check for valid context

        let value = Some(self.computed_values.remove(&node.key).unwrap_or_else(|| {
            panic!(
                "cannot update {} node that is already being updated",
                std::any::type_name::<V>()
            )
        }));

        Lease {
            node,
            value,
            typ: PhantomData,
        }
    }

    pub(super) fn end_lease<V>(&mut self, mut lease: Lease<V, Selector<V>>)
    where
        V: NodeValue,
    {
        self.computed_values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}
