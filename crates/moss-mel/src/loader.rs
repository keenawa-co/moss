use crate::eval::evaluate_locals;
use crate::foundations::configuration::{ConfigurationDecl, ConfigurationNode};
use crate::foundations::scope::{ModuleScope, ResolvedScope};
use crate::parse::parse_module_file;
use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::HashMap;
use hcl::eval::{Context as EvalContext, Context};
use hcl::Value::Object;
use hcl::{Expression, Identifier, Map as HclMap, Value};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

const FILE_EXTENSION: &'static str = "hcl";

#[derive(Debug)]
struct Package {
    modules: HashMap<PathBuf, ModuleScope>,
}

impl Package {
    fn resolve_decl_order(&self) -> Result<Vec<(PathBuf, ConfigurationDecl)>> {
        let configurations = self
            .modules
            .iter()
            .flat_map(|(path, module)| {
                module
                    .configurations
                    .iter()
                    .map(|conf| (path.clone(), conf.clone()))
            })
            .collect::<Vec<(PathBuf, ConfigurationDecl)>>();

        let mut named_confs = HashMap::new();
        let mut genesis = Vec::new();
        let mut successor = Vec::new();
        let mut anonymous = Vec::new();

        for (path, conf) in configurations {
            match conf {
                ConfigurationDecl::Genesis { ref ident, .. } => {
                    if named_confs.contains_key(ident) {
                        return Err(anyhow!("Duplicte configuration ident `{}`", ident));
                    }
                    genesis.push((path.clone(), ident.clone()));
                    named_confs.insert(ident.clone(), (path, conf));
                }
                ConfigurationDecl::Successor { ref ident, .. } => {
                    if named_confs.contains_key(ident) {
                        return Err(anyhow!("Duplicte configuration ident `{}`", ident));
                    }
                    successor.push((path.clone(), ident.clone()));
                    named_confs.insert(ident.clone(), (path, conf));
                }
                ConfigurationDecl::Anonymous { .. } => {
                    anonymous.push((path, conf));
                }
            }
        }

        // TODO: Right now we only have a basic extend order resolution
        // We will need more sophisticated dependency tracking in the future
        // Similar to `collect_local_refs`

        let mut extend_graph = petgraph::Graph::<ArcStr, ()>::new();
        let mut node_map = HashMap::new();
        let mut name_map = HashMap::new();

        for ident in named_confs.keys() {
            let idx = extend_graph.add_node(ident.clone());
            node_map.insert(ident.clone(), idx);
            name_map.insert(idx, ident.clone());
        }

        for (path, ident) in successor {
            let parent_ident = named_confs[&ident].1.parent_ident().unwrap();
            let from_idx = node_map[&ident];

            if let Some(&to_idx) = node_map.get(&parent_ident) {
                println!("{} depends on {}", ident, parent_ident);
                extend_graph.add_edge(from_idx, to_idx, ());
            } else {
                return Err(anyhow!("Cannot find configuration `{}`", parent_ident));
            }
        }

        Ok(petgraph::algo::toposort(&extend_graph, None)
            .map_err(|_| anyhow!("Cycle detected in extends"))?
            .into_iter()
            .rev()
            .map(|idx| name_map.get(&idx).unwrap())
            .map(|name| named_confs.get(name).unwrap().to_owned())
            .collect::<Vec<_>>())
    }

    pub fn evaluate_with_context(self, ctx: &mut Context) -> Result<ResolvedScope> {
        let mut result = ResolvedScope::new();
        let mut global_ctxmap = HclMap::<Identifier, Value>::new();
        let module_context = self
            .modules
            .iter()
            .map(|(path, module)| (path.clone(), module.generate_ctx().unwrap()))
            .collect::<HashMap<PathBuf, Context>>();

        let mut anonymous_extends = Vec::new();
        for (path, module) in self.modules.iter() {
            for conf in module.configurations.iter() {
                if conf.ident().is_none() {
                    anonymous_extends.push((path.clone(), conf.clone()));
                }
            }
        }

        // TODO: Introducing package-level variables

        // TODO: Right now we only have a basic extend order resolution
        // We will need more sophisticated dependency tracking in the future
        // Similar to `collect_local_refs`
        let resolution_order: Vec<(PathBuf, ConfigurationDecl)> = self.resolve_decl_order()?;

        for (path, decl) in resolution_order {
            let mut ctx = module_context.get(&path).unwrap().clone();
            global_ctxmap
                .iter()
                .for_each(|(ident, value)| ctx.declare_var(ident.clone(), value.clone()));
            let evaluated = decl.evaluate(&ctx)?;
            result.insert_configuration(evaluated.ident.clone().as_str(), evaluated);
            // TODO: update the global context based on newly evaluated configuration node
        }

        for (path, decl) in anonymous_extends {
            let mut ctx = module_context.get(&path).unwrap().clone();
            global_ctxmap
                .iter()
                .for_each(|(ident, value)| ctx.declare_var(ident.clone(), value.clone()));
            let evaluated = decl.evaluate(&ctx)?;
            result.insert_anonymous_extends(evaluated);
        }

        Ok(result)
    }
}

fn validate_successor(parent: &ConfigurationDecl, successor: &ConfigurationDecl) -> Result<()> {
    // TODO: implement various validation logic
    // e.g. No duplicate parameter names
    Ok(())
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
        let mut ctx = EvalContext::new();
        for path in paths {
            let package_path = &workspace_root.join(&path);
            let package = self.load_package(package_path)?;
            self.packages.insert(package_path.to_owned(), package);
        }

        Ok(())
    }

    fn load_package(&self, path: &PathBuf) -> Result<Package> {
        let mut read_dir = std::fs::read_dir(path)?;
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
        let mut read_dir = std::fs::read_dir(path)?;
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
