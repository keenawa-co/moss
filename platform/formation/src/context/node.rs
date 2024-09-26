use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use slotmap::SlotMap;
use std::{any::Any, marker::PhantomData, sync::atomic::AtomicUsize};

use super::atom::Atom;

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
