use derive_more::{Deref, DerefMut};
use std::any::TypeId;

use super::{
    atom::Atom,
    common,
    node::{AnyNode, NodeValue, WeakNode},
    selector::Selector,
    selector_context::SelectorContext,
    subscription::Subscription,
    AnyContext, Context, Emmiteble, NonTransactableContext,
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

impl<'a, V: NodeValue> AnyContext for AtomContext<'a, V> {
    type Output<T> = T;
    type ReadOutput<'b, T: 'b> = &'b T where 'a: 'b;

    fn create_atom<T>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Output<Atom<T>>
    where
        T: NodeValue,
    {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<'b, T: NodeValue>(&'b self, atom: &Atom<T>) -> Self::ReadOutput<'b, T> {
        common::read_atom(self, atom)
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Output<R>
    where
        T: NodeValue,
    {
        common::stage_update_atom(self, atom, callback)
    }

    fn create_selector<T>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Output<Selector<T>>
    where
        T: NodeValue,
    {
        common::stage_create_selector(self, callback)
    }

    fn read_selector<'b, T>(&'b mut self, selector: &Selector<T>) -> Self::ReadOutput<'b, T>
    where
        T: NodeValue,
    {
        common::resolve_selector(self, selector)
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
        V: Emmiteble<E>,
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
        T: Emmiteble<P> + 'static,
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
