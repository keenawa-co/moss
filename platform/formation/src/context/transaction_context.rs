#![feature(negative_impls)]

use derive_more::{Deref, DerefMut};

use super::atom::Atom;
use super::atom_context::AtomContext;
use super::common;
use super::node::{AnyNode, NodeValue};
use super::selector::Selector;
use super::selector_context::SelectorContext;
use super::AnyContext;
use super::Context;

#[derive(Deref, DerefMut)]
pub struct TransactionContext<'a> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    depth_value: usize,
}

impl<'a> Drop for TransactionContext<'a> {
    fn drop(&mut self) {
        self.assert_valid_depth_value(self.ctx.batcher.commit_depth.get());

        if self.ctx.batcher.dec_commit_depth() > 0 {
            return;
        } else {
            self.commit();
        }
    }
}

impl<'a> From<&'a mut Context> for TransactionContext<'a> {
    fn from(ctx: &'a mut Context) -> Self {
        let depth_value = ctx.batcher.inc_commit_depth();

        Self { ctx, depth_value }
    }
}

impl<'a> AnyContext for TransactionContext<'a> {
    type Result<T> = T;

    fn create_atom<T: NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        self.next_tree().atom_values.read(&atom.key())
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        common::stage_update_atom(self, atom, callback)
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>> {
        common::stage_create_selector(self, callback)
    }

    fn read_selector<T: NodeValue>(&mut self, selector: &Selector<T>) -> &T {
        todo!()
        // if !self.next_tree().selector_values.lookup(&selector.key()) {
        //     self.stage(|transaction_context| {
        //         let value = selector.compute(&mut SelectorContext::new(
        //             transaction_context,
        //             selector.downgrade(),
        //         ));

        //         transaction_context
        //             .next_tree_mut()
        //             .selector_values
        //             .insert(selector.key(), value);
        //     });
        // }

        // self.next_tree().selector_values.read(&selector.key())
    }
}

impl<'a> TransactionContext<'a> {
    fn assert_valid_depth_value(&self, prev_value: usize) {
        debug_assert!(
            self.depth_value == prev_value,
            "inconsistent decrementation of transaction context, context depth {}, expected {}",
            self.depth_value,
            prev_value
        );
    }
}
