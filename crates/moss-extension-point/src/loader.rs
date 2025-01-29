use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::eval::Context as EvalContext;
use moss_mel::foundations::configuration::ConfigurationNode;
use moss_mel::foundations::scope::ScopeRepr;
use moss_mel::{foundations::scope::ResolvedScope, parse::parse};
use std::collections::HashSet;
use std::path::PathBuf;

const FILE_EXTENSION: &'static str = "hcl";

// struct Package {
//     modules: HashMap<PathBuf, ResolvedScope>,
// }

pub struct Loader {
    modules: HashMap<PathBuf, ResolvedScope>,
    // packages: HashMap<PathBuf, Package>
}

impl Loader {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn modules(self) -> HashMap<PathBuf, ResolvedScope> {
        self.modules
    }

    pub fn load(&mut self, workspace_root: PathBuf, paths: Vec<PathBuf>) -> Result<()> {
        let mut ctx = EvalContext::new();

        for path in paths {
            let module_path = &workspace_root.join(path);
            let module = self.load_module(&mut ctx, &workspace_root.join(&module_path))?;
            self.modules.insert(module_path.to_path_buf(), module);
        }

        Ok(())
    }

    fn load_module(&self, ctx: &mut EvalContext, path: &PathBuf) -> Result<ResolvedScope> {
        let mut read_dir = std::fs::read_dir(path)?;
        let mut module_scope = ScopeRepr::new();

        while let Some(entry) = read_dir.next() {
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

                self.parse_module_file(&mut module_scope, ctx, &path)?;
            }
        }

        // FIXME:
        // The `eval` operation now needs to be performed at the package level, not the module level.
        // This is because, in the future, a module might reference other modules.
        // Therefore, we must first collect all modules, build the module dependency graph, and then execute the `eval` operation.
        Ok(module_scope.evaluate_with_context(ctx)?)
    }

    fn parse_module_file(
        &self,
        module_scope: &mut ScopeRepr,
        _ctx: &mut EvalContext,
        path: &PathBuf,
    ) -> Result<()> {
        let file_content = std::fs::read_to_string(path)?;

        // OPTIMIZE: We can change the parse function to directly update a ScopeRepr
        // But I'm not sure if it's necessary.
        let mut newly_parsed = parse(&file_content)?;
        module_scope
            .configurations
            .append(&mut newly_parsed.configurations);
        module_scope
            .configuration_extends
            .append(&mut newly_parsed.configuration_extends);
        module_scope.locals.append(&mut newly_parsed.locals);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hcl::eval::Evaluate;
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
        println!("{:#?}", loader.modules());
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
        // let v: crate::module::ExtensionPointFile =
        //     serde_json::from_value(json_value.clone()).unwrap();
        // dbg!(v);
        let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
        println!("{}", pretty_json);
    }
}
