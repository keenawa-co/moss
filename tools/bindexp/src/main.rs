use anyhow::Result;
use specta_typescript::BigIntExportBehavior;

fn main() -> Result<()> {
    moss_uikit::export_ts_bindings(
        &specta_typescript::Typescript::default()
            .formatter(specta_typescript::formatter::prettier)
            .header("/* eslint-disable */")
            .bigint(BigIntExportBehavior::Number),
    )?;

    Ok(())
}
