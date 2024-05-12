use manifest::Manifest;
use std::path::PathBuf;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub async fn new(path: &PathBuf) -> anyhow::Result<Self> {
        let manifest = Manifest::new(&manifest::Config {
            database_path: path.join(".moss/cache").join("cache.db"),
        })
        .await?;

        Ok(Self {
            root: path.clone(),
            manifest,
        })
    }
}
