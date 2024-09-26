use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::TypeId,
    marker::PhantomData,
    sync::{Arc, Weak},
};

use super::{
    atom::{ProtoAtom, WeakAtom},
    node::{AnyNode, AnyNodeValue, NodeKey, NodeRefCounter, NodeValue, SlotNode},
    selector_context::SelectorContext,
    AnyContext, Context,
};

#[derive(Deref, DerefMut)]
pub struct Selector<T> {
    #[deref]
    #[deref_mut]
    node: ProtoAtom,
    typ: PhantomData<T>,
    compute: Box<dyn Fn(&mut SelectorContext<'_, T>) -> T + 'static>,
    // observed_nodes: SubscriberSet<NodeKey, SelectorCallback>,
}

impl<T: 'static> AnyNode<T> for Selector<T> {
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
            node: ProtoAtom::new(key, TypeId::of::<T>(), rc),
            compute: Box::new(compute),
            typ: PhantomData,
        }
    }

    pub fn read<'a, C: AnyContext + AsMut<Context>>(&self, ctx: &'a mut C) -> &'a T {
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

    pub(super) fn reserve<T: NodeValue, N: AnyNode<T>>(
        &self,
        create_slot: impl FnOnce(&Self, NodeKey) -> N,
    ) -> SlotNode<T, N> {
        let key = self.rc.write().counts.insert(1.into());
        SlotNode(create_slot(self, key), PhantomData)
    }

    pub(super) fn insert<T: NodeValue>(&mut self, key: NodeKey, value: T) {
        self.computed_values = self.computed_values.update(key, Box::new(value));
    }

    pub(super) fn read<T: 'static>(&self, key: &NodeKey) -> &T {
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

    pub(super) fn begin_lease<'a, T: 'static>(
        &mut self,
        node: &'a Selector<T>,
    ) -> SelectorLease<'a, T> {
        // TODO: add check for valid context

        let value = Some(self.computed_values.remove(&node.key).unwrap_or_else(|| {
            panic!(
                "cannot update {} node that is already being updated",
                std::any::type_name::<T>()
            )
        }));

        SelectorLease {
            node,
            value,
            typ: PhantomData,
        }
    }

    pub(super) fn end_lease<T>(&mut self, mut lease: SelectorLease<T>) {
        self.computed_values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}

pub(super) struct SelectorLease<'a, T> {
    node: &'a Selector<T>,
    value: Option<Box<dyn AnyNodeValue>>,
    typ: PhantomData<T>,
}

impl<'a, T: NodeValue> SelectorLease<'a, T> {
    pub(super) fn set_value(&mut self, value: T) {
        self.value = Some(Box::new(value));
    }

    pub(super) fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl<'a, T: 'static> core::ops::Deref for SelectorLease<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
            .as_ref()
            .unwrap()
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap()
    }
}

impl<'a, T: 'static> core::ops::DerefMut for SelectorLease<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
            .as_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }
}

impl<'a, T> Drop for SelectorLease<'a, T> {
    fn drop(&mut self) {
        if self.value.is_some() && !std::thread::panicking() {
            panic!("Drop node which is in leasing")
        }
    }
}
