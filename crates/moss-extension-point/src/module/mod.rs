pub mod extends;

use extends::ExtendsDecl;
use hashbrown::HashSet;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionPointFile {
    extends: ExtendsDecl,
}

pub struct ExtensionPointModule {
    pub extends: Vec<ExtendsDecl>,
    pub known_files: HashSet<PathBuf>,
}

impl ExtensionPointModule {
    pub fn new() -> Self {
        Self {
            extends: Vec::new(),
            known_files: HashSet::new(),
        }
    }

    pub fn register_file(&mut self, path: PathBuf, file: ExtensionPointFile) -> bool {
        if !self.known_files.insert(path) {
            return false;
        }

        self.extends.push(file.extends);
        true
    }
}
