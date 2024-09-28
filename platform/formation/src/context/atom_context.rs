use derive_more::{Deref, DerefMut};
use platform_core::context::EventEmitter;
use std::any::TypeId;

use super::{
    atom::Atom,
    common,
    node::{AnyNode, NodeValue, WeakNode},
    selector::Selector,
    selector_context::SelectorContext,
    subscription::Subscription,
    AnyContext, Context, NonTransactableContext,
};

#[derive(Deref, DerefMut)]
pub struct AtomContext<'a, V: NodeValue> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    weak: WeakNode<V, Atom<V>>,
}

impl<V: NodeValue> NonTransactableContext for AtomContext<'_, V> {
    fn as_mut(&mut self) -> &mut Context {
        self
    }

    fn as_ref(&self) -> &Context {
        self
    }
}

impl<V: NodeValue> AnyContext for AtomContext<'_, V> {
    type Result<T> = T;

    fn create_atom<T>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>
    where
        T: NodeValue,
    {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<T>(&self, atom: &Atom<T>) -> &T
    where
        T: NodeValue,
    {
        common::read_atom(self, atom)
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: NodeValue,
    {
        common::stage_update_atom(self, atom, callback)
    }

    fn new_selector<T>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>
    where
        T: NodeValue,
    {
        common::stage_create_selector(self, callback)
    }

    fn read_selector<T>(&mut self, selector: &Selector<T>) -> &T
    where
        T: NodeValue,
    {
        common::resolve_selector::<Self, _>(self, selector)
    }
}

impl<'a, V: NodeValue> AtomContext<'a, V> {
    pub(super) fn new(ctx: &'a mut Context, weak: WeakNode<V, Atom<V>>) -> Self {
        Self { ctx, weak }
    }

    pub fn weak_atom(&self) -> WeakNode<V, Atom<V>> {
        self.weak.clone()
    }
}

impl<'a, V: NodeValue> AtomContext<'a, V> {
    pub fn emit<E>(&mut self, event: E)
    where
        V: EventEmitter<E>,
        E: 'static,
    {
        self.ctx.push_effect(super::Effect::Event {
            emitter: self.weak.key,
            typ: TypeId::of::<E>(),
            payload: Box::new(event),
        });
    }

    pub fn notify(&mut self) {
        if self.ctx.batcher.pending_notifications.insert(self.weak.key) {
            self.ctx.push_effect(super::Effect::Notify {
                emitter: self.weak.key,
            });
        }
    }

    pub fn subscribe<T, N, P>(
        &mut self,
        node: &N,
        mut on_event: impl FnMut(&mut V, N, &P, &mut AtomContext<'_, V>) + 'static,
    ) -> Subscription
    where
        T: EventEmitter<P> + 'static,
        N: AnyNode<T>,
        P: 'static,
    {
        let this = self.weak_atom();
        self.ctx.subscribe_internal(node, move |n, payload, ctx| {
            if let Some(atom) = this.upgrade() {
                atom.update(ctx, |this, atom_context| {
                    on_event(this, n, payload, atom_context)
                });

                true
            } else {
                false
            }
        })
    }
}
