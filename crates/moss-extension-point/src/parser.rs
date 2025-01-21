use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use hcl::{
    eval::{Context, Evaluate},
    expr::{Traversal, Variable},
    Block, Body, Expression, Object, ObjectKey, Value,
};
use phf::phf_set;

use crate::interpreter::{
    ConfigurationDecl, ConfigurationOverrideDecl, ConfigurationParameterDecl, Scope,
};

const CONFIGURATION_LIT: &'static str = "configuration";
const PARAMETER_LIT: &'static str = "parameter";
const OVERRIDE_LIT: &'static str = "override";
const LOCALS_LIT: &'static str = "locals";

// TODO

static RESERVED_WORDS: phf::Set<&'static str> = phf_set! {
    "configuration",
    "theme",
};

fn is_reserved_word(value: &str) -> bool {
    RESERVED_WORDS.get_key(value).is_some()
}

fn is_valid_configuration_id(value: &str) -> bool {
    unimplemented!()
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_module(&self, input: &str) -> Result<Scope> {
        let body: Body = hcl::from_str(input)?;
        let mut result = Scope::default();
        for block in body.into_blocks() {
            match block.identifier() {
                CONFIGURATION_LIT => {
                    let parsed = self.parse_configuration_block(block)?;
                    result.configurations.push(parsed);
                }
                LOCALS_LIT => {
                    let parsed = self.parse_locals_block(block)?;
                    let mut graph = petgraph::Graph::<String, ()>::new();
                    let mut node_map = HashMap::new();
                    let mut name_map = HashMap::new();
                    for local_name in parsed.keys() {
                        let idx = graph.add_node(local_name.clone());
                        node_map.insert(local_name.clone(), idx);
                        name_map.insert(idx, local_name.clone());
                    }

                    for (name, expr) in parsed.iter() {
                        let from_idx = node_map[name];
                        let deps = collect_local_refs(&expr);

                        for dep in deps {
                            if let Some(&to_idx) = node_map.get(&dep) {
                                graph.add_edge(from_idx, to_idx, ());
                            }
                        }
                    }

                    let dependency_chain = petgraph::algo::toposort(&graph, None)
                        .map_err(|_| anyhow!("Cycle detected in locals"))?
                        .iter()
                        .map(|idx| name_map.get(idx).unwrap().to_string())
                        .rev()
                        .collect::<Vec<String>>();

                    for name in dependency_chain.iter() {
                        // TODO: We could potentially optimize this part
                        let expr = parsed.get(name).unwrap();
                        let mut ctx = Context::new();
                        ctx.declare_var("local", Value::Object(result.locals.clone()));
                        let value = expr.evaluate(&ctx)?;
                        result.locals.insert(name.to_string(), value.clone());
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        Ok(result)
    }

    fn parse_locals_block(&self, block: Block) -> Result<HashMap<String, Expression>> {
        let mut result = HashMap::new();

        for attr in block.body.into_attributes() {
            result.insert(attr.key().to_string(), attr.expr);
        }

        Ok(result)
    }

    fn parse_configuration_block(&self, block: Block) -> Result<ConfigurationDecl> {
        let mut result = ConfigurationDecl {
            ident: block
                .labels()
                .get(0)
                .map(|label| ArcStr::from(label.as_str())),
            parent_ident: block
                .labels()
                .get(1)
                .map(|label| ArcStr::from(label.as_str())),
            display_name: Expression::Null,
            description: Expression::Null,
            order: Expression::Null,
            parameters: Vec::new(),
            overrides: Vec::new(),
        };

        for attribute in block.body.clone().into_attributes() {
            match attribute.key() {
                "display_name" => result.display_name = attribute.expr,
                "description" => result.description = attribute.expr,
                "order" => result.order = attribute.expr,
                _ => {

                    // TODO: Add logging for encountering an unknown attribute
                }
            }
        }

        for block in block.body.into_blocks() {
            match block.identifier() {
                OVERRIDE_LIT => {
                    let ident = if let Some(label) = block
                        .labels()
                        .get(0)
                        .map(|label| ArcStr::from(label.as_str()))
                    {
                        label
                    } else {
                        // TODO: Add logging for encountering an unknown parameter
                        continue;
                    };

                    let mut override_decl = ConfigurationOverrideDecl {
                        ident,
                        value: Expression::Null,
                        context: Expression::Null,
                    };

                    for attribute in block.body.into_attributes() {
                        match attribute.key() {
                            "value" => override_decl.value = attribute.expr,
                            "context" => override_decl.context = attribute.expr,
                            _ => {

                                // TODO: Add logging for encountering an unknown attribute
                            }
                        }
                    }

                    result.overrides.push(override_decl);
                }

                PARAMETER_LIT => {
                    let ident = if let Some(label) = block
                        .labels()
                        .get(0)
                        .map(|label| ArcStr::from(label.as_str()))
                    {
                        label
                    } else {
                        // TODO: Add logging for encountering an unknown parameter
                        continue;
                    };

                    let mut parameter_decl = ConfigurationParameterDecl {
                        ident,
                        value_type: Expression::Null,
                        maximum: Expression::Null,
                        minimum: Expression::Null,
                        default: Expression::Null,
                        order: Expression::Null,
                        scope: Expression::Null,
                        description: Expression::Null,
                        excluded: Expression::Null,
                        protected: Expression::Null,
                    };

                    for attribute in block.body.into_attributes() {
                        match attribute.key() {
                            "type" => parameter_decl.value_type = attribute.expr,
                            "maximum" => parameter_decl.maximum = attribute.expr,
                            "minimum" => parameter_decl.minimum = attribute.expr,
                            "default" => parameter_decl.default = attribute.expr,
                            "order" => parameter_decl.order = attribute.expr,
                            "scope" => parameter_decl.scope = attribute.expr,
                            "description" => parameter_decl.description = attribute.expr,
                            "excluded" => parameter_decl.excluded = attribute.expr,
                            "protected" => parameter_decl.protected = attribute.expr,
                            _ => {

                                // TODO: Add logging for encountering an unknown attribute
                            }
                        }
                    }

                    result.parameters.push(parameter_decl);
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

fn collect_local_refs(expr: &Expression) -> HashSet<String> {
    let mut set = HashSet::new();

    match expr {
        Expression::Null | Expression::Bool(_) | Expression::Number(_) | Expression::String(_) => {
            //
        }
        Expression::Variable(var) => {
            if let Some(dep_name) = parse_local_variable(var) {
                set.insert(dep_name);
            }
        }
        Expression::Traversal(trav) => {
            set.extend(collect_refs_in_traversal(trav));
        }
        Expression::Object(obj) => {
            set.extend(collect_refs_in_object(obj));
        }
        Expression::Array(arr) => {
            set.extend(collect_refs_in_array(arr));
        }
        _ => unimplemented!(),
    }

    set
}

fn collect_refs_in_traversal(trav: &Traversal) -> HashSet<String> {
    let mut set = HashSet::new();

    set.extend(collect_local_refs(&trav.expr));

    for op in &trav.operators {
        match op {
            hcl::TraversalOperator::GetAttr(ident) => {
                if let Expression::Variable(base_var) = &trav.expr {
                    if base_var.as_str() == "local" {
                        set.insert(ident.as_str().to_string());
                        break;
                    }
                }
            }
            hcl::TraversalOperator::Index(idx_expr) => {
                set.extend(collect_local_refs(idx_expr));
            }
            _ => {}
        }
    }

    set
}

fn collect_refs_in_object(obj: &Object<ObjectKey, Expression>) -> HashSet<String> {
    let mut set = HashSet::new();
    for (_, expr) in obj.iter() {
        set.extend(collect_local_refs(expr));
    }
    set
}

fn collect_refs_in_array(arr: &Vec<Expression>) -> HashSet<String> {
    let mut set = HashSet::new();
    for expr in arr {
        set.extend(collect_local_refs(&expr.to_owned().into()));
    }
    set
}

fn parse_local_variable(var: &Variable) -> Option<String> {
    let full_name = var.as_str();
    if let Some(stripped) = full_name.strip_prefix("local.") {
        // local.xxx
        Some(stripped.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test() {
        let input = r#"
        locals {
            desc = "The width of the application window in pixels 2"
            default = {
                width = 800
            }
            dimensions = [800, 3840]
        }
        configuration "moss.kernel.window" {
            display_name = "Window"
            description = local.desc

            parameter "window.defaultWidth" {
                type = number
                minimum = local.dimensions[0]
                maximum = local.dimensions[1]
                default = local.default.width
                order = 1
                scope = "APPLICATION"
                description = local.desc
            }

            override "editor.fontSize" {
                value = 16
            }
        }
            "#;

        let parser = Parser::new();
        let parsed_module = parser.parse_module(input).unwrap();

        let resolved = parsed_module.evaluate().unwrap();
        println!("Resolved: {:#?}", resolved);

        // let resolved = resolve(parsed_module).unwrap();
        // dbg!(resolved);
    }

    #[test]
    fn test_cycle_sort_error() {
        let input = r#"
        locals {
            A = local.B
            B = local.C
            C = local.A
        }
        "#;
        let parser = Parser::new();
        let parsed_module = parser.parse_module(input);
        assert_eq!(
            parsed_module.unwrap_err().to_string(),
            "Cycle detected in locals"
        );
    }

    #[test]
    fn test_cycle_self_referential_sort_error() {
        let input = r#"
        locals {
            A = local.A
        }
        "#;
        let parser = Parser::new();
        let parsed_module = parser.parse_module(input);
        assert_eq!(
            parsed_module.unwrap_err().to_string(),
            "Cycle detected in locals"
        );
    }

    #[test]
    fn test_cycle_with_objects_sort_error() {
        let input = r#"
        locals {
            A = {
                x = local.B.x
            }
            B = {
                x = local.A.x
            }
        }


        "#;
        let parser = Parser::new();
        let parsed_module = parser.parse_module(input);
        assert_eq!(
            parsed_module.unwrap_err().to_string(),
            "Cycle detected in locals"
        );
    }

    #[test]
    fn test_cycle_with_arrays_sort_error() {
        let input = r#"
        locals {
            arr = [local.A.x]
            A = {
                x = local.arr[0]
            }
        }
        "#;
        let parser = Parser::new();
        let parsed_module = parser.parse_module(input);
        assert_eq!(
            parsed_module.unwrap_err().to_string(),
            "Cycle detected in locals"
        );
    }

    #[test]
    fn test_unregistered_local_variable_eval_error() {
        let input = r#"
        locals {
            A = local.B
        }
        "#;
        let parser = Parser::new();
        let parsed_module = parser.parse_module(input);
        assert_eq!(
            parsed_module.unwrap_err().to_string(),
            "no such key: `B` in expression `local.B`"
        );
    }
}
