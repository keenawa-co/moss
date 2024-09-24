use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use moss_std::collection::ImHashMap;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

use super::{AnyContext, Context};

slotmap::new_key_type! {
    pub struct NodeKey;
}

#[derive(Deref, DerefMut)]
pub struct Slot<T>(pub(super) Atom<T>);

#[derive(Deref, DerefMut)]
pub struct SlotNode<T, N: AnyNode<T>>(
    #[deref]
    #[deref_mut]
    pub(super) N,
    PhantomData<T>,
);

pub struct NodeRefCounter {
    counts: SlotMap<NodeKey, AtomicUsize>,
    dropped: Vec<NodeKey>,
}

pub struct AtomContext<'a, T> {
    ctx: &'a mut Context,
    weak: WeakAtom<T>,
}

impl<V> AnyContext for AtomContext<'_, V> {
    type Result<T> = T;

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        self.ctx.new_atom(build_atom)
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        todo!()
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        todo!()
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        build_selector: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>> {
        todo!()
    }

    fn read_selector<T: NodeValue>(&mut self, atom: &Selector<T>) -> &T {
        todo!()
    }
}

impl<'a, T: 'static> AtomContext<'a, T> {
    pub(super) fn new(ctx: &'a mut Context, weak: WeakAtom<T>) -> Self {
        Self { ctx, weak }
    }
}

pub trait AnyNode<T> {
    type Weak: 'static;

    fn key(&self) -> NodeKey;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

pub trait AnyAtom<T>: AnyNode<T> {}

#[derive(Clone)]
// AnyWeakModel
pub struct WeakProtoAtom {
    key: NodeKey,
    typ: TypeId,
    rc: Weak<RwLock<NodeRefCounter>>,
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

// AnyModel
pub struct ProtoAtom {
    key: NodeKey,
    typ: TypeId,
    rc: Weak<RwLock<NodeRefCounter>>,
}

impl ProtoAtom {
    fn new(key: NodeKey, typ: TypeId, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            key,
            typ,
            rc: rc.clone(),
        }
    }

    fn downgrade(&self) -> WeakProtoAtom {
        WeakProtoAtom {
            key: self.key,
            typ: self.typ,
            rc: self.rc.clone(),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct WeakAtom<T> {
    #[deref]
    #[deref_mut]
    weak_proto_atom: WeakProtoAtom,
    typ: PhantomData<T>,
}

#[derive(Deref, DerefMut)]
pub struct Atom<T> {
    #[deref]
    #[deref_mut]
    node: ProtoAtom,
    typ: PhantomData<T>,
}

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
        todo!()
    }
}

impl<T: NodeValue> AnyAtom<T> for Atom<T> {}

impl<T: NodeValue> Atom<T> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            node: ProtoAtom::new(key, TypeId::of::<T>(), rc),
            typ: PhantomData,
        }
    }

    pub fn read<'a, C: AnyContext>(&self, ctx: &'a mut C) -> &'a T {
        ctx.read_atom(self)
    }
}

pub(super) struct Lease<'a, T> {
    node: &'a Atom<T>,
    value: Option<Box<dyn AnyNodeValue>>,
    typ: PhantomData<T>,
}

impl<'a, T: NodeValue> core::ops::Deref for Lease<'a, T> {
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

impl<'a, T: NodeValue> core::ops::DerefMut for Lease<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
            .as_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }
}

impl<'a, T> Drop for Lease<'a, T> {
    fn drop(&mut self) {
        if self.value.is_some() && !std::thread::panicking() {
            panic!("Drop node which is in leasing")
        }
    }
}

dyn_clone::clone_trait_object!(AnyNodeValue);
pub trait AnyNodeValue: Any + DynClone {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait NodeValue: AnyNodeValue + Clone + 'static {}
impl<T: AnyNodeValue + Clone + 'static> NodeValue for T {}

#[derive(Clone)]
pub(super) struct AtomMap {
    values: ImHashMap<NodeKey, Box<dyn AnyNodeValue>>,
    rc: Arc<RwLock<NodeRefCounter>>,
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

    pub fn reserve<T: NodeValue>(&self) -> Slot<T> {
        let id = self.rc.write().counts.insert(1.into());
        Slot(Atom::new(id, Arc::downgrade(&self.rc)))
    }

    pub fn insert<T: NodeValue>(&mut self, slot: Slot<T>, entity: T) -> Atom<T> {
        let atom = slot.0;
        dbg!(std::any::type_name::<T>());
        self.values = self.values.update(atom.key, Box::new(entity));

        atom
    }

    pub fn read<T: NodeValue>(&self, key: &NodeKey) -> &T {
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

    pub fn begin_lease<'a, T: NodeValue>(&mut self, node: &'a Atom<T>) -> Lease<'a, T> {
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

    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.values
            .insert(lease.node.key, lease.value.take().unwrap());
    }
}

// ---------------

pub struct SelectorContext<'a, V> {
    ctx: &'a mut Context,
    weak: WeakAtom<V>,
}

impl<'a, V> AnyContext for SelectorContext<'a, V> {
    type Result<T> = T;

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        todo!()
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        todo!()
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        todo!()
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        build_selector: impl FnOnce(&mut SelectorContext<'_, T>) -> T,
    ) -> Self::Result<Selector<T>> {
        todo!()
    }

    fn read_selector<T: NodeValue>(&mut self, atom: &Selector<T>) -> &T {
        todo!()
    }
}

impl<'a, V> SelectorContext<'a, V> {
    pub(super) fn new(ctx: &'a mut Context, weak: WeakAtom<V>) -> Self {
        Self { ctx, weak }
    }

    fn origin_key(&self) -> &NodeKey {
        &self.weak.key
    }

    pub fn read<T: NodeValue>(&self, key: &NodeKey) -> &T {
        // TODO: The fact of reading means the subscription is initialized.
        // Implement saving connections with other entities for further processing

        let origin_key = self.origin_key();

        if self
            .ctx
            .read_graph(|graph| graph.has_subscription(origin_key, key))
        {
            self.ctx.store.current_tree.atom_values.read::<T>(key)
        } else {
            self.ctx.advance_graph(|graph| {
                graph.create_dependency(origin_key.to_owned(), key.clone());
            });

            self.ctx.store.current_tree.atom_values.read::<T>(key)
        }
    }
}

pub struct WeakSelector<T> {
    typ: PhantomData<T>,
}

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

    pub(super) fn reserve<T: 'static + AnyNodeValue, N: AnyNode<T>>(
        &self,
        create_slot: impl FnOnce(&Self, NodeKey) -> N,
    ) -> SlotNode<T, N> {
        let key = self.rc.write().counts.insert(1.into());
        SlotNode(create_slot(self, key), PhantomData)
    }

    pub(super) fn insert<T>(&mut self, key: NodeKey, value: T)
    where
        T: AnyNodeValue + Clone + 'static,
    {
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

impl<'a, T: AnyNodeValue + 'static> SelectorLease<'a, T> {
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
