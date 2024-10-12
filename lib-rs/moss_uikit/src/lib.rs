pub mod component;

use anyhow::Result;
use component::{layout::*, primitive::*};

use specta::Language;
use specta_typescript::{export, Typescript};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

struct ExportGroup {
    path: &'static str,
    binding_fns: Vec<fn(&Typescript) -> Result<String, specta_typescript::ExportError>>,
}

impl ExportGroup {
    pub fn export(&self, dir: &PathBuf, conf: &Typescript) -> Result<()> {
        let path = dir.join(self.path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut result = String::with_capacity(conf.header.len() + 2);
        result.push_str(&conf.header);
        result.push_str("\n\n");

        for generator in &self.binding_fns {
            let binding = generator(conf)?;
            result.push_str(&binding);
            result.push_str("\n\n");
        }

        let mut file = File::create(&path)?;
        file.write_all(result.as_bytes())?;
        conf.format(&path)?;

        Ok(())
    }
}

pub fn export_ts_bindings(conf: &Typescript) -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let layout_group = ExportGroup {
        path: "bindings/components/layout.ts",
        binding_fns: vec![export::<Order>, export::<Alignment>, export::<Visibility>],
    };
    layout_group.export(&dir, conf)?;

    let primitive_group = ExportGroup {
        path: "bindings/components/primitive.ts",
        binding_fns: vec![
            export::<Link>,
            export::<Tooltip>,
            export::<Icon>,
            export::<Button>,
        ],
    };
    primitive_group.export(&dir, conf)?;

    Ok(create_index_file(
        &dir,
        &[layout_group.path, primitive_group.path],
    )?)
}

fn create_index_file(dir: &PathBuf, modules: &[&str]) -> Result<()> {
    let mut result = String::new();

    for module_path in modules {
        result.push_str(&format!("export * from \"./{}\"; \n", module_path));
    }

    let path = dir.join("index.ts");
    let mut file = File::create(path)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}
