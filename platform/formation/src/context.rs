pub mod graph;
pub mod node;
pub mod subscription;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use derive_more::{Deref, DerefMut};
use graph::Graph;
use moss_std::collection::{FxHashMap, FxHashSet};
use node::{
    AnyNode, AnyNodeValue, Atom, AtomContext, AtomMap, NodeKey, Selector, SelectorContext,
    SelectorMap,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[derive(Clone)]
pub(super) struct TreeState {
    // A unique identifier for the state version.
    version: usize,
    atom_values: AtomMap,
    selector_values: SelectorMap,

    // Set of atoms that have changed.
    dirty_atoms: FxHashSet<NodeKey>,
}

impl TreeState {
    fn new(version: usize) -> Self {
        Self {
            version,
            atom_values: AtomMap::new(),
            selector_values: SelectorMap::new(),
            dirty_atoms: FxHashSet::default(),
        }
    }

    fn advance(&self) -> Self {
        TreeState {
            version: self.version + 1,
            atom_values: self.atom_values.clone(),
            selector_values: self.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        }
    }
}

pub struct StoreState {
    // The previous state, used during transitions.
    previous_tree: Option<TreeState>,
    // The current committed state.
    current_tree: TreeState,
    // The state being built during a transaction.
    next_tree: Option<TreeState>,
    // Used to detect nested transactions.
    // commit_depth: usize,
    // Dependency graphs for each version of the state.
    graph_by_version: FxHashMap<usize, Graph>,
}

pub(crate) trait AnyStateProvider {
    fn tree(&self) -> Option<&TreeState>;
    fn with_next_tree(&mut self) -> &TreeState;
}

impl StoreState {
    fn new() -> Self {
        Self {
            previous_tree: None,
            current_tree: TreeState::new(1),
            next_tree: None,
            graph_by_version: FxHashMap::default(),
        }
    }
}

// struct Batcher {
//     pending_updates: VecDeque<Box<dyn FnOnce(&mut Context) + 'static>>,
//     commit_depth: usize,
// }

// impl Batcher {
//     fn begin_transaction<'a, R>(&mut self, ctx: &mut TransactionContext) {
//         // self.commit_depth += 1;
//         // let result = (tx.callback)(&mut TransactionContext::new(tx.ctx));
//         // self.apply_updates(tx.ctx);

//         // self.commit_depth -= 1;

//         todo!()
//     }

//     fn apply_updates(&mut self, ctx: &mut Context) {
//         loop {
//             if let Some(update) = self.pending_updates.pop_front() {
//                 update(ctx)
//             } else {
//                 if self.pending_updates.is_empty() {
//                     break;
//                 }
//             }
//         }
//     }
// }

pub trait AnyContext {
    type Result<T>;

    fn new_atom<T>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>
    where
        T: 'static + AnyNodeValue + Clone;

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    fn new_selector<T>(
        &mut self,
        build_selector: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>
    where
        T: 'static + AnyNodeValue + Clone;
}

pub struct Context {
    // this: Weak<RefCell<Self>>,
    // batcher: Batcher,
    store: StoreState,
    // commit_depth: usize,
}

impl AnyContext for Context {
    type Result<T> = T;

    fn new_atom<T>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>
    where
        T: 'static + AnyNodeValue + Clone,
    {
        self.commit(|ctx| {
            if ctx.store.next_tree.is_none() {
                ctx.store.next_tree = Some(TreeState::new(ctx.store.current_tree.version));
            };

            let slot = ctx.store.next_tree.as_ref().unwrap().atom_values.reserve();
            let value = build_atom(&mut AtomContext::new(ctx, slot.downgrade()));

            ctx.store
                .next_tree
                .as_mut()
                .unwrap()
                .atom_values
                .insert(slot, value)
        })
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.commit(|ctx| ctx.update_atom(atom, callback))
    }

    fn new_selector<T>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>
    where
        T: 'static + AnyNodeValue + Clone,
    {
        self.commit(|ctx| {
            if ctx.store.next_tree.is_none() {
                ctx.store.next_tree = Some(TreeState::new(ctx.store.current_tree.version));
            };

            let slot = ctx
                .store
                .next_tree
                .as_ref()
                .unwrap()
                .selector_values
                .reserve();
            // let value = callback(&mut SelectorContext::new(ctx, slot.downgrade()));

            ctx.store
                .next_tree
                .as_mut()
                .unwrap()
                .selector_values
                .insert(slot, callback)
        })
    }
}

impl AnyStateProvider for Context {
    fn tree(&self) -> Option<&TreeState> {
        Some(&self.store.current_tree)
    }

    fn with_next_tree(&mut self) -> &TreeState {
        let next_tree = TreeState {
            version: self.store.current_tree.version + 1,
            atom_values: self.store.current_tree.atom_values.clone(),
            selector_values: self.store.current_tree.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        };

        self.store.next_tree = Some(next_tree);
        self.store.next_tree.as_ref().unwrap()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            // batcher: Batcher {
            //     pending_updates: VecDeque::new(),
            //     commit_depth: 0,
            // },
            store: StoreState::new(),
            // commit_depth: 0,
        }
    }

    pub fn sc(&mut self) -> usize {
        todo!()
    }
}

impl<'a> AnyContext for TransactionContext<'a> {
    type Result<T> = T;

    fn new_atom<T: 'static>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        todo!()
    }

    fn update_atom<T, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let mut value = self
            .ctx
            .store
            .next_tree
            .as_mut()
            .unwrap()
            .atom_values
            .begin_lease(atom);

        let result = callback(&mut value, &mut AtomContext::new(self, atom.downgrade()));

        self.ctx
            .store
            .next_tree
            .as_mut()
            .unwrap()
            .atom_values
            .end_lease(value);

        result
    }

    fn new_selector<T: 'static>(
        &mut self,
        build_selector: impl FnOnce(&mut SelectorContext<'_, T>) -> T,
    ) -> Self::Result<Selector<T>> {
        todo!()
    }
}

