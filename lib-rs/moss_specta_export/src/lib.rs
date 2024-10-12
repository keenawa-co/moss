use anyhow::Result;
use specta::Language;
use specta_typescript::Typescript;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub type BindingFn = fn(&Typescript) -> Result<String, specta_typescript::ExportError>;

pub struct ExportGroup {
    path: &'static str,
    binding_fns: Vec<BindingFn>,
}

impl ExportGroup {
    pub fn new(path: impl Into<&'static str>, binding_fns: Vec<BindingFn>) -> Self {
        Self {
            path: path.into(),
            binding_fns,
        }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

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

pub fn create_index_file(dir: &PathBuf, modules: &[&str]) -> Result<()> {
    let mut result = String::new();

    for module_path in modules {
        result.push_str(&format!("export * from \"./{}\"; \n", module_path));
    }

    let path = dir.join("index.ts");
    let mut file = File::create(path)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}
