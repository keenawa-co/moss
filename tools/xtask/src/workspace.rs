use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};

pub fn load_workspace() -> Result<Metadata> {
    MetadataCommand::new()
        .exec()
        .context("failed to load cargo metadata")
}
