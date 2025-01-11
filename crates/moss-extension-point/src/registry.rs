use parking_lot::Mutex;
use std::path::PathBuf;

use crate::module::extends::ConfigurationDecl;

static __EP_REGISTRY__: Mutex<Vec<PathBuf>> = Mutex::new(vec![]);

#[macro_export]
macro_rules! submit {
    ($path:expr) => {
        #[$crate::ctor::ctor]
        fn __submit__() {
            $crate::registry::with_mut(|registry| {
                registry.push(std::path::PathBuf::from($path));
            });
        }
    };
}

pub fn take() -> Vec<PathBuf> {
    std::mem::take(&mut *__EP_REGISTRY__.lock())
}

pub fn with_mut(f: impl FnOnce(&mut Vec<PathBuf>)) {
    f(&mut __EP_REGISTRY__.lock())
}

pub struct ConfigurationRegistry {}

impl ConfigurationRegistry {
    pub fn register(&self, decl: ConfigurationDecl) {}
}

pub struct Registry {
    configurations: ConfigurationRegistry,
}
