pub mod atom;
pub mod atom_context;
pub mod node;
pub mod selector;
pub mod selector_context;
pub mod subscription;
pub mod transaction_context;

mod common;
mod graph;

use atom::{Atom, AtomMap};
use atom_context::AtomContext;
use graph::Graph;
use moss_std::collection::{FxHashMap, FxHashSet};
use node::{AnyNode, NodeKey, NodeValue};
use once_cell::sync::OnceCell;
use platform_core::context::EventEmitter;
use selector::{Selector, SelectorMap};
use selector_context::SelectorContext;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    mem,
    sync::Arc,
};
use subscription::{SubscriberSet, Subscription};
use transaction_context::TransactionContext;

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
    next_tree: OnceCell<TreeState>,

    node_observers: SubscriberSet<NodeKey, Handler>,
    event_listeners: SubscriberSet<NodeKey, (TypeId, Listener)>,

    // Dependency graphs for each version of the state.
    graph_by_version: RefCell<FxHashMap<usize, Graph>>,
    known_selectors: FxHashSet<NodeKey>,
}

impl StoreState {
    fn new() -> Self {
        let initial_graph_map = {
            let initial_graph = Graph::new();
            let mut graph_map = FxHashMap::default();
            graph_map.insert(1, initial_graph);

            graph_map
        };

        Self {
            previous_tree: None,
            current_tree: TreeState::default(),
            next_tree: OnceCell::new(),
            node_observers: SubscriberSet::new(),
            event_listeners: SubscriberSet::new(),
            graph_by_version: RefCell::new(initial_graph_map),
            known_selectors: FxHashSet::default(),
        }
    }
}

pub trait AnyContext {
    type Result<T>;

    fn create_atom<T: NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>>;

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T;

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Result<R>;

    fn new_selector<T: NodeValue>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Result<Selector<T>>;

    fn read_selector<T: NodeValue>(&mut self, atom: &Selector<T>) -> &T;
}

pub(super) trait NonTransactableContext {
    fn as_mut(&mut self) -> &mut Context;
    fn as_ref(&self) -> &Context;
}

pub enum Effect {
    Notify {
        emitter: NodeKey,
    },
    Event {
        emitter: NodeKey,
        typ: TypeId,
        payload: Box<dyn Any>,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut Context) + 'static>,
    },
}

type Handler = Box<dyn FnMut(&mut Context) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut Context) -> bool + 'static>;

pub struct Context {
    store: StoreState,
    batcher: Batcher,
}

impl NonTransactableContext for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }

    fn as_ref(&self) -> &Context {
        self
    }
}

impl AnyContext for Context {
    type Result<T> = T;

    fn create_atom<T: NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Result<Atom<T>> {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<T: NodeValue>(&self, atom: &Atom<T>) -> &T {
        common::read_atom::<Self, _>(self, atom)
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
        common::resolve_selector::<Self, _>(self, selector)
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            store: StoreState::new(),
            batcher: Batcher::new(),
        }
    }

