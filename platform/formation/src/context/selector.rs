use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    marker::PhantomData,
    ptr::NonNull,
    rc::Rc,
    sync::{Arc, Weak},
};

use super::{
    node::{
        AnyNode, AnyNodeValue, Lease, NodeKey, NodeRefCounter, NodeValue, ProtoNode, Slot, WeakNode,
    },
    selector_context::SelectorContext,
    AnyContext, Context,
};

/// Marker structure for type erasure.
/// Used to cast different types to a common abstract type.
struct Abstract(());

/// Represents a computer that can store and invoke selector
/// computers with different types.
pub(super) struct Computer {
    /// Pointer to the stored callback.
    data: NonNull<Abstract>,
    /// Function pointer to call the stored callback.
    call: unsafe fn(NonNull<Abstract>, NonNull<Abstract>, NonNull<Abstract>),
    /// Function pointer to drop the stored callback.
    drop: unsafe fn(NonNull<Abstract>),
    /// PhantomData to prevent `Send` and `Sync` implementations.
    not_send: PhantomData<Rc<()>>,
}

impl Drop for Computer {
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.data);
        }
    }
}

impl Computer {
    /// Creates a new `Computer` with the provided callback.
    /// This function uses `unsafe` code to perform type erasure and manage memory manually.
    pub(super) fn new<R, F>(f: F) -> Self
    where
        R: NodeValue,
        F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
    {
        // The function that calls the stored callback.
        unsafe fn call<R, F>(
            data: NonNull<Abstract>,
            ctx: NonNull<Abstract>,
            result: NonNull<Abstract>,
        ) where
            R: NodeValue,
            F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
        {
            let f = &*(data.cast::<F>().as_ref());
            let ctx = &mut *(ctx.cast::<SelectorContext<'_, R>>().as_ptr());

            let v = f(ctx);
            std::ptr::write(result.cast::<R>().as_ptr(), v)
        }

        // The function that drops the stored callback.
        unsafe fn drop<R, F>(data: NonNull<Abstract>)
        where
            R: NodeValue,
            F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
        {
            // Reconstruct and drop to free memory.
            let _ = Box::from_raw(data.cast::<F>().as_ptr());
        }

        // Box the callback and convert it to a raw pointer.
        let boxed_f = Box::new(f) as Box<F>;
        let raw_f = Box::into_raw(boxed_f);
        let data = unsafe { NonNull::new_unchecked(raw_f as *mut Abstract) };

        Computer {
            data,
            call: call::<R, F>,
            drop: drop::<R, F>,
            not_send: PhantomData::default(),
        }
    }

    /// Calls the stored callback with the provided context and returns the result  of type `V`.
    /// This function is `unsafe` because it assumes that `ctx` is of the correct type `V` and that
    /// the stored callback corresponds to this type.
    pub(super) unsafe fn compute<R: NodeValue>(&self, ctx: &mut SelectorContext<'_, R>) -> R {
        let mut result: R = std::mem::MaybeUninit::uninit().assume_init();
        let ctx_ptr = NonNull::from(ctx).cast::<Abstract>();
        let result_ptr = NonNull::new(&mut result as *mut _ as *mut Abstract).unwrap();

        (self.call)(self.data, ctx_ptr, result_ptr);
        result
    }
}

/// Represents the context in which a selector operates.
/// Holds a mutable reference to the main `Context` and a weak reference to the selector.
#[derive(Deref, DerefMut)]
pub struct Selector<V: NodeValue> {
    #[deref]
    #[deref_mut]
    pub(super) p_node: ProtoNode,
    result_typ: PhantomData<V>,
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
        Some(Selector {
            p_node: weak.wp_node.upgrade()?,
            result_typ: weak.value_typ,
        })
    }
}

impl<V: NodeValue> Selector<V> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            p_node: ProtoNode::new(key, rc),
            result_typ: PhantomData,
        }
    }

    pub fn read<'a>(&self, ctx: &'a mut Context) -> &'a V {
        ctx.read_selector(self)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        any::Any,
        sync::atomic::{AtomicUsize, Ordering},
    };

    #[derive(Debug, Clone, PartialEq)]
    struct TestString(String);

    impl AnyNodeValue for TestValue {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone)]
    struct TestValue {
        a: usize,
    }

    impl AnyNodeValue for TestString {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_computer_creation_and_compute() {
        let ctx = &mut Context::new();
        let atom_a = ctx.create_atom(|_| TestValue { a: 0 });

        ctx.update_atom(&atom_a, |this, _| {
            this.a += 10;
        });

        let atom_a_key = atom_a.key();
        let selector_a = ctx.new_selector(move |selector_context| {
            let atom_a_value = selector_context.read::<TestValue>(&atom_a_key);

            let result = format!("Hello, {}!", atom_a_value.a);
            TestString(result)
        });

        let selector_a_result = selector_a.read(ctx);
        assert_eq!(selector_a_result, &TestString("Hello, 10!".to_string()));

        ctx.update_atom(&atom_a, |this, atom_context| {
            this.a += 10;

            atom_context.notify();
        });

        let selector_a_result = selector_a.read(ctx);
        assert_eq!(selector_a_result, &TestString("Hello, 20!".to_string()));
    }

    #[test]
    fn test_computer_drop() {
        // A counter to track drop calls.
        struct DropCounter {
            count: Rc<AtomicUsize>,
        }

        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.count.fetch_add(1, Ordering::SeqCst);
            }
        }

        let counter = Rc::new(AtomicUsize::new(0));
        let counter_clone = Rc::clone(&counter);

        let computer = {
            let drop_counter = DropCounter {
                count: counter_clone,
            };

            // Create a `Computer` that takes a DropCounter in its closure.
            Computer::new(move |_: &mut SelectorContext<'_, TestValue>| -> TestValue {
                // Closure uses drop_counter, which will be dropped when the Computer is dropped.
                let _ = &drop_counter;

                TestValue { a: 0 }
            })
        };

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        drop(computer); //  must drop the DropCounter inside the closure
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
