use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use parking_lot::RwLock;
use platform_core::context::entity::Model;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    cell::OnceCell,
    marker::PhantomData,
    mem,
    ops::DerefMut,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

use crate::context::{TransactionContext, TreeState};

use super::{
    subscription::{SubscriberSet, Subscription},
    AnyContext, AnyStateProvider, Context,
};

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

impl<T: 'static> AnyNode<T> for Atom<T> {
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

impl<T: 'static> AnyAtom<T> for Atom<T> {}

impl<T: 'static> Atom<T> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            node: ProtoAtom::new(key, TypeId::of::<T>(), rc),
            typ: PhantomData,
        }
    }

    #[allow(private_bounds)]
    pub fn read<'a, C: AnyContext + AnyStateProvider>(&self, ctx: &'a mut C) -> &'a T {
        if ctx.tree().is_some() {
            ctx.tree().unwrap().atom_values.read(&self.key)
        } else {
            let tree = ctx.with_next_tree();
            tree.atom_values.read(&self.key)
        }
    }
}

pub(super) struct Lease<'a, T> {
    node: &'a Atom<T>,
    value: Option<Box<dyn AnyNodeValue>>,
    typ: PhantomData<T>,
}

impl<'a, T: 'static> core::ops::Deref for Lease<'a, T> {
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

impl<'a, T: 'static> core::ops::DerefMut for Lease<'a, T> {
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

#[derive(Clone)]
pub(super) struct AtomMap {
    values: im::HashMap<NodeKey, Box<dyn AnyNodeValue>>,
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

    pub fn reserve<T: 'static>(&self) -> Slot<T> {
        let id = self.rc.write().counts.insert(1.into());
        Slot(Atom::new(id, Arc::downgrade(&self.rc)))
    }

    pub fn insert<T: AnyNodeValue>(&mut self, slot: Slot<T>, entity: T) -> Atom<T>
    where
        T: 'static + Clone,
    {
        let atom = slot.0;
        dbg!(std::any::type_name::<T>());
        self.values = self.values.update(atom.key, Box::new(entity));

        atom
    }

    pub fn read<T: 'static>(&self, key: &NodeKey) -> &T {
        // TODO: add check for valid context
        dbg!(&self.values.len());
        &self.values[key]
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            })
    }

    pub fn begin_lease<'a, T: 'static>(&mut self, node: &'a Atom<T>) -> Lease<'a, T> {
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

    fn new_atom<T>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>
    where
        T: 'static + AnyNodeValue + Clone,
    {
        todo!()
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        todo!()
    }

    fn new_selector<T>(
        &mut self,
        build_selector: impl FnOnce(&mut SelectorContext<'_, T>) -> T,
    ) -> Self::Result<Selector<T>>
    where
        T: 'static + AnyNodeValue + Clone,
    {
        todo!()
    }
}

impl<'a, V> AnyStateProvider for SelectorContext<'a, V> {
    fn tree(&self) -> Option<&TreeState> {
        todo!()
    }

    fn with_next_tree(&mut self) -> &TreeState {
        todo!()
    }
}

impl<'a, V> SelectorContext<'a, V> {
    pub(super) fn new(ctx: &'a mut Context, weak: WeakAtom<V>) -> Self {
        Self { ctx, weak }
    }

    pub fn read<T: 'static>(&mut self, key: &NodeKey) -> &T {
        // TODO: The fact of reading means the subscription is initialized.
        // Implement saving connections with other entities for further processing

        self.ctx.store.current_tree.atom_values.read::<T>(key)
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
    func: OnceCell<Box<dyn Fn(&mut SelectorContext<'_, T>) -> T>>,
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

impl<T: 'static + AnyNodeValue> Selector<T> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            node: ProtoAtom::new(key, TypeId::of::<T>(), rc),
            func: OnceCell::new(),
            typ: PhantomData,
        }
    }

    fn set(&self, transformer: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static) {
        self.func
            .set(Box::new(transformer))
            .unwrap_or_else(|_| panic!("can only update the transform function once"));
    }

    pub fn read<'a, C: AnyContext + AsMut<Context>>(&self, ctx: &'a mut C) -> &'a T {
        ctx.as_mut().commit(|cx| {
            let mut lease = cx
                .store
                .next_tree
                .as_mut()
                .unwrap()
                .selector_values
                .begin_lease(self);

            if lease.value.is_none() {
                let transformer_fn = self.func.get().unwrap();
                let result = transformer_fn(&mut SelectorContext::new(cx, self.downgrade()));
                lease.value = Some(Box::new(result));
            }

            cx.store
                .next_tree
                .as_mut()
                .unwrap()
                .selector_values
                .end_lease(lease);
        });

        ctx.as_mut()
            .store
            .current_tree
            .selector_values
            .read(&self.key)
            .unwrap()
    }
}

#[derive(Clone)]
pub(super) struct SelectorMap {
    values: im::HashMap<NodeKey, Option<Box<dyn AnyNodeValue>>>,
    rc: Arc<RwLock<NodeRefCounter>>,
}

impl SelectorMap {
    pub fn new() -> Self {
        Self {
            values: im::HashMap::new(),
            rc: Arc::new(RwLock::new(NodeRefCounter {
                counts: SlotMap::with_key(),
                dropped: Vec::new(),
            })),
        }
    }

    pub fn reserve<T: 'static + AnyNodeValue>(&self) -> SlotNode<T, Selector<T>> {
        let id = self.rc.write().counts.insert(1.into());
        SlotNode(Selector::new(id, Arc::downgrade(&self.rc)), PhantomData)
    }

    pub fn insert<T>(
        &mut self,
        slot: SlotNode<T, Selector<T>>,
        transformer: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Selector<T>
    where
        T: 'static + AnyNodeValue + Clone,
    {
        let selector = slot.0;
        selector.set(transformer);
        self.values = self.values.update(selector.key, None);

        selector
    }

    pub fn read<T: 'static>(&self, key: &NodeKey) -> Option<&T> {
        // TODO: add check for valid context

        if let Some(value) = &self.values[key] {
            Some(value.as_any_ref().downcast_ref::<T>().unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            }))
        } else {
            None
        }
    }

    pub fn begin_lease<'a, T: 'static>(&mut self, node: &'a Selector<T>) -> SelectorLease<'a, T> {
        // TODO: add check for valid context

        let value = Some(self.values.remove(&node.key).unwrap_or_else(|| {
            panic!(
                "cannot update {} node that is already being updated",
                std::any::type_name::<T>()
            )
        }))
        .unwrap();

        SelectorLease {
            value,
            node,
            typ: PhantomData,
        }
    }

    pub fn end_lease<T>(&mut self, mut lease: SelectorLease<T>) {
        self.values
            .insert(lease.node.key, Some(lease.value.take().unwrap()));
    }
}

pub(super) struct SelectorLease<'a, T> {
    node: &'a Selector<T>,
    value: Option<Box<dyn AnyNodeValue>>,
    typ: PhantomData<T>,
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
