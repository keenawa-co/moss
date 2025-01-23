use arcstr::ArcStr;
use hashbrown::HashMap;

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
