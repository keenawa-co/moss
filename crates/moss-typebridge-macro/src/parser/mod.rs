pub(super) mod enum_parser;
pub(super) mod struct_parser;

use crate::{TypeBase, TypeDefinition};
use quote::ToTokens;

fn parse_type_string(type_str: &str) -> TypeDefinition {
    if type_str.starts_with("(") && type_str.ends_with(")") {
        return parse_tuple(type_str);
    }
    let parts: Vec<String> = type_str.split(" ").map(|s| s.to_string()).collect();
    if parts.len() == 1 {
        parse_simple_type(parts.first().unwrap())
    } else {
        //println!("{}", type_str);
        parse_compound_type(parts)
    }
}

fn parse_simple_type(type_str: &str) -> TypeDefinition {
    match type_str {
        "i32" => TypeDefinition {
            base: TypeBase::Integer(32),
            is_nullable: false,
        },
        "usize" => TypeDefinition {
            base: TypeBase::Unsigned(32),
            is_nullable: false,
        },
        "f64" => TypeDefinition {
            base: TypeBase::Float(64),
            is_nullable: false,
        },
        "bool" => TypeDefinition {
            base: TypeBase::Boolean,
            is_nullable: false,
        },
        "char" => TypeDefinition {
            base: TypeBase::Character,
            is_nullable: false,
        },
        "String" => TypeDefinition {
            base: TypeBase::String,
            is_nullable: false,
        },
        custom => TypeDefinition {
            base: TypeBase::Custom(custom.to_string()),
            is_nullable: false,
        },
    }
}

fn parse_tuple(tuple_str: &str) -> TypeDefinition {
    let inner = tuple_str
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap();
    let tuple_types = inner
        .split(",")
        .map(|e| e.trim())
        .map(|e| parse_type_string(e))
        .collect();
    let base = TypeBase::Tuple(tuple_types);
    let type_def = TypeDefinition {
        base,
        is_nullable: false,
    };
    //println!("{:?}", type_def);
    type_def
}

fn remove_square_brackets(type_parts: Vec<String>) -> Vec<String> {
    let start_index = type_parts.iter().position(|t| t == "<").unwrap() + 1;
    let end_index = type_parts.iter().rposition(|t| t == ">").unwrap() - 1;
    type_parts[start_index..=end_index].to_vec()
}

fn extract_inner_type(type_parts: Vec<String>) -> TypeDefinition {
    let inner_type_parts = remove_square_brackets(type_parts);
    parse_type_string(inner_type_parts.join(" ").as_str())
}

fn parse_compound_type(type_parts: Vec<String>) -> TypeDefinition {
    match type_parts.first().unwrap().as_str() {
        "Option" => {
            let inner_type = extract_inner_type(type_parts);
            TypeDefinition {
                is_nullable: true,
                ..inner_type
            }
        }
        "Vec" => {
            let inner_type = extract_inner_type(type_parts);
            TypeDefinition {
                is_nullable: false,
                base: TypeBase::Vec(Box::new(inner_type)),
            }
        }
        _ => TypeDefinition {
            is_nullable: false,
            base: TypeBase::Unknown,
        },
    }
}
