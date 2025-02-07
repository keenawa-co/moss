use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use hcl::{
    expr::{Traversal, Variable},
    Block, Body, Expression, Object, ObjectKey,
};

use crate::foundations::configuration::{OverrideBodyStmt, ParameterBodyStmt};
use crate::foundations::{
    configuration::{ConfigurationBodyStmt, ConfigurationDecl, OverrideDecl, ParameterDecl},
    scope::ModuleScope,
    token::*,
};

pub fn parse_module_file(input: &str, module_scope: &mut ModuleScope) -> Result<()> {
    let body: Body = hcl::from_str(input)?;

    for block in body.into_blocks() {
        let labels = block.labels();

        match block.identifier() {
            CONFIGURATION_LIT => {
                let (ident, parent_ident) = (labels.get(0), labels.get(2));
                println!("Ident: {:?}, Parent Ident: {:?}", ident, parent_ident);
                if ident.is_some_and(|_| RESERVED_WORDS.contains(ident.unwrap().as_str())) {
                    return Err(anyhow!("Illegal ident `{}`", ident.unwrap().as_str()));
                }
                if parent_ident
                    .is_some_and(|_| RESERVED_WORDS.contains(parent_ident.unwrap().as_str()))
                {
                    return Err(anyhow!(
                        "Illegal parent_ident `{}`",
                        parent_ident.unwrap().as_str()
                    ));
                }

                let decl = match (labels.get(0), labels.get(2)) {
                    // Genesis
                    (Some(ident), None) => {
                        if let Some(keyword) = labels.get(1) {
                            if keyword.as_str() == EXTEND_LIT {
                                return Err(anyhow!(
                                    "Missing parent ident for configuration `{}`",
                                    ident.as_str()
                                ));
                            }
                        }
                        ConfigurationDecl::Genesis {
                            ident: ident.as_str().into(),
                            body: parse_configuration_body(block)?,
                        }
                    }
                    // Successor
                    (Some(ident), Some(parent_ident)) => ConfigurationDecl::Successor {
                        ident: ident.as_str().into(),
                        parent_ident: parent_ident.as_str().into(),
                        body: parse_configuration_body(block)?,
                    },
                    // Anonymous
                    (None, None) => ConfigurationDecl::Anonymous {
                        body: parse_configuration_body(block)?,
                    },
                    _ => {
                        return Err(anyhow!(
                            "Incorrect syntax: {} {:#?}",
                            block.identifier(),
                            block.labels
                        ))
                    }
                };
                module_scope.configurations.push(decl);
            }

            LOCALS_LIT => {
                module_scope.locals.extend(parse_locals_block(block)?);
            }
            _ => {
                continue;
            }
        }
    }

    Ok(())
}

fn parse_locals_block(block: Block) -> Result<HashMap<String, Expression>> {
    let mut result = HashMap::new();

    for attr in block.body.into_attributes() {
        result.insert(attr.key().to_string(), attr.expr);
    }

    Ok(result)
}

