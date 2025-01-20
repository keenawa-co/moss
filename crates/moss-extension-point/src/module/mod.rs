pub mod configuration;

use arcstr::ArcStr;
use configuration::ConfigurationDecl;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionPointFile {
    #[serde(rename = "configuration")]
    configurations: Option<HashMap<ArcStr, Arc<ConfigurationDecl>>>,
}

pub struct ExtensionPointModule {
    pub configurations: HashMap<ArcStr, Arc<ConfigurationDecl>>,
    pub known_files: HashSet<PathBuf>,
}

impl ExtensionPointModule {
    pub fn new() -> Self {
        Self {
            configurations: HashMap::new(),
            known_files: HashSet::new(),
        }
    }

    pub fn register_file(&mut self, path: PathBuf, file: ExtensionPointFile) -> bool {
        if !self.known_files.insert(path) {
            return false;
        }

        if let Some(configuration) = file.configurations {
            self.configurations.extend(configuration);
        }

        true
    }
}
