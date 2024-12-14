mod exporter;
mod parameters;
mod parser;
mod serde;

use proc_macro::{TokenStream, TokenTree};
use quote::ToTokens;
use std::fs;
use syn::parse::{Parse, Parser};

use crate::exporter::{Exporter, TSExporter};
use crate::parameters::parse_macro_args;
use crate::parameters::OutputLanguage::TypeScript;
use crate::parser::enum_parser::parse_enum;
use crate::parser::struct_parser::parse_struct;
use syn::{ItemEnum, ItemStruct};

pub(crate) enum ItemCategory {
    Struct,
    Enum,
    Invalid,
}

#[derive(Debug)]
pub(crate) enum TypeBase {
    Integer(usize),
    Unsigned(usize),
    Float(usize),
    Boolean,
    Character,
    String,
    Array(Box<TypeDefinition>, usize),
    Vec(Box<TypeDefinition>),
    Tuple(Vec<TypeDefinition>),
    Custom(String),
    Unknown,
}

#[derive(Debug)]
pub(crate) struct TypeDefinition {
    base: TypeBase,
    is_nullable: bool,
}

#[proc_macro_attribute]
pub fn type_bridge(args: TokenStream, item: TokenStream) -> TokenStream {
    let parameters = parse_macro_args(args);
    let category = detect_item_category(item.clone());

    match category {
        ItemCategory::Struct => {
            let struct_item = ItemStruct::parse.parse(item.clone()).unwrap();
            let struct_definition = parse_struct(struct_item);
            let struct_export = match parameters.language {
                TypeScript => TSExporter::export_struct(&struct_definition),
            };
            fs::write(parameters.output_path, struct_export).unwrap();
        }
        ItemCategory::Enum => {
            let enum_item = ItemEnum::parse.parse(item.clone()).unwrap();
            let enum_definition = parse_enum(enum_item);
        }
        _ => {}
    }
    TokenStream::new()
}

fn detect_item_category(input: TokenStream) -> ItemCategory {
    let tokens = input;
    for token in tokens {
        match token {
            TokenTree::Group(_) => {}
            TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "struct" => return ItemCategory::Struct,
                "enum" => return ItemCategory::Enum,
                _ => {}
            },
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
    }
    ItemCategory::Invalid
}