fn parse_configuration_body(block: Block) -> Result<ConfigurationBodyStmt> {
    let mut result = ConfigurationBodyStmt {
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

                let mut override_decl = OverrideDecl {
                    ident,
                    body: OverrideBodyStmt {
                        value: Expression::Null,
                        context: Expression::Null,
                    },
                };

                for attribute in block.body.into_attributes() {
                    match attribute.key() {
                        "value" => override_decl.body.value = attribute.expr,
                        "context" => override_decl.body.context = attribute.expr,
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

                let mut parameter_decl = ParameterDecl {
                    ident,
                    body: ParameterBodyStmt {
                        value_type: Expression::Null,
                        maximum: Expression::Null,
                        minimum: Expression::Null,
                        default: Expression::Null,
                        order: Expression::Null,
                        scope: Expression::Null,
                        description: Expression::Null,
                        excluded: Expression::Null,
                        protected: Expression::Null,
                    },
                };

                for attribute in block.body.into_attributes() {
                    match attribute.key() {
                        "type" => parameter_decl.body.value_type = attribute.expr,
                        "maximum" => parameter_decl.body.maximum = attribute.expr,
                        "minimum" => parameter_decl.body.minimum = attribute.expr,
                        "default" => parameter_decl.body.default = attribute.expr,
                        "order" => parameter_decl.body.order = attribute.expr,
                        "scope" => parameter_decl.body.scope = attribute.expr,
                        "description" => parameter_decl.body.description = attribute.expr,
                        "excluded" => parameter_decl.body.excluded = attribute.expr,
                        "protected" => parameter_decl.body.protected = attribute.expr,
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

pub(crate) fn collect_local_refs(expr: &Expression) -> HashSet<String> {
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
    use super::parse_module_file;
    use crate::foundations::scope::ModuleScope;

    fn resolve(input: &str) {
        // FIXME: Rewrite this test
        let mut scope = ModuleScope::new();
        parse_module_file(input, &mut scope).unwrap();
        println!("Module: {:#?}", scope);
    }
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

        resolve(input);

        // let resolved = resolve(parsed_module).unwrap();
        // dbg!(resolved);
    }

    #[test]
    fn test_extend_normal() {
        let input = r#"
        configuration "parent" {}
        configuration "child" extends "parent" {}
        "#;
        resolve(input);
    }

    #[test]
    fn test_extend_other() {
        let input = r#"
        configuration {}
        configuration {}
        "#;
        resolve(input);
    }

    #[test]
    #[should_panic]
    fn test_extend_missing_child_ident() {
        let input = r#"
        configuration "Parent" {}
        configuration extends "Parent" {}
        "#;
        resolve(input);
    }
    #[test]
    #[should_panic]
    fn test_extend_missing_parent_ident() {
        let input = r#"
        configuration "Parent" {}
        configuration "Child" extends {}
        "#;
        resolve(input);
    }

    // TODO: Move these tests to the validation step
    // #[test]
    // #[should_panic]
    // fn test_extend_nonexistent_parent() {
    //     let input = r#"
    //     configuration "Child" extends "NonExistent" {}
    //     "#;
    //     resolve(input);
    // }

    // #[test]
    // #[should_panic]
    // fn test_duplicate_configuration() {
    //     let input = r#"
    //     configuration "Duplicate" {}
    //     configuration "Duplicate" {}
    //     "#;
    //     resolve(input);
    // }

    // TODO: testing further validation logic
}

// #[test]
// fn test_cycle_sort_error() {
//     let input = r#"
//     locals {
//         A = local.B
//         B = local.C
//         C = local.A
//     }
//     "#;
//     let parsed_module = parse(input).unwrap();

//     assert_eq!(
//         parsed_module.unwrap_err().to_string(),
//         "Cycle detected in locals"
//     );
// }

// #[test]
// fn test_cycle_self_referential_sort_error() {
//     let input = r#"
//     locals {
//         A = local.A
//     }
//     "#;
//     let parsed_module = parse(input).unwrap();

//     assert_eq!(
//         parsed_module.unwrap_err().to_string(),
//         "Cycle detected in locals"
//     );
// }

// #[test]
// fn test_cycle_with_objects_sort_error() {
//     let input = r#"
//     locals {
//         A = {
//             x = local.B.x
//         }
//         B = {
//             x = local.A.x
//         }
//     }

//     "#;
//     let parsed_module = parse(input).unwrap();

//     assert_eq!(
//         parsed_module.unwrap_err().to_string(),
//         "Cycle detected in locals"
//     );
// }

// #[test]
// fn test_cycle_with_arrays_sort_error() {
//     let input = r#"
//     locals {
//         arr = [local.A.x]
//         A = {
//             x = local.arr[0]
//         }
//     }
//     "#;
//     let parsed_module = parse(input).unwrap();

//     assert_eq!(
//         parsed_module.unwrap_err().to_string(),
//         "Cycle detected in locals"
//     );
// }

// #[test]
// fn test_unregistered_local_variable_eval_error() {
//     let input = r#"
//     locals {
//         A = local.B
//     }
//     "#;
//     let parsed_module = parse(input).unwrap();

//     assert_eq!(
//         parsed_module.unwrap_err().to_string(),
//         "no such key: `B` in expression `local.B`"
//     );
// }
