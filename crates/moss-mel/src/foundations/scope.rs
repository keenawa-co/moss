use anyhow::Result;
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};

use crate::foundations::configuration::ConfigurationDecl;

use super::configuration::ConfigurationNode;

#[derive(Debug)]
pub struct ResolvedScope {
    pub configurations: HashMap<ArcStr, ConfigurationNode>,
}

impl ResolvedScope {
    pub fn new() -> Self {
        Self {
            configurations: HashMap::new(),
        }
    }

    pub fn insert_configuration(&mut self, node: ConfigurationNode) {
        self.configurations.insert(ArcStr::clone(&node.ident), node);
    }

    pub fn extend_configuration(&mut self, node: ConfigurationNode) {
        unimplemented!()
    }
}

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

        for decl in self.configurations {
            package.insert_configuration(decl.evaluate(&ctx)?);
        }

        Ok(package)
    }

    pub fn evaluate(self) -> Result<ResolvedScope> {
        self.evaluate_with_context(&Context::default())
    }
}
