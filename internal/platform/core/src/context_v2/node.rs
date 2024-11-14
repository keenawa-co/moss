use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::Any,
    marker::PhantomData,
    mem,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

use moss_base::collection::ImHashMap;

// Represents the key for a node.
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

/// Represents the reference counter for nodes.
/// Manages counts of references to nodes.
pub(super) struct NodeRefCounter {
    pub counts: SlotMap<NodeKey, AtomicUsize>,
    pub dropped: Vec<NodeKey>,
}

pub trait AnyNode<T> {
    type Weak: 'static;

    /// Returns the key associated with the atom.
    fn key(&self) -> NodeKey;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

/// Trait representing a node value.
/// Combines `Any` for type erasure and `Clone` for duplicating values.
pub trait AnyNodeValue: Any + DynClone {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
dyn_clone::clone_trait_object!(AnyNodeValue);

/// Trait representing a node value that can be used in the system.
pub trait NodeValue: AnyNodeValue + Clone + 'static {}
impl<T: AnyNodeValue + Clone + 'static> NodeValue for T {}

pub struct ProtoNode {
    pub(super) key: NodeKey,
    pub(super) rc: Weak<RwLock<NodeRefCounter>>,
}

impl Clone for ProtoNode {
    fn clone(&self) -> Self {
        if let Some(ref_counts) = self.rc.upgrade() {
            let ref_counts_lock = ref_counts.read();
            let count = ref_counts_lock
                .counts
                .get(self.key)
                .expect("node over-release has been detected");
            let prev_count = count.fetch_add(1, Ordering::SeqCst);
            assert_ne!(
                prev_count, 0,
                "detected node release beyond permissible levels"
            );
        }

        Self {
            key: self.key,
            rc: self.rc.clone(),
        }
    }
}

impl ProtoNode {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            key,
            rc: rc.clone(),
        }
    }

    pub(super) fn downgrade(&self) -> WeakProtoNode {
        WeakProtoNode {
            key: self.key,
            rc: self.rc.clone(),
        }
    }
}

#[derive(Clone)]
pub struct WeakProtoNode {
    pub(super) key: NodeKey,
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
            rc: self.rc.clone(),
        })
    }
}

/// Represents a weak reference to a node.
#[derive(Deref, DerefMut)]
pub struct WeakNode<T, N: AnyNode<T>> {
    #[deref]
    #[deref_mut]
    pub(super) wp_node: WeakProtoNode,
    pub(super) value_typ: PhantomData<T>,
    pub(super) node_typ: PhantomData<N>,
}

unsafe impl<T, N: AnyNode<T>> Send for WeakNode<T, N> {}
unsafe impl<T, N: AnyNode<T>> Sync for WeakNode<T, N> {}

impl<T, N: AnyNode<T>> Clone for WeakNode<T, N> {
    fn clone(&self) -> Self {
        Self {
            wp_node: self.wp_node.clone(),
            value_typ: self.value_typ,
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

#[derive(Clone)]
pub(super) struct NodeImMap {
    pub values: ImHashMap<NodeKey, Box<dyn AnyNodeValue>>,
    pub rc: Arc<RwLock<NodeRefCounter>>,
}

impl NodeImMap {
    pub(super) fn new() -> Self {
        Self {
            values: ImHashMap::new(),
            rc: Arc::new(RwLock::new(NodeRefCounter {
                counts: SlotMap::with_key(),
                dropped: Vec::new(),
            })),
        }
    }

    pub(super) fn begin_lease<'a, V, N>(&mut self, node: &'a N) -> Lease<'a, V, N>
    where
        V: NodeValue,
        N: AnyNode<V>,
    {
        // TODO: add check for valid context

        let value = Some(self.values.remove(&node.key()).unwrap_or_else(|| {
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

    pub fn end_lease<V, N>(&mut self, mut lease: Lease<V, N>)
    where
        V: NodeValue,
        N: AnyNode<V>,
    {
        self.values
            .insert(lease.node.key(), lease.value.take().unwrap());
    }

    pub(super) fn take_dropped(&mut self) -> Vec<(NodeKey, Box<dyn AnyNodeValue>)> {
        let mut ref_counts_lock = self.rc.write();
        let dropped_nodes = mem::take(&mut ref_counts_lock.dropped);
        dropped_nodes
            .into_iter()
            .filter_map(|node_key| {
                let count = ref_counts_lock.counts.remove(node_key).unwrap();
                NodeImMap::assert_referenced(count);

                Some((node_key, self.values.remove(&node_key)?))
            })
            .collect()
    }
}

impl NodeImMap {
    fn assert_referenced(count: AtomicUsize) {
        debug_assert_eq!(
            count.load(std::sync::atomic::Ordering::SeqCst),
            0,
            "dropped a node that was referenced"
        );
    }
}
