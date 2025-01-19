use std::result;

use anyhow::{anyhow, Result};
use hashbrown::{HashMap, HashSet};
use hcl::{
    eval::{Context, Evaluate},
    expr::{Traversal, Variable},
    Block, Body, Expression, Identifier, Map, Object,
};
use phf::phf_set;

use crate::interpreter::{ParsedConfigurationDecl, ParserLocalVarDecl, ResolvedScope, Scope};

const CONFIGURATION_LIT: &'static str = "configuration";
const LOCALS_LIT: &'static str = "locals";

const CONFIGURATION_IDENT_POS: usize = 0;
const CONFIGURATION_PARENT_IDENT_POS: usize = 1;

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

                    for local_name in parsed.keys() {
                        let idx = graph.add_node(local_name.clone());
                        node_map.insert(local_name.clone(), idx);
                    }

                    for (name, expr) in parsed {
                        let from_idx = node_map[&name];
                        let deps = collect_local_refs(&expr);

                        for dep in deps {
                            if let Some(&to_idx) = node_map.get(&dep) {
                                graph.add_edge(from_idx, to_idx, ());
                            }
                        }
                    }

                    let _ = petgraph::algo::toposort(&graph, None)
                        .map_err(|_| anyhow!("Cycle detected in locals"))?;
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

    fn parse_configuration_block(&self, block: Block) -> Result<ParsedConfigurationDecl> {
        let mut result = ParsedConfigurationDecl {
            ident: block
                .labels()
                .get(CONFIGURATION_IDENT_POS)
                .map(|label| label.as_str().to_string()),
            parent_ident: block
                .labels()
                .get(CONFIGURATION_PARENT_IDENT_POS)
                .map(|label| label.as_str().to_string()),
            display_name: None,
            description: None,
            order: None,
            parameters: Vec::new(),
        };

        for attribute in block.body.into_attributes() {
            match attribute.key() {
                "display_name" => result.display_name = Some(attribute.expr),
                "description" => result.description = Some(attribute.expr),
                "order" => result.order = Some(attribute.expr),
                _ => {
                    // TODO: Add logging for encountering an unknown attribute
                }
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
            private_test = "Hello, World!"
            test = local.private_test
            some = local.test
        }

        configuration "moss.kernel.window" {
            display_name = local.test
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

        let parser = Parser::new();
        let parsed_module = parser.parse_module(input).unwrap();
        // let resolved = resolve(parsed_module).unwrap();
        // dbg!(resolved);
    }
}
