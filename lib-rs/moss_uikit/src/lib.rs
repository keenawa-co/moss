pub mod component;

use anyhow::Result;
use component::{layout::*, primitive::*};

use moss_specta_export::ExportGroup;
use specta_typescript::{export, Typescript};
use std::path::PathBuf;

pub fn export_ts_bindings(conf: &Typescript) -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let layout_group = ExportGroup::new(
        "bindings/components/layout.ts",
        vec![export::<Order>, export::<Alignment>, export::<Visibility>],
    );
    layout_group.export(&dir, conf)?;

    let primitive_group = ExportGroup::new(
        "bindings/components/primitive.ts",
        vec![
            export::<Link>,
            export::<Tooltip>,
            export::<Icon>,
            export::<Button>,
        ],
    );
    primitive_group.export(&dir, conf)?;

    Ok(moss_specta_export::create_index_file(
        &dir,
        &[layout_group.path(), primitive_group.path()],
    )?)
}
