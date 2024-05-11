pub mod cache;
pub mod ignored;

use cache::CacheAdapter;
use hashbrown::HashSet;
use std::{path::PathBuf, sync::Arc};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub ignored_list: Arc<HashSet<String>>,
    pub cache: Arc<dyn CacheAdapter>,
}
