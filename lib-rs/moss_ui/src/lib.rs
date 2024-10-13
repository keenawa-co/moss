pub mod parts;

use anyhow::Result;
use moss_specta::ExportGroup;
use specta_typescript::{export, Typescript};
use std::path::PathBuf;

use parts::{common::*, toolbar::*};

pub fn export_ts_bindings(conf: &Typescript) -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let common_group = ExportGroup::new(
        "bindings/parts/common/types.ts",
        vec![export::<ContextMenuCell>, export::<ContextMenu>],
    );
    common_group.export(&dir, conf)?;

    let toolbar_group = ExportGroup::new(
        "bindings/parts/toolbar/types.ts",
        vec![
            export::<ToolBarProjectCell>,
            export::<ActivityCell>,
            export::<ToolBarLeftSide>,
            export::<DescribeToolBarOutput>,
        ],
    );
    toolbar_group.export(&dir, conf)?;

    Ok(moss_specta::create_index_file(
        &dir,
        &[common_group.path(), toolbar_group.path()],
    )?)
}
