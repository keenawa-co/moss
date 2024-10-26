use crate::base::collection::{ImHashMap, ImHashSet};

use super::node::NodeKey;

#[derive(Clone)]
pub struct Graph {
    pub(super) node_to_dep: ImHashMap<NodeKey, ImHashSet<NodeKey>>,
    pub(super) node_to_sub: ImHashMap<NodeKey, ImHashSet<NodeKey>>,
}

impl Graph {
    pub(super) fn new() -> Self {
        Self {
            node_to_dep: ImHashMap::new(),
            node_to_sub: ImHashMap::new(),
        }
    }

    pub(super) fn has_subscription(&self, from: &NodeKey, to: &NodeKey) -> bool {
        self.node_to_dep
            .get(from)
            .map_or(false, |deps| deps.contains(to))
    }

    pub(super) fn create_dependency(&mut self, from: NodeKey, to: NodeKey) {
        self.node_to_dep
            .entry(from)
            .or_insert_with(ImHashSet::new)
            .insert(to);

        self.node_to_sub
            .entry(to)
            .or_insert_with(ImHashSet::new)
            .insert(from);
    }
}
