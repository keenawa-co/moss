use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};
use std::convert::identity;
use std::ops::Index;
use std::sync::atomic::AtomicUsize;

use super::configuration::ConfigurationNode;
use crate::foundations::configuration::ConfigurationDecl;
use crate::foundations::token::OTHER_EXTEND_CONFIGURATION_PARENT_ID;

// TODO: Further validation logic
// e.g. Checking for duplicate parameter names between parent and child
#[derive(Debug)]
pub struct ConfigurationSet {
    pub graph: petgraph::Graph<ConfigurationNode, ()>,
    pub index_map: HashMap<ArcStr, petgraph::graph::NodeIndex>,
    pub other_extend_idx: AtomicUsize,
}

impl ConfigurationSet {
    pub fn new() -> Self {
        Self {
            graph: petgraph::Graph::new(),
            index_map: HashMap::new(),
            other_extend_idx: AtomicUsize::new(0),
        }
    }

    pub fn add_node(&mut self, node: ConfigurationNode) -> Result<()> {
        if self.index_map.contains_key(&node.ident) {
            return Err(anyhow!("Duplicate configuration ident: {}", node.ident));
        }
        let idx = self.graph.add_node(node);
        let ident = self.graph[idx].ident.clone();
        self.index_map.insert(ident, idx);
        Ok(())
    }

    pub fn extend_other(&mut self, node: ConfigurationNode) -> Result<()> {
        let child_id = ArcStr::from(format!(
            "{}::{}",
            OTHER_EXTEND_CONFIGURATION_PARENT_ID,
            self.other_extend_idx
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        ));
        let node = ConfigurationNode {
            ident: child_id.clone(),
            ..node
        };
        self.add_node(node)?;
        self.add_edge(
            &ArcStr::from(OTHER_EXTEND_CONFIGURATION_PARENT_ID),
            &child_id,
        )
    }

    pub fn add_edge(&mut self, from: &ArcStr, to: &ArcStr) -> Result<()> {
        println!("{:?}", self.index_map);
        if !self.index_map.contains_key(from) {
            return Err(anyhow!("Parent `{}` does not exist", from));
        } else if !self.index_map.contains_key(to) {
            return Err(anyhow!("Child `{}` does not exist", to));
        }
        let (&from_idx, &to_idx) = (
            self.index_map.get(from).unwrap(),
            self.index_map.get(to).unwrap(),
        );
        self.graph.add_edge(from_idx, to_idx, ());
        Ok(())
    }

    pub fn get_parents(&self, node_ident: &ArcStr) -> Vec<&ConfigurationNode> {
        if let Some(&idx) = self.index_map.get(node_ident) {
            let mut parents = Vec::new();
            for neighbor in self
                .graph
                .neighbors_directed(idx, petgraph::Direction::Incoming)
            {
                parents.push(&self.graph[neighbor]);
            }
            parents
        } else {
            Vec::new()
        }
    }

    pub fn get_children(&self, node_ident: &ArcStr) -> Vec<&ConfigurationNode> {
        if let Some(&idx) = self.index_map.get(node_ident) {
            let mut children = Vec::new();
            for neighbor in self
                .graph
                .neighbors_directed(idx, petgraph::Direction::Outgoing)
            {
                children.push(&self.graph[neighbor]);
            }
            children
        } else {
            Vec::new()
        }
    }

    pub fn into_values(self) -> Vec<ConfigurationNode> {
        petgraph::algo::toposort(&self.graph, None)
            .expect("Cycles detected")
            .iter()
            .rev()
            .map(|&i| self.graph.index(i).clone())
            .collect::<Vec<ConfigurationNode>>()
    }
}

#[derive(Debug)]
pub struct ResolvedScope {
    pub configurations: ConfigurationSet,
}

impl ResolvedScope {
    pub fn new() -> Self {
        Self {
            configurations: ConfigurationSet::new(),
        }
    }

    pub fn insert_configuration(&mut self, node: ConfigurationNode) -> Result<()> {
        self.configurations.add_node(node)
    }

    pub fn extend_configuration(&mut self, node: ConfigurationNode) -> Result<()> {
        // TODO: Add validation logic for adding child nodes
        let parent_id = node.parent_ident.clone().unwrap();
        let child_id = node.ident.clone();
        // Handling anonymous extends for `other` node
        if child_id == "" {
            self.configurations.extend_other(node)?;
            Ok(())
        } else {
            self.configurations.add_node(node)?;
            self.configurations.add_edge(&parent_id, &child_id)
        }
    }
}

#[derive(Debug)]
pub struct ScopeRepr {
    pub configurations: Vec<ConfigurationDecl>,
    pub configuration_extends: Vec<ConfigurationDecl>,
    pub locals: Map<String, HclValue>,
}

impl ScopeRepr {
    pub fn new() -> Self {
        Self {
            configurations: Vec::new(),
            configuration_extends: Vec::new(),
            locals: Map::new(),
        }
    }

    pub fn evaluate_with_context(self, ctx: &Context) -> Result<ResolvedScope> {
        let mut ctx = ctx.clone();
        let mut package = ResolvedScope::new();
        ctx.declare_var("local", hcl::Value::Object(self.locals));
        package.insert_configuration(ConfigurationNode {
            ident: OTHER_EXTEND_CONFIGURATION_PARENT_ID.into(),
            parent_ident: None,
            display_name: None,
            description: None,
            order: None,
            parameters: Default::default(),
            overrides: Default::default(),
        })?;
        for decl in self.configurations {
            package.insert_configuration(decl.evaluate(&ctx)?)?
        }

        for decl in self.configuration_extends {
            package.extend_configuration(decl.evaluate(&ctx)?)?
        }

        Ok(package)
    }

    pub fn evaluate(self) -> Result<ResolvedScope> {
        self.evaluate_with_context(&Context::default())
    }
}
