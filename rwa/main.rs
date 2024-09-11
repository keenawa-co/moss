use cargo_metadata::{MetadataCommand, Package};
use std::collections::HashSet;

fn main() {
    let crate_names = get_workspace_crate_names();
    println!("Workspace crate names:");
    for name in crate_names {
        println!("- {}", name);
    }
}

fn get_workspace_crate_names() -> HashSet<String> {
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to get cargo metadata");

    metadata
        .packages
        .into_iter()
        .map(|package: Package| package.name)
        .collect()
}
