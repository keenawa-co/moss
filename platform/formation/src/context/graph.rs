use moss_std::collection::{FxHashMap, FxHashSet};

use super::node::NodeKey;

pub struct Graph {
    pub(super) node_to_dep: FxHashMap<NodeKey, FxHashSet<NodeKey>>,
    pub(super) node_to_sub: FxHashMap<NodeKey, FxHashSet<NodeKey>>,
}

impl Graph {
    pub(super) fn new() -> Self {
        Self {
            node_to_dep: FxHashMap::default(),
            node_to_sub: FxHashMap::default(),
        }
    }

    pub(super) fn create_dependency(&mut self, from: NodeKey, to: NodeKey) {
        self.node_to_dep
            .entry(from)
            .or_insert_with(FxHashSet::default)
            .insert(to);

        self.node_to_sub
            .entry(to)
            .or_insert_with(FxHashSet::default)
            .insert(from);
    }

    pub(super) fn get_all_subscribers(&self, key: &NodeKey) -> Option<&FxHashSet<NodeKey>> {
        self.node_to_sub.get(key)
    }
}
