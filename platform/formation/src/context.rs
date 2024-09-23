pub mod graph;
pub mod node;
pub mod subscription;

use derive_more::{Deref, DerefMut};
use graph::Graph;
use moss_std::collection::{FxHashMap, FxHashSet};
use node::{
    AnyNode, AnyNodeValue, Atom, AtomContext, AtomMap, NodeKey, NodeValue, Selector,
    SelectorContext, SelectorMap,
};
use once_cell::sync::OnceCell;
use smallvec::SmallVec;
use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::HashSet,
    mem,
    rc::{Rc, Weak},
    sync::Arc,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[derive(Clone)]
pub struct TreeState {
    // A unique identifier for the state version.
    version: usize,
    graph_version: Cell<usize>,
    atom_values: AtomMap,
    selector_values: SelectorMap,

    // Set of atoms that have changed.
    dirty_atoms: FxHashSet<NodeKey>,
}

impl Default for TreeState {
    fn default() -> Self {
        Self {
            version: 1,
            graph_version: Cell::new(1),
            atom_values: AtomMap::new(),
            selector_values: SelectorMap::new(),
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
    // next_tree: Option<TreeState>,
    // Used to detect nested transactions.
    // commit_depth: usize,
    // Dependency graphs for each version of the state.
    graph_by_version: RefCell<FxHashMap<usize, Graph>>,
    known_selectors: FxHashSet<NodeKey>,
}

impl StoreState {
    fn new() -> Self {
        let initial_graph = Graph::new();
        let mut graph_map = FxHashMap::default();
        graph_map.insert(1, initial_graph);

        Self {
            previous_tree: None,
            current_tree: TreeState::default(),
            graph_by_version: RefCell::new(graph_map),
            known_selectors: FxHashSet::default(),
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

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>;

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T;

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>;

    fn new_selector<T: NodeValue>(
        &mut self,
        build_selector: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>;

    fn read_selector<T: NodeValue>(&mut self, atom: &Selector<T>) -> &T;
}

pub struct Context {
    store: StoreState,
}

impl AnyContext for Context {
    type Result<T> = T;

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        self.commit(|ctx| {
            let slot = ctx.tree().atom_values.reserve();
            let value = build_atom(&mut AtomContext::new(ctx, slot.downgrade()));

            ctx.tree_mut().atom_values.insert(slot, value)
        })
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        self.store.current_tree.atom_values.read(&atom.key())
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        self.commit(|ctx| ctx.update_atom(atom, callback))
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>> {
        self.commit(|ctx| {
            let slot = ctx
                .tree()
                .selector_values
                .reserve(|map, key| Selector::new(key, callback, Arc::downgrade(&map.rc)));

            ctx.store.known_selectors.insert(slot.key());

            slot.0
        })
    }

    fn read_selector<T: NodeValue>(&mut self, selector: &Selector<T>) -> &T {
        if !self
            .store
            .current_tree
            .selector_values
            .lookup(&selector.key())
        {
            self.commit(|transaction_context| {
                let value = (&selector.compute)(&mut SelectorContext::new(
                    transaction_context,
                    selector.downgrade(),
                ));

                transaction_context
                    .tree_mut()
                    .selector_values
                    .insert(selector.key(), value);
            });
        }

        self.store
            .current_tree
            .selector_values
            .read(&selector.key())
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            store: StoreState::new(),
        }
    }

    pub fn commit<'a, R>(
        &'a mut self,
        transaction_callback: impl FnOnce(&mut TransactionContext) -> R + 'a,
    ) -> R {
        let transaction_context = &mut TransactionContext::from(self);
        let result = transaction_callback(transaction_context);

        result
    }

    pub(super) fn advance_graph<R>(&self, callback: impl FnOnce(&mut Graph) -> R) -> R {
        let mut graph_by_version = self.store.graph_by_version.borrow_mut();
        let current_graph_version = self.store.current_tree.graph_version.get();
        let current_graph = graph_by_version
            .get(&current_graph_version)
            // This indicates a bug in the context state.
            // If a correct version of the graph is received, it should always be
            // in the hash map of the graph states.
            .unwrap_or_else(|| panic!("graph version {current_graph_version} is undefined"));

        let mut new_graph = current_graph.clone();
        let result = callback(&mut new_graph);
        let new_version = current_graph_version + 1;

        graph_by_version.insert(new_version, new_graph);

        self.store.current_tree.graph_version.set(new_version);

        result
    }

    pub(super) fn read_graph<R>(&self, callback: impl FnOnce(&Graph) -> R) -> R {
        let graph_by_version = self.store.graph_by_version.borrow();
        let current_graph_version = self.store.current_tree.graph_version.get();

        let current_graph = graph_by_version
            .get(&current_graph_version)
            // This indicates a bug in the context state.
            // If a correct version of the graph is received, it should always be
            // in the hash map of the graph states.
            .unwrap_or_else(|| panic!("graph version {current_graph_version} is undefined"));

        callback(current_graph)
    }
}

impl<'a> AnyContext for TransactionContext<'a> {
    type Result<T> = T;

    fn new_atom<T: NodeValue>(
        &mut self,
        build_atom: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        todo!()
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        self.tree().atom_values.read(&atom.key())
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let mut value = self.tree_mut().atom_values.begin_lease(atom);

        let result = callback(&mut value, &mut AtomContext::new(self, atom.downgrade()));

        self.tree_mut().atom_values.end_lease(value);
        self.dirty_atoms.insert(atom.key());

        result
    }

    fn new_selector<T: NodeValue>(
        &mut self,
        build_selector: impl FnOnce(&mut SelectorContext<'_, T>) -> T,
    ) -> Self::Result<Selector<T>> {
        todo!()
    }

    fn read_selector<T: NodeValue>(&mut self, selector: &Selector<T>) -> &T {
        if !self.tree().selector_values.lookup(&selector.key()) {
            self.commit(|transaction_context| {
                let value = (&selector.compute)(&mut SelectorContext::new(
                    transaction_context,
                    selector.downgrade(),
                ));

                transaction_context
                    .tree_mut()
                    .selector_values
                    .insert(selector.key(), value);
            });
        }

        self.tree().selector_values.read(&selector.key())
    }
}

#[derive(Deref, DerefMut)]
pub struct TransactionContext<'a> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    next_tree: OnceCell<TreeState>,
    dirty_atoms: FxHashSet<NodeKey>,
}

impl<'a> Drop for TransactionContext<'a> {
    fn drop(&mut self) {
        self.invalidate();

        if let Some(next_tree) = self.next_tree.take() {
            let previous_tree = mem::replace(&mut self.ctx.store.current_tree, next_tree);
            self.ctx.store.previous_tree = Some(previous_tree);
        }
    }
}

impl<'a> From<&'a mut Context> for TransactionContext<'a> {
    fn from(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            next_tree: OnceCell::new(),
            dirty_atoms: FxHashSet::default(),
        }
    }
}

impl<'a> TransactionContext<'a> {
    fn invalidate(&mut self) {
        // Finds all nodes whose cache should be invalidated starting from the changed nodes.
        let nodes_to_invalidate = self.read_graph(|current_graph| {
            let mut nodes_to_invalidate: HashSet<NodeKey> = HashSet::new();
            let mut stack: SmallVec<[NodeKey; 8]> = SmallVec::new();
            for atom_key in &self.dirty_atoms {
                if nodes_to_invalidate.insert(atom_key.clone()) {
                    stack.push(atom_key.clone());
                }
            }

            // DFS traversal to find all dependent nodes
            while let Some(node_key) = stack.pop() {
                if let Some(subscribers) = current_graph.node_to_sub.get(&node_key) {
                    for subscriber_key in subscribers {
                        if nodes_to_invalidate.insert(subscriber_key.clone()) {
                            stack.push(subscriber_key.clone());
                        }
                    }
                }
            }

            nodes_to_invalidate
        });

        for node_key in nodes_to_invalidate {
            self.tree_mut().selector_values.remove(&node_key);
        }
    }

    pub(super) fn tree(&self) -> &TreeState {
        self.next_tree.get_or_init(|| self.advance_tree())
    }

    pub(super) fn tree_mut(&mut self) -> &mut TreeState {
        self.next_tree.get_or_init(|| self.advance_tree());
        self.next_tree.get_mut().unwrap_or_else(|| unreachable!())
    }

    fn advance_tree(&self) -> TreeState {
        let current_tree = &self.ctx.store.current_tree;

        TreeState {
            version: current_tree.version + 1,
            graph_version: Cell::clone(&current_tree.graph_version),
            atom_values: current_tree.atom_values.clone(),
            selector_values: current_tree.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        }
    }
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
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

        let atom_a_key = atom_a.key();

        let selector_a = ctx.new_selector(move |ctx| {
            println!("111");

            let atom_a_value = ctx.read::<Value>(&atom_a_key);

            let result = format!("Hello, {}!", atom_a_value.a);
            MyString(result)
        });

        let v = selector_a.read(ctx);
        dbg!(v);

        ctx.update_atom(&atom_a, |this, cx| {
            this.a += 10;
        });

        dbg!(atom_a.read(ctx));

        let v = selector_a.read(ctx);
        dbg!(v);
    }
}
