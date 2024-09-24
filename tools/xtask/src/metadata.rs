use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};

pub fn load_cargo_metadata() -> Result<Metadata> {
    MetadataCommand::new()
        .exec()
        .context("failed to load cargo metadata")
}
