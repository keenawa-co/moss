use std::sync::Arc;

use crate::context::SelectorContext;

use super::{
    atom::Atom,
    atom_context::AtomContext,
    node::{AnyNode, NodeValue},
    selector::Selector,
    AnyContext, Context, NonTransactableContext,
};

// ----------------------------------------------------------------------------
// Atom

pub(super) fn stage_create_atom<T: NodeValue>(
    ctx: &mut Context,
    callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
) -> Atom<T> {
    ctx.stage(|ctx| {
        let slot = ctx
            .next_tree()
            .atom_values
            .reserve(|map, key| Atom::new(key, Arc::downgrade(&map.rc)));
        let value = callback(&mut AtomContext::new(ctx, slot.downgrade()));

        ctx.next_tree_mut().atom_values.insert(slot, value)
    })
}

pub(super) fn stage_update_atom<T: NodeValue, R>(
    ctx: &mut Context,
    atom: &Atom<T>,
    callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
) -> R {
    ctx.stage(|tx_ctx| {
        let mut value = tx_ctx.next_tree_mut().atom_values.begin_lease(atom);
        let result = callback(&mut value, &mut AtomContext::new(tx_ctx, atom.downgrade()));

        tx_ctx.next_tree_mut().atom_values.end_lease(value);
        tx_ctx.next_tree_mut().dirty_atoms.insert(atom.key());

        result
    })
}

pub(super) fn read_atom<'a, C, T>(ctx: &'a C, atom: &Atom<T>) -> &'a T
where
    C: AnyContext + NonTransactableContext,
    T: NodeValue,
{
    ctx.as_ref()
        .store
        .current_tree
        .atom_values
        .read(&atom.key())
}

// ----------------------------------------------------------------------------
// Selector

pub(super) fn stage_create_selector<T: NodeValue>(
    ctx: &mut Context,
    callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
) -> Selector<T> {
    ctx.stage(|ctx| {
        let slot = ctx
            .next_tree()
            .selector_values
            .reserve(|map, key| Selector::new(key, callback, Arc::downgrade(&map.rc)));

        ctx.store.known_selectors.insert(slot.key());

        slot.0
    })
}

pub fn resolve_selector<'a, C, T>(ctx: &'a mut C, selector: &Selector<T>) -> &'a T
where
    C: AnyContext + NonTransactableContext,
    T: NodeValue,
{
    if !ctx
        .as_ref()
        .store
        .current_tree
        .selector_values
        .lookup(&selector.key())
    {
        ctx.as_mut().stage(|transaction_context| {
            let value = selector.compute(&mut SelectorContext::new(
                transaction_context,
                selector.downgrade(),
            ));

            transaction_context
                .next_tree_mut()
                .selector_values
                .insert(selector.key(), value);
        });
    }

    ctx.as_ref()
        .store
        .current_tree
        .selector_values
        .read(&selector.key())
}
