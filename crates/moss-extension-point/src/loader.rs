use anyhow::Result;
use hashbrown::HashMap;
use hcl::eval::Context as EvalContext;
use moss_mel::{foundations::scope::ResolvedScope, parse::parse};
use std::path::PathBuf;

const FILE_EXTENSION: &'static str = "hcl";

pub struct Loader {
    modules: HashMap<PathBuf, ResolvedScope>,
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
        let mut module = ResolvedScope::new();

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

                let ep_file = self.parse_file(ctx, &path)?;
                module.configurations.extend(ep_file.configurations);
            }
        }

        Ok(module)
    }

    fn parse_file(&self, ctx: &mut EvalContext, path: &PathBuf) -> Result<ResolvedScope> {
        let file_content = std::fs::read_to_string(path)?;

        parse(&file_content).map(|scope| scope.evaluate_with_context(ctx))?
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
