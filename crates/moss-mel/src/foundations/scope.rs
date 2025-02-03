use super::configuration::ConfigurationNode;
use crate::eval::evaluate_locals;
use crate::foundations::configuration::ConfigurationDecl;
use crate::foundations::token::OTHER_EXTEND_CONFIGURATION_PARENT_ID;
use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::Value::Object;
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};
use std::convert::identity;
use std::ops::Index;
use std::sync::atomic::AtomicUsize;

// impl ConfigurationSet {

// pub fn add_node(&mut self, node: ConfigurationNode) -> Result<()> {
//     if self.index_map.contains_key(&node.ident) {
//         return Err(anyhow!("Duplicate configuration ident: {}", node.ident));
//     }
//     let idx = self.dependency_graph.add_node(node);
//     let ident = self.dependency_graph[idx].ident.clone();
//     self.index_map.insert(ident, idx);
//     Ok(())
// }
//
// pub fn extend_other(&mut self, node: ConfigurationNode) -> Result<()> {
//     let child_id = ArcStr::from(format!(
//         "{}::{}",
//         OTHER_EXTEND_CONFIGURATION_PARENT_ID,
//         self.other_extend_idx
//             .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
//     ));
//     let node = ConfigurationNode {
//         ident: child_id.clone(),
//         ..node
//     };
//     self.add_node(node)?;
//     self.add_edge(
//         &ArcStr::from(OTHER_EXTEND_CONFIGURATION_PARENT_ID),
//         &child_id,
//     )
// }
//
// pub fn add_edge(&mut self, from: &ArcStr, to: &ArcStr) -> Result<()> {
//     println!("{:?}", self.index_map);
//     if !self.index_map.contains_key(from) {
//         return Err(anyhow!("Parent `{}` does not exist", from));
//     } else if !self.index_map.contains_key(to) {
//         return Err(anyhow!("Child `{}` does not exist", to));
//     }
//     let (&from_idx, &to_idx) = (
//         self.index_map.get(from).unwrap(),
//         self.index_map.get(to).unwrap(),
//     );
//     self.dependency_graph.add_edge(from_idx, to_idx, ());
//     Ok(())
// }
//
// pub fn get_parents(&self, node_ident: &ArcStr) -> Vec<&ConfigurationNode> {
//     if let Some(&idx) = self.index_map.get(node_ident) {
//         let mut parents = Vec::new();
//         for neighbor in self
//             .dependency_graph
//             .neighbors_directed(idx, petgraph::Direction::Incoming)
//         {
//             parents.push(&self.dependency_graph[neighbor]);
//         }
//         parents
//     } else {
//         Vec::new()
//     }
// }
//
// pub fn get_children(&self, node_ident: &ArcStr) -> Vec<&ConfigurationNode> {
//     if let Some(&idx) = self.index_map.get(node_ident) {
//         let mut children = Vec::new();
//         for neighbor in self
//             .dependency_graph
//             .neighbors_directed(idx, petgraph::Direction::Outgoing)
//         {
//             children.push(&self.dependency_graph[neighbor]);
//         }
//         children
//     } else {
//         Vec::new()
//     }
// }
//
// }

#[derive(Debug)]
pub struct ConfigurationSet {
    pub named_configs: HashMap<String, ConfigurationNode>,
    pub anonymous_extends: Vec<ConfigurationNode>,
}

#[derive(Debug)]
pub struct ResolvedScope {
    pub configuration_set: ConfigurationSet,
}

impl ResolvedScope {
    pub fn new() -> Self {
        Self {
            configuration_set: ConfigurationSet {
                named_configs: Default::default(),
                anonymous_extends: vec![],
            },
        }
    }
    pub fn get_configuration(&self, name: &str) -> Option<&ConfigurationNode> {
        self.configuration_set.named_configs.get(name)
    }

    pub fn insert_configuration(&mut self, name: &str, configuration: ConfigurationNode) {
        self.configuration_set
            .named_configs
            .insert(name.to_string(), configuration);
    }

    pub fn insert_anonymous_extends(&mut self, configuration: ConfigurationNode) {
        self.configuration_set.anonymous_extends.push(configuration);
    }

    // pub fn into_values(self) -> Vec<ConfigurationNode> {
    //     petgraph::algo::toposort(&self.dependency_graph, None)
    //         .expect("Cycles detected")
    //         .iter()
    //         .rev()
    //         .map(|&i| self.dependency_graph.index(i).clone())
    //         .collect::<Vec<ConfigurationNode>>()
    // }

    pub fn into_values(self) -> Vec<ConfigurationNode> {
        self.configuration_set
            .named_configs
            .into_values()
            .chain(self.configuration_set.anonymous_extends.into_iter())
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct ModuleScope {
    pub configurations: Vec<ConfigurationDecl>,
    pub locals: HashMap<String, Expression>,
}

impl ModuleScope {
    pub fn new() -> Self {
        Self {
            configurations: Vec::new(),
            locals: Default::default(),
        }
    }

    pub fn generate_ctx(&self) -> Result<Context> {
        let mut ctx = Context::new();
        let evaluated_locals = evaluate_locals(self.locals.clone())?;
        ctx.declare_var("local", Object(evaluated_locals));
        Ok(ctx)
    }
}
