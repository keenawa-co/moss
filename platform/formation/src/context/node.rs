use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use slotmap::SlotMap;
use std::{any::Any, marker::PhantomData, sync::atomic::AtomicUsize};

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
