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

struct Abstract(());

pub(super) struct Computer {
    data: NonNull<Abstract>,
    call: unsafe fn(NonNull<Abstract>, NonNull<Abstract>, NonNull<Abstract>),
    drop_fn: unsafe fn(NonNull<Abstract>),
    not_send: PhantomData<Rc<()>>,
}

impl Drop for Computer {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.data);
        }
    }
}

impl Computer {
    pub(super) fn new<V, F>(hook: F) -> Self
    where
        V: NodeValue,
        F: Fn(&mut SelectorContext<'_, V>) -> V + 'static,
    {
        unsafe fn call<V, F>(
            data: NonNull<Abstract>,
            ctx: NonNull<Abstract>,
            result: NonNull<Abstract>,
        ) where
            V: NodeValue,
            F: Fn(&mut SelectorContext<'_, V>) -> V + 'static,
        {
            let f = &*(data.cast::<F>().as_ref());
            let ctx = &mut *(ctx.cast::<SelectorContext<'_, V>>().as_ptr());
            let v = f(ctx);
            std::ptr::write(result.cast::<V>().as_ptr(), v)
        }

        unsafe fn drop<V, F>(data: NonNull<Abstract>)
        where
            V: NodeValue,
            F: Fn(&mut SelectorContext<'_, V>) -> V + 'static,
        {
            let _ = Box::from_raw(data.cast::<F>().as_ptr());
        }

        let boxed_f = Box::new(hook) as Box<F>;
        let raw_f = Box::into_raw(boxed_f);
        let data = unsafe { NonNull::new_unchecked(raw_f as *mut Abstract) };

        Computer {
            data,
            call: call::<V, F>,
            drop_fn: drop::<V, F>,
            not_send: PhantomData::default(),
        }
    }

    pub(super) unsafe fn compute<V: NodeValue>(&self, ctx: &mut SelectorContext<'_, V>) -> V {
        let mut result: V = std::mem::MaybeUninit::uninit().assume_init();

        let ctx_ptr = NonNull::from(ctx).cast::<Abstract>();
        let result_ptr = NonNull::new(&mut result as *mut _ as *mut Abstract).unwrap();

        (self.call)(self.data, ctx_ptr, result_ptr);
        result
    }
}

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
            // compute: Box::new(compute),
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
