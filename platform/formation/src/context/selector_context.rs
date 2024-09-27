use super::{
    atom::Atom,
    atom_context::AtomContext,
    node::{NodeKey, NodeValue, WeakNode},
    selector::Selector,
    AnyContext, Context,
};

pub struct SelectorContext<'a, V: NodeValue> {
    ctx: &'a mut Context,
    weak: WeakNode<V, Selector<V>>,
}

impl<'a, V: NodeValue> AnyContext for SelectorContext<'a, V> {
    type Result<T> = T;

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        todo!()
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        todo!()
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        todo!()
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        build_selector: impl FnOnce(&mut SelectorContext<'_, T>) -> T,
    ) -> Self::Result<Selector<T>> {
        todo!()
    }

    fn read_selector<T: NodeValue>(&mut self, atom: &Selector<T>) -> &T {
        todo!()
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