    pub fn stage<'a, R>(
        &'a mut self,
        transaction_callback: impl FnOnce(&mut TransactionContext) -> R + 'a,
    ) -> R {
        let transaction_context = &mut TransactionContext::from(self);
        let result = transaction_callback(transaction_context);

        result
    }

    pub(super) fn next_tree(&self) -> &TreeState {
        self.store.next_tree.get_or_init(|| self.advance_tree())
    }

    pub(super) fn next_tree_mut(&mut self) -> &mut TreeState {
        self.store.next_tree.get_or_init(|| self.advance_tree());
        self.store
            .next_tree
            .get_mut()
            .unwrap_or_else(|| unreachable!())
    }

    fn advance_tree(&self) -> TreeState {
        let current_tree = &self.store.current_tree;

        TreeState {
            version: current_tree.version + 1,
            graph_version: Cell::clone(&current_tree.graph_version),
            atom_values: current_tree.atom_values.clone(),
            selector_values: current_tree.selector_values.clone(),
            dirty_atoms: FxHashSet::default(),
        }
    }

    fn commit(&mut self) {
        let mut next_tree = if let Some(next_tree) = self.store.next_tree.take() {
            next_tree
        } else {
            return;
        };

        self.invalidate(&mut next_tree);

        let previous_tree = mem::replace(&mut self.store.current_tree, next_tree);
        self.store.previous_tree = Some(previous_tree);

        self.flush_effects();
    }

    fn invalidate(&mut self, next_tree: &mut TreeState) {
        // Finds all nodes whose cache should be invalidated starting from the changed nodes.
        let nodes_to_invalidate = self.read_graph(|current_graph| {
            let mut nodes_to_invalidate: HashSet<NodeKey> = HashSet::new();
            let mut stack: SmallVec<[NodeKey; 8]> = SmallVec::new();
            for atom_key in next_tree.dirty_atoms.iter() {
                if current_graph.node_to_sub.get(&atom_key).is_some() {
                    if nodes_to_invalidate.insert(atom_key.clone()) {
                        stack.push(atom_key.clone());
                    }
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
            next_tree.selector_values.remove(&node_key);
        }
    }

    fn flush_effects(&mut self) {
        // TODO: release dropped nodes

        loop {
            if let Some(effect) = self.batcher.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => self.apply_notify_effect(emitter),
                    Effect::Event {
                        emitter,
                        typ: payload_typ,
                        payload,
                    } => self.apply_event_effect(emitter, payload_typ, payload),
                    Effect::Defer { callback } => self.apply_defer_effect(callback),
                }
            } else {
                if self.batcher.pending_effects.is_empty() {
                    break;
                }
            }
        }
    }

    fn apply_notify_effect(&mut self, emitter: NodeKey) {
        self.batcher.pending_notifications.remove(&emitter);

        self.store
            .node_observers
            .clone()
            .retain(&emitter, |handler| handler(self));
    }

    fn apply_event_effect(&mut self, emitter: NodeKey, payload_typ: TypeId, payload: Box<dyn Any>) {
        self.store
            .event_listeners
            .clone()
            .retain(&emitter, |(stored_typ, handler)| {
                if *stored_typ == payload_typ {
                    handler(payload.as_ref(), self)
                } else {
                    true
                }
            });
    }

    fn apply_defer_effect(&mut self, callback: Box<dyn FnOnce(&mut Context) + 'static>) {
        callback(self);
    }

    pub(crate) fn subscribe_internal<V, N, T>(
        &mut self,
        node: &N,
        mut on_event: impl FnMut(N, &T, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        V: EventEmitter<T>,
        N: AnyNode<V>,
        T: 'static,
    {
        let node_key = node.key();
        let weak_node = node.downgrade();

        self.new_subscription(
            node_key,
            (
                TypeId::of::<T>(),
                Box::new(move |payload, ctx| {
                    let payload: &T = payload.downcast_ref().expect("invalid event payload type");
                    if let Some(handle) = N::upgrade_from(&weak_node) {
                        on_event(handle, payload, ctx)
                    } else {
                        false
                    }
                }),
            ),
        )
    }

    fn new_subscription(&mut self, key: NodeKey, value: (TypeId, Listener)) -> Subscription {
        let (subscription, activate) = self.store.event_listeners.insert(key, value);
        self.defer(move |_| activate());

        subscription
    }

    pub fn observe<V, N>(
        &mut self,
        node: &N,
        mut on_notify: impl FnMut(N, &mut Context) + 'static,
    ) -> Subscription
    where
        V: 'static,
        N: AnyNode<V>,
    {
        self.observe_internal(node, move |n, ctx| {
            on_notify(n, ctx);
            true
        })
    }

    fn observe_internal<V, N>(
        &mut self,
        node: &N,
        mut on_notify: impl FnMut(N, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        V: 'static,
        N: AnyNode<V>,
    {
        let handle = node.downgrade();
        self.new_observer(
            node.key(),
            Box::new(move |ctx| {
                if let Some(n) = N::upgrade_from(&handle) {
                    on_notify(n, ctx)
                } else {
                    false
                }
            }),
        )
    }

    fn new_observer(&mut self, key: NodeKey, handler: Handler) -> Subscription {
        let (subscription, activate) = self.store.node_observers.insert(key, handler);
        self.defer(move |_| activate());

        subscription
    }

    pub fn defer(&mut self, f: impl FnOnce(&mut Context) + 'static) {
        self.push_effect(Effect::Defer {
            callback: Box::new(f),
        });
    }

    pub fn push_effect(&mut self, effect: Effect) {
        self.batcher.pending_effects.push_back(effect);
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

struct Batcher {
    // Used to detect nested transactions.
    commit_depth: Cell<usize>,
    pending_effects: VecDeque<Effect>,
    pending_notifications: FxHashSet<NodeKey>,
}

impl Batcher {
    fn new() -> Self {
        Self {
            commit_depth: Cell::new(0),
            pending_effects: VecDeque::new(),
            pending_notifications: FxHashSet::default(),
        }
    }

    fn inc_commit_depth(&self) -> usize {
        let current_version = self.commit_depth.get();
        let new_version = current_version + 1;
        self.commit_depth.set(new_version);

        new_version
    }

    fn dec_commit_depth(&self) -> usize {
        let current_version = self.commit_depth.get();
        let new_version = current_version - 1;
        self.commit_depth.set(new_version);

        new_version
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use atom::OnChangeAtomEvent;
    use node::AnyNodeValue;

    use super::*;

    #[derive(Debug, Clone)]
    struct Value {
        a: usize,
    }

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug)]
    struct Change {
        b: usize,
    }

    impl EventEmitter<Change> for Value {}

    impl EventEmitter<OnChangeAtomEvent> for Value {}

    #[test]
    fn subscription_on_atom_change_test() {
        let ctx = &mut Context::new();
        let atom_a = ctx.create_atom(|_| Value { a: 0 });

        let atom_b = ctx.create_atom(|ctx| {
            ctx.subscribe(
                &atom_a,
                |atom_b_inner: &mut Value, value_a, event: &OnChangeAtomEvent, _cx| {
                    println!("Hello, form atom subscription");
                },
            )
            .detach();

            Value { a: 0 }
        });

        ctx.update_atom(&atom_a, |this, cx| {
            this.a += 10;

            cx.notify();
            cx.emit(OnChangeAtomEvent {});
        });

        debug_assert_eq!(atom_a.read(ctx).a, 10);
        debug_assert_eq!(atom_b.read(ctx).a, 0);
    }

    #[test]
    fn subscription_test() {
        let ctx = &mut Context::new();
        let atom_a = ctx.create_atom(|_| Value { a: 0 });

        let atom_b = ctx.create_atom(|ctx| {
            ctx.subscribe(
                &atom_a,
                |atom_b_inner: &mut Value, value_a, event: &Change, _cx| {
                    println!("Hello, form atom subscription");

                    atom_b_inner.a = event.b;
                },
            )
            .detach();

            Value { a: 0 }
        });

        ctx.update_atom(&atom_a, |this, cx| {
            this.a += 10;

            cx.notify();
            cx.emit(Change { b: this.a });
        });

        debug_assert_eq!(atom_a.read(ctx).a, 10);
        debug_assert_eq!(atom_b.read(ctx).a, 10);
    }

    #[test]
    fn observe_test() {
        let ctx = &mut Context::new();
        let atom_a = ctx.create_atom(|_| Value { a: 0 });

        let _subscription = ctx.observe(&atom_a, move |this, atom_context| {
            let this_a_read_result = this.read(atom_context).a;
            debug_assert_eq!(this_a_read_result, 10);

            println!("Hello, form atom observe, this value: {this_a_read_result}",);
        });

        ctx.update_atom(&atom_a, |this, cx| {
            this.a += 10;

            cx.notify();
        });

        dbg!(atom_a.read(ctx));
    }

    #[test]
    fn simple_test() {
        let ctx = &mut Context::new();

        let atom_a = ctx.create_atom(|_| Value { a: 0 });

        ctx.update_atom(&atom_a, |this, atom_context| {
            this.a += 10;
        });

        let atom_a_key = atom_a.key();

        let selector_a = ctx.new_selector(move |selector_context| {
            let atom_a_value = selector_context.read::<Value>(&atom_a_key);

            let result = format!("Hello, {}!", atom_a_value.a);
            MyString(result)
        });

        let selector_a_result = selector_a.read(ctx);
        debug_assert_eq!(selector_a_result, &MyString("Hello, 10!".to_string()));
        dbg!(selector_a_result);

        ctx.update_atom(&atom_a, |this, atom_context| {
            this.a += 10;

            atom_context.notify();
        });

        dbg!(atom_a.read(ctx));

        let selector_a_result = selector_a.read(ctx);
        debug_assert_eq!(selector_a_result, &MyString("Hello, 20!".to_string()));
        dbg!(selector_a_result);
    }
}
