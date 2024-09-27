use platform_core::context::EventEmitter;
use std::any::TypeId;

use super::{
    atom::Atom,
    node::{AnyNode, NodeValue, WeakNode},
    selector::Selector,
    selector_context::SelectorContext,
    subscription::Subscription,
    AnyContext, Context,
};

pub struct AtomContext<'a, V: NodeValue> {
    ctx: &'a mut Context,
    weak: WeakNode<V, Atom<V>>,
}

impl<V: NodeValue> AnyContext for AtomContext<'_, V> {
    type Result<T> = T;

    fn new_atom<T>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>
    where
        T: NodeValue,
    {
        self.ctx.new_atom(build_atom)
    }

    fn read_atom<T>(&self, atom: &Atom<T>) -> &T
    where
        T: NodeValue,
    {
        self.ctx.read_atom(atom)
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: NodeValue,
    {
        todo!()
    }

    fn new_selector<T>(
        &mut self,
        build_selector: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>
    where
        T: NodeValue,
    {
        todo!()
    }

    fn read_selector<T>(&mut self, atom: &Selector<T>) -> &T
    where
        T: NodeValue,
    {
        todo!()
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
