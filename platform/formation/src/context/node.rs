use anyhow::Result;
use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Weak,
    },
};

slotmap::new_key_type! {
    pub struct NodeKey;
}

#[derive(Deref, DerefMut)]
pub struct Slot<T, N: AnyNode<T>>(
    #[deref]
    #[deref_mut]
    pub(super) N,
    pub(super) PhantomData<T>,
);

pub(super) struct NodeRefCounter {
    pub counts: SlotMap<NodeKey, AtomicUsize>,
    pub dropped: Vec<NodeKey>,
}

pub trait AnyNode<T> {
    type Weak: 'static;

    fn key(&self) -> NodeKey;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

dyn_clone::clone_trait_object!(AnyNodeValue);
pub trait AnyNodeValue: Any + DynClone {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait NodeValue: AnyNodeValue + Clone + 'static {}
impl<T: AnyNodeValue + Clone + 'static> NodeValue for T {}

pub struct ProtoNode {
    pub(super) key: NodeKey,
    typ: TypeId,
    rc: Weak<RwLock<NodeRefCounter>>,
}

impl ProtoNode {
    pub(super) fn new(key: NodeKey, typ: TypeId, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            key,
            typ,
            rc: rc.clone(),
        }
    }

    pub(super) fn downgrade(&self) -> WeakProtoNode {
        WeakProtoNode {
            key: self.key,
            typ: self.typ,
            rc: self.rc.clone(),
        }
    }
}

#[derive(Clone)]
// AnyWeakModel
pub struct WeakProtoNode {
    pub(super) key: NodeKey,
    pub(super) typ: TypeId,
    pub(super) rc: Weak<RwLock<NodeRefCounter>>,
}

impl WeakProtoNode {
    pub(super) fn upgrade(&self) -> Option<ProtoNode> {
        let ref_counts = &self.rc.upgrade()?;
        let ref_counts_lock = ref_counts.read();
        let ref_count = ref_counts_lock.counts.get(self.key)?;

        if ref_count.load(Ordering::SeqCst) == 0 {
            return None;
        }

        ref_count.fetch_add(1, Ordering::SeqCst);
        drop(ref_counts_lock);

        Some(ProtoNode {
            key: self.key,
            typ: self.typ,
            rc: self.rc.clone(),
        })
    }
}

#[derive(Deref, DerefMut)]
pub struct WeakNode<T, N: AnyNode<T>> {
    #[deref]
    #[deref_mut]
    pub(super) weak_proto_atom: WeakProtoNode,
    pub(super) typ: PhantomData<T>,
    pub(super) node_typ: PhantomData<N>,
}

unsafe impl<T, N: AnyNode<T>> Send for WeakNode<T, N> {}
unsafe impl<T, N: AnyNode<T>> Sync for WeakNode<T, N> {}

impl<T, N: AnyNode<T>> Clone for WeakNode<T, N> {
    fn clone(&self) -> Self {
        Self {
            weak_proto_atom: self.weak_proto_atom.clone(),
            typ: self.typ,
            node_typ: self.node_typ,
        }
    }
}

pub(super) struct Lease<'a, T, N: AnyNode<T>> {
    pub node: &'a N,
    pub value: Option<Box<dyn AnyNodeValue>>,
    pub typ: PhantomData<T>,
}

impl<'a, T: 'static, N: AnyNode<T>> core::ops::Deref for Lease<'a, T, N> {
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

impl<'a, T: 'static, N: AnyNode<T>> core::ops::DerefMut for Lease<'a, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
            .as_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }
}

impl<'a, T, N: AnyNode<T>> Drop for Lease<'a, T, N> {
    fn drop(&mut self) {
        if self.value.is_some() && !std::thread::panicking() {
            panic!("Drop node which is in leasing")
        }
    }
}
