use proc_macro::TokenStream;
use quote::ToTokens;
use std::fs;
use std::path::PathBuf;
use syn::parse::{Parse, Parser};
use syn::Fields::Named;
use syn::{Field, ItemStruct, Type};

#[proc_macro_attribute]
pub fn type_bridge(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = item.clone();
    let struct_parser = ItemStruct::parse;
    let struct_item = struct_parser.parse(tokens).unwrap();

    let struct_def = parse_struct(struct_item);
    println!("{}", struct_def.export_to_ts());
    TokenStream::new()
}

fn parse_struct(struct_item: ItemStruct) -> StructDefinition {
    let name = struct_item.ident.to_string();

    if let Named(fields) = struct_item.fields {
        let fields = fields
            .named
            .into_iter()
            .map(|field| parse_field(field))
            .collect::<Vec<_>>();

        let struct_def = StructDefinition { name, fields };
        struct_def
    } else {
        StructDefinition {
            name,
            fields: Vec::new(),
        }
    }
}

fn parse_field(field: Field) -> FieldDefinition {
    let name = field.ident.unwrap().to_string();
    let ty = field.ty.to_token_stream().to_string();
    println!("{}", ty);
    let parts: Vec<String> = ty.split(" ").map(|s| s.to_string()).collect();
    let type_def = parse_type(parts);
    FieldDefinition { name, ty: type_def }
}

fn parse_type(parts: Vec<String>) -> TypeDefinition {
    if parts.len() == 1 {
        parse_basic_type(parts.first().unwrap())
    } else {
        parse_compound_type(parts)
    }
}

fn parse_basic_type(type_str: &str) -> TypeDefinition {
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

fn remove_square_brackets(type_parts: Vec<String>) -> Vec<String> {
    let start_index = type_parts.iter().position(|t| t == "<").unwrap() + 1;
    let end_index = type_parts.iter().rposition(|t| t == ">").unwrap() - 1;
    type_parts[start_index..=end_index].to_vec()
}

fn extract_inner_type(type_parts: Vec<String>) -> TypeDefinition {
    let inner_type_parts = remove_square_brackets(type_parts);
    parse_type(inner_type_parts)
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

#[derive(Debug)]
enum TypeBase {
    Integer(usize),
    Unsigned(usize),
    Float(usize),
    Boolean,
    Character,
    String,
    Array(Box<TypeDefinition>, usize),
    Vec(Box<TypeDefinition>),
    Custom(String),
    Unknown,
}

#[derive(Debug)]
struct FieldDefinition {
    name: String,
    ty: TypeDefinition,
}

#[derive(Debug)]
struct TypeDefinition {
    base: TypeBase,
    is_nullable: bool,
}

#[derive(Debug)]
struct StructDefinition {
    name: String,
    fields: Vec<FieldDefinition>,
}

trait TSExport {
    fn export_to_ts(&self) -> String;
}
impl TSExport for StructDefinition {
    fn export_to_ts(&self) -> String {
        let mut struct_ts: Vec<String> = vec![];
        struct_ts.extend(self.fields.iter().map(|field| field.export_to_ts()));
        struct_ts.insert(0, format!("interface {} {{", self.name));
        struct_ts.push("}".to_string());
        struct_ts.join("\n")
    }
}

impl TSExport for FieldDefinition {
    fn export_to_ts(&self) -> String {
        format!(
            "\t{}{}: {};",
            self.name,
            if self.ty.is_nullable { "?" } else { "" },
            self.ty.export_to_ts()
        )
    }
}

impl TSExport for TypeBase {
    fn export_to_ts(&self) -> String {
        match self {
            TypeBase::Integer(size) => {
                if *size <= 32 {
                    "number".to_string()
                } else {
                    "bigint".to_string()
                }
            }
            TypeBase::Unsigned(size) => {
                if *size <= 32 {
                    "number".to_string()
                } else {
                    "bigint".to_string()
                }
            }
            TypeBase::Float(_) => "number".to_string(),
            TypeBase::Boolean => "boolean".to_string(),
            TypeBase::Character => "string".to_string(),
            TypeBase::String => "string".to_string(),
            TypeBase::Array(ref base, ref size) => {
                format!("{}[]", base.export_to_ts())
            }
            TypeBase::Vec(ref base) => {
                format!("{}[]", base.export_to_ts())
            }
            TypeBase::Unknown => "any".to_string(),
            _ => "any".to_string(),
        }
    }
}

impl TSExport for TypeDefinition {
    fn export_to_ts(&self) -> String {
        self.base.export_to_ts()
    }
}