impl<'a> AnyStateProvider for TransactionContext<'a> {
    fn tree(&self) -> Option<&TreeState> {
        self.store.next_tree.as_ref()
    }

    fn with_next_tree(&mut self) -> &TreeState {
        let next_tree = TreeState {
            version: self.store.current_tree.version + 1,
            atom_values: self.store.current_tree.atom_values.clone(),
            selector_values: self.store.current_tree.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        };

        self.store.next_tree = Some(next_tree);
        self.store.next_tree.as_ref().unwrap()
    }
}

#[derive(Deref, DerefMut)]
pub struct TransactionContext<'a> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    dirty_atoms: FxHashSet<NodeKey>,
}

impl<'a> TransactionContext<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            dirty_atoms: FxHashSet::default(),
        }
    }

    // fn tree(&mut self) -> &mut TreeState {
    //     match &self.store.next_tree {
    //         Some(tree) => &tree,
    //         None => todo!(),
    //     }
    // }
}

// pub struct Transaction<'a, R> {
//     callback: Option<Box<dyn FnOnce(&mut TransactionContext) -> R + 'a>>,
//     commit_depth: usize,
//     // pending_updates: VecDeque<Box<dyn FnOnce(&mut Context) + 'static>>,
//     not_send: PhantomData<Rc<()>>,
// }

// impl<'a, R> Transaction<'a, R> {
//     pub fn commit(&mut self, ctx: &'a mut Context) -> R {
//         ctx.commit_transaction(self)
//     }

//     pub fn rollback(&mut self) {
//         unimplemented!()
//     }
// }

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}

impl Context {
    pub fn commit<'a, R>(&mut self, callback: impl FnOnce(&mut TransactionContext) -> R + 'a) -> R {
        let ctx = &mut TransactionContext::new(self);
        ctx.store.next_tree = Some(TreeState {
            version: ctx.store.current_tree.version + 1,
            atom_values: ctx.store.current_tree.atom_values.clone(),
            selector_values: ctx.store.current_tree.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        });

        let result = callback(ctx);

        self.swap_tree();

        result
    }

    fn swap_tree(&mut self) {
        self.store.previous_tree = Some(self.store.current_tree.clone());
        self.store.current_tree = self.store.next_tree.as_ref().unwrap().clone();
        self.store.next_tree = None
    }

    // fn commit_transaction<'a, R>(&mut self, tx: &'a mut Transaction<R>) -> R {
    //     // self.commit_depth += 1;
    //     // let callback = tx.callback.take().expect("Transaction already committed");
    //     // let result = (tx.callback)(&mut TransactionContext::new(self));
    //     // self.apply_updates();

    //     // // self.commit_depth -= 1;

    //     // result

    //     // let c = &mut TransactionContext::new(self);

    //     // loop {
    //     //     if let Some(update) = tx.pending_updates.pop_front() {
    //     //         update(c)
    //     //     } else {
    //     //         if tx.pending_updates.is_empty() {
    //     //             break;
    //     //         }
    //     //     }
    //     // }

    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[derive(Debug, Clone)]
    struct Value {
        a: usize,
    }

    #[derive(Debug, Clone)]
    struct MyString(String);

    impl AnyNodeValue for MyString {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl AnyNodeValue for Value {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn simple_test() {
        let ctx = &mut Context::new();

        let atom_a = ctx.new_atom(|_| Value { a: 0 });

        dbg!(atom_a.read(ctx));

        ctx.update_atom(&atom_a, |this, cx| {
            this.a += 10;
        });

        dbg!(atom_a.read(ctx));

        let atom_a_key = atom_a.key();

        let selector_a = ctx.new_selector(move |ctx| {
            println!("111");

            let atom_a_value = ctx.read::<Value>(&atom_a_key);

            let result = format!("Hello, {}!", atom_a_value.a);
            MyString(result)
        });

        let v = selector_a.read(ctx);
        let v = selector_a.read(ctx);
        dbg!(v);
    }
}
