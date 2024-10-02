use derive_more::{Deref, DerefMut};

use super::{
    atom::Atom,
    atom_context::AtomContext,
    common,
    node::{NodeKey, NodeValue, WeakNode},
    selector::Selector,
    AnyContext, Context, NonTransactableContext,
};

#[derive(Deref, DerefMut)]
pub struct SelectorContext<'a, V: NodeValue> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    weak: WeakNode<V, Selector<V>>,
}

impl<V: NodeValue> NonTransactableContext for SelectorContext<'_, V> {
    fn as_mut(&mut self) -> &mut Context {
        self
    }

    fn as_ref(&self) -> &Context {
        self
    }
}

impl<'a, V: NodeValue> AnyContext for SelectorContext<'a, V> {
    type Output<T> = T;
    type ReadOutput<'b, T: 'b> = &'b T where 'a: 'b;

    fn create_atom<T: NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Output<Atom<T>> {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<'b, T: NodeValue>(&'b self, atom: &Atom<T>) -> Self::ReadOutput<'b, T> {
        common::read_atom::<Self, _>(self, atom)
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Output<R> {
        common::stage_update_atom(self, atom, callback)
    }

    fn create_selector<T: NodeValue>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Output<Selector<T>> {
        common::stage_create_selector(self, callback)
    }

    fn read_selector<'b, T: NodeValue>(
        &'b mut self,
        selector: &Selector<T>,
    ) -> Self::ReadOutput<'b, T> {
        common::resolve_selector(self, selector)
    }
}

impl<'a, V: NodeValue> SelectorContext<'a, V> {
    pub(super) fn new(ctx: &'a mut Context, weak: WeakNode<V, Selector<V>>) -> Self {
        Self { ctx, weak }
    }

    fn origin_key(&self) -> &NodeKey {
        &self.weak.key
    }

    pub fn read<T: NodeValue>(&self, key: &NodeKey) -> &T {
        // TODO: The fact of reading means the subscription is initialized.
        // Implement saving connections with other entities for further processing

        let origin_key = self.origin_key();

        if self
            .ctx
            .read_graph(|graph| graph.has_subscription(origin_key, key))
        {
            self.ctx.store.current_tree.atom_values.read::<T>(key)
        } else {
            self.ctx.advance_graph(|graph| {
                graph.create_dependency(origin_key.to_owned(), key.clone());
            });

            self.ctx.store.current_tree.atom_values.read::<T>(key)
        }
    }
}
