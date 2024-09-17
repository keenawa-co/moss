use anyhow::Result;
use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use moss_std::collection::FxHashSet;
use parking_lot::RwLock;
use platform_core::context::entity::Model;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem,
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

pub(super) struct NodeValueMap(SecondaryMap<NodeKey, Box<dyn Any>>);

impl NodeValueMap {
    pub fn new() -> Self {
        Self(SecondaryMap::new())
    }

    pub fn insert(&mut self, key: NodeKey, value: Box<dyn Any>) {
        self.0.insert(key, value);
    }

    pub fn read<T: 'static>(&self, node: &Atom<T>) -> Option<&T> {
        self.0.get(node.key())?.downcast_ref()
    }
}

pub struct NodeRefCounter {
    counts: SlotMap<NodeKey, AtomicUsize>,
    dropped: Vec<NodeKey>,
}

// AnyModel
// pub struct Node {
//     key: NodeKey,
//     typ: TypeId,
//     rc: Weak<RwLock<NodeRefCounter>>,
// }

// impl Node {
//     fn new(key: NodeKey, typ: TypeId, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
//         Self { key, typ, rc }
//     }
// }

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

// pub struct ProtoAtom {
//     key: NodeKey,
//     typ: TypeId,
//     rc: Weak<RwLock<RefCounter>>,
// }

// impl ProtoAtom {
//     fn new(key: NodeKey, typ: TypeId, rc: Weak<RwLock<RefCounter>>) -> Self {
//         Self {
//             key,
//             typ,
//             rc: rc.clone(),
//         }
//     }
// }

// #[derive(Deref, DerefMut)]
// pub struct Atom<T> {
//     #[deref]
//     #[deref_mut]
//     proto: ProtoAtom,
//     typ: PhantomData<T>,
// }

// impl<T: 'static> Atom<T> {
//     fn new(key: NodeKey, rc: Weak<RwLock<RefCounter>>) -> Self {
//         Self {
//             proto: ProtoAtom::new(key, TypeId::of::<T>(), rc),
//             typ: PhantomData,
//         }
//     }
// }

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

    pub fn read<'a>(&self, ctx: &'a mut Context) -> &'a T {
        if ctx.tree().is_some() {
            ctx.tree().unwrap().atom_values.read(self)
        } else {
            let tree = ctx.with_next_tree();
            tree.atom_values.read(self)
        }

        // let tree = if let Some(tree) = ctx.tree() {
        //     tree
        // } else {
        //     ctx.with_next_tree()
        // };

        // tree.atom_values.read(self)

        // let r = (ctx as &dyn Any).downcast_ref::<TransactionContext>();

        // let value = ctx.store.current_tree.atom_values.read::<T>(self);

        // if value.is_some() {
        //     return value.unwrap();
        // } else {
        //     let tx = ctx.begin();
        // }

        // tx.exec(|tx_ctx| {
        //     let tree = tx_ctx.store.next_tree.unwrap_or(TreeState::new());
        // });

        // todo!()
        // let value_option = ctx.store.current_tree.atom_values.get(self.key);

        // if let Some(value) = value_option {

        // return value.downcast_ref().unwrap_or_else(|| {
        //     panic!(
        //         "cannot read {} node that is being updated",
        //         std::any::type_name::<T>()
        //     )
        // });

        //     todo!()
        // } else {
        //
        //     let tx = ctx.begin();

        //     tx.exec(|tx_ctx| {
        //         let tree = tx_ctx.store.next_tree.unwrap_or(TreeState::new());
        //     });

        //     panic!("atom value is undefined");
        // }
    }
}

pub(super) struct Lease<'a, T> {
    entity: Option<Box<dyn Any>>,
    node: &'a Atom<T>,
    typ: PhantomData<T>,
}

impl<'a, T: 'static> core::ops::Deref for Lease<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.entity.as_ref().unwrap().downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> core::ops::DerefMut for Lease<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity.as_mut().unwrap().downcast_mut().unwrap()
    }
}

impl<'a, T> Drop for Lease<'a, T> {
    fn drop(&mut self) {
        if self.entity.is_some() && !std::thread::panicking() {
            panic!("Drop node which is in leasing")
        }
    }
}

pub trait AnyNodeValue: Any + DynClone {
    fn as_any(&self) -> &dyn Any;
}

dyn_clone::clone_trait_object!(AnyNodeValue);

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

    pub fn read<T: 'static>(&self, node: &Atom<T>) -> &T {
        // TODO: add check for valid context

        &self.values[&node.key]
            .as_any()
            .downcast_ref::<T>()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            })
    }

    // pub fn begin_lease<'a, T>(&mut self, node: &'a Atom<T>) -> Lease<'a, T> {
    //     // TODO: add check for valid context

    //     let entity = Some(self.nodes.remove(node.key).unwrap_or_else(|| {
    //         panic!(
    //             "cannot update {} node that is already being updated",
    //             std::any::type_name::<T>()
    //         )
    //     }));

    //     Lease {
    //         entity,
    //         node,
    //         typ: PhantomData,
    //     }
    // }

    // pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
    //     self.nodes
    //         .insert(lease.node.key, lease.entity.take().unwrap());
    // }
}

// ---------------

type SelectorCallback = Box<dyn FnMut(&mut Context) -> bool + 'static>;

pub struct SelectorContext<'a, T> {
    ctx: &'a mut Context,
    weak: WeakSelector<T>,
}

pub struct WeakSelector<T> {
    typ: PhantomData<T>,
}

pub struct Selector<T> {
    typ: PhantomData<T>,
    // compute
    observed_nodes: SubscriberSet<NodeKey, SelectorCallback>,
}

impl<T> Selector<T> {
    pub fn observe<Node, Type>(
        &mut self,
        node: &Node,
        mut callback: impl FnMut(Node, &mut Context) + 'static,
    ) -> Subscription
    where
        Type: 'static,
        Node: AnyNode<Type>,
    {
        self.observe_internal(node, move |n, ctx| {
            callback(n, ctx);
            true
        })
    }

    fn observe_internal<Node, Type>(
        &mut self,
        node: &Node,
        mut callback: impl FnMut(Node, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        Type: 'static,
        Node: AnyNode<Type>,
    {
        let node_key = node.key();
        let node_weak = node.downgrade();
        self.new_observer(
            node_key,
            Box::new(move |ctx| {
                if let Some(n) = Node::upgrade_from(&node_weak) {
                    callback(n, ctx)
                } else {
                    false
                }
            }),
        )
    }

    fn new_observer(&mut self, key: NodeKey, callback: SelectorCallback) -> Subscription {
        let (subscription, activate) = self.observed_nodes.insert(key, callback);

        activate();

        subscription
    }
}

pub struct SelectorMap {}

impl SelectorMap {
    pub fn new() -> Self {
        Self {}
    }
}
