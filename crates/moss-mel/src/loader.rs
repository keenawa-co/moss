use crate::foundations::configuration::ConfigurationDecl;
use crate::foundations::scope::{ModuleScope, ResolvedScope};
use crate::parse::parse_module_file;
use anyhow::Result;
use hashbrown::HashMap;
use hcl::eval::{Context as EvalContext, Context};
use std::ffi::OsStr;
use std::path::PathBuf;

const FILE_EXTENSION: &'static str = "hcl";

// TODO: graph at the module level
#[derive(Debug)]
struct Package {
    modules: HashMap<PathBuf, ModuleScope>,
}

impl Package {
    fn resolve_module_order(&self) -> Result<Vec<ModuleScope>> {
        // TODO: implement this once we have module imports and dependency
        // Right now we will just use an arbitrary order
        Ok(self.modules.values().cloned().collect::<Vec<_>>())
    }

    pub fn evaluate_with_context(self, global_ctx: &mut Context) -> Result<ResolvedScope> {
        let mut result = ResolvedScope::new();
        for module in self.resolve_module_order()? {
            result.merge(module.evaluate_with_context(global_ctx)?);
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct Loader {
    packages: HashMap<PathBuf, Package>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn load(&mut self, workspace_root: PathBuf, paths: Vec<PathBuf>) -> Result<()> {
        // FIXME: clarify the loading of packages and modules
        // What does `paths` refer to here? The paths of all modules of a package?
        // Here I assume each path refers to one package
        for path in paths {
            let package_path = &workspace_root.join(&path);
            let package = self.load_package(package_path)?;
            self.packages.insert(package_path.to_owned(), package);
        }

        Ok(())
    }

    fn load_package(&self, path: &PathBuf) -> Result<Package> {
        let read_dir = std::fs::read_dir(path)?;
        let mut package = Package {
            modules: Default::default(),
        };
        let mut top_module = ModuleScope::new();
        let mut ctx = EvalContext::new();
        for entry in read_dir {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                package
                    .modules
                    .insert(entry_path.clone(), self.load_module(&mut ctx, &entry_path)?);
            } else if entry_path.extension() == Some(OsStr::new(FILE_EXTENSION)) {
                self.load_module_file(&mut top_module, &mut ctx, &entry_path)?
            }
        }

        package.modules.insert(path.clone(), top_module);

        Ok(package)
    }

    fn load_module(&self, ctx: &mut EvalContext, path: &PathBuf) -> Result<ModuleScope> {
        let read_dir = std::fs::read_dir(path)?;
        let mut module_scope = ModuleScope::new();

        for entry in read_dir {
            let path = entry?.path();

            // TODO: Implement a recursive traversal of all nested folders.
            // Currently, we only support files located at the top level.
            if path.is_dir() {
                continue;
            }

            if let Some(extension) = path.extension() {
                if extension != FILE_EXTENSION {
                    continue;
                }

                self.load_module_file(&mut module_scope, ctx, &path)?;
            }
        }

        Ok(module_scope)
    }

    fn load_module_file(
        &self,
        module_scope: &mut ModuleScope,
        _ctx: &mut EvalContext,
        path: &PathBuf,
    ) -> Result<()> {
        let file_content = std::fs::read_to_string(path)?;
        parse_module_file(&file_content, module_scope)
    }

    pub fn resolve(self) -> Vec<Result<ResolvedScope>> {
        self.packages
            .into_values()
            .map(|package| package.evaluate_with_context(&mut Context::new()))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use hcl::eval::{Context, Evaluate};
    use std::path::PathBuf;

    use super::Loader;

    fn workspace_dir() -> PathBuf {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
        cargo_path.parent().unwrap().to_path_buf()
    }

    #[test]
    fn test() {
        let paths = vec![PathBuf::from("crates/moss-desktop/contributions")];
        let mut loader = Loader::new();
        loader.load(workspace_dir(), paths).unwrap();
        let evaluated = loader
            .packages
            .into_iter()
            .map(|(path, package)| package.evaluate_with_context(&mut Context::new()).unwrap())
            .collect::<Vec<_>>();
        println!("{:#?}", evaluated);
    }

    #[test]
    fn test2() {
        let input = r#"
configuration "moss.core.window" {
    title = "Window"
    order = 5

    parameter "window.defaultWidth" {
        type = number
        minimum = 800
        maximum = 3840
        default = 800
        order = 1
        scope = "APPLICATION"
        description = "The width of the application window in pixels."
    }

    parameter "window.defaultHeight" {
        type = number
        minimum = 600
        maximum = 2160
        default = 600
        order = 2
        scope = "APPLICATION"
        description = "The height of the application window in pixels."
    }

    parameter "editor.fontSize" {
        type = number
        minimum = 10
        maximum = 20
        default = 14
        order = 1
        scope = "WINDOW"
        description = "The width of the application window in pixels."
    }
}
    "#;
        let mut ctx = hcl::eval::Context::new();

        ctx.declare_var("number", "number");
        let body: hcl::Body = hcl::from_str(input).unwrap();
        let r = body.evaluate(&ctx).unwrap();

        let json_value: serde_json::Value = hcl::from_body(r).unwrap();
        let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
        println!("{}", pretty_json);
    }
}
