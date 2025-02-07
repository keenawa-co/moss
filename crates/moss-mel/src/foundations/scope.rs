use super::configuration::ConfigurationNode;
use crate::eval::evaluate_locals;
use crate::foundations::configuration::ConfigurationDecl;
use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::Value::Object;
use hcl::{eval::Context, Expression};

#[derive(Debug)]
pub struct ConfigurationSet {
    pub named_configs: HashMap<String, ConfigurationNode>,
    pub anonymous_extends: Vec<ConfigurationNode>,
}

impl ConfigurationSet {
    pub fn new() -> Self {
        ConfigurationSet {
            named_configs: Default::default(),
            anonymous_extends: vec![],
        }
    }

    pub fn merge(&mut self, other: ConfigurationSet) {
        self.named_configs.extend(other.named_configs);
        self.anonymous_extends.extend(other.anonymous_extends);
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

    pub fn merge(&mut self, other: ResolvedScope) {
        self.configurations.merge(other.configurations);
    }

    pub fn get_configuration(&self, name: &str) -> Option<&ConfigurationNode> {
        self.configurations.named_configs.get(name)
    }

    pub fn insert_configuration(&mut self, name: &str, configuration: ConfigurationNode) {
        self.configurations
            .named_configs
            .insert(name.to_string(), configuration);
    }

    pub fn insert_anonymous_extends(&mut self, configuration: ConfigurationNode) {
        self.configurations.anonymous_extends.push(configuration);
    }

    pub fn into_values(self) -> Vec<ConfigurationNode> {
        self.configurations
            .named_configs
            .into_values()
            .chain(self.configurations.anonymous_extends.into_iter())
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
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

    fn resolve_extend_order(&self) -> Result<Vec<ConfigurationDecl>> {
        let mut named_confs = HashMap::new();
        let mut genesis = Vec::new();
        let mut successor = Vec::new();

        for conf in self.configurations.iter() {
            match conf {
                ConfigurationDecl::Genesis { ref ident, .. } => {
                    if named_confs.contains_key(ident) {
                        return Err(anyhow!("Duplicte configuration ident `{}`", ident));
                    }
                    genesis.push(ident.clone());
                    named_confs.insert(ident.clone(), conf.clone());
                }
                ConfigurationDecl::Successor { ref ident, .. } => {
                    if named_confs.contains_key(ident) {
                        return Err(anyhow!("Duplicte configuration ident `{}`", ident));
                    }
                    successor.push(ident.clone());
                    named_confs.insert(ident.clone(), conf.clone());
                }
                _ => {}
            }
        }

        let mut extend_graph = petgraph::Graph::<ArcStr, ()>::new();
        let mut node_map = HashMap::new();
        let mut name_map = HashMap::new();

        for ident in named_confs.keys() {
            let idx = extend_graph.add_node(ident.clone());
            node_map.insert(ident.clone(), idx);
            name_map.insert(idx, ident.clone());
        }

        for ident in successor {
            let parent_ident = named_confs[&ident].parent_ident().unwrap();
            let from_idx = node_map[&ident];

            if let Some(&to_idx) = node_map.get(&parent_ident) {
                println!("{} depends on {}", ident, parent_ident);
                extend_graph.add_edge(from_idx, to_idx, ());
            } else {
                return Err(anyhow!("Cannot find configuration `{}`", parent_ident));
            }
        }

        Ok(petgraph::algo::toposort(&extend_graph, None)
            .map_err(|_| anyhow!("Cycle detected in extends"))?
            .into_iter()
            .rev()
            .map(|idx| name_map.get(&idx).unwrap())
            .map(|name| named_confs.get(name).unwrap().to_owned())
            .collect::<Vec<_>>())
    }

    pub fn collect_dependencies(&self) -> Option<Vec<ArcStr>> {
        None
        // TODO: Implement it after we have module import mechanism
    }

    pub fn evaluate_with_context(self, global_ctx: &mut Context) -> Result<ResolvedScope> {
        let mut result = ResolvedScope::new();
        let evaluated_locals = evaluate_locals(self.locals.clone())?;
        let mut module_ctx = global_ctx.clone();
        module_ctx.declare_var("local", Object(evaluated_locals));
        let resolution_queue = self.resolve_extend_order()?;
        let anonymous_extends = self
            .configurations
            .iter()
            .filter(|decl| decl.ident().is_some())
            .map(|decl| decl.to_owned())
            .collect::<Vec<_>>();

        for decl in resolution_queue {
            let evaluated = decl.evaluate(&module_ctx)?;
            result.insert_configuration(evaluated.ident.clone().as_str(), evaluated);
            // TODO: update the module/package context based on newly evaluated ConfigurationNode
        }

        for decl in anonymous_extends {
            let evaluated = decl.evaluate(&module_ctx)?;
            result.insert_anonymous_extends(evaluated);
        }
        Ok(result)
    }
}
