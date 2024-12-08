use crate::parser::parse_type_string;
use crate::TypeDefinition;
use quote::ToTokens;
use syn::Fields::Named;
use syn::{Attribute, Field, ItemStruct};

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<StructFieldDefinition>,
}

#[derive(Debug)]
pub struct StructFieldDefinition {
    pub name: String,
    pub ty: TypeDefinition,
}

pub fn parse_struct(struct_item: ItemStruct) -> StructDefinition {
    let name = struct_item.ident.to_string();

    if let Named(fields) = struct_item.fields {
        let fields = fields
            .named
            .into_iter()
            .map(|field| parse_struct_field(field))
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

fn parse_struct_field(field: Field) -> StructFieldDefinition {
    let mut name = field.ident.unwrap().to_string();
    let serde_attrs_string = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("serde"))
        .map(|attr| attr.meta.require_list().unwrap().tokens.clone().to_string())
        .collect::<Vec<_>>()
        .join(",");

    let serde_attrs = serde_attrs_string
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    for attr in serde_attrs {
        if attr.contains("=") {
            let split = attr
                .split("=")
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            let key = split[0].to_string();
            let value = split[1].to_string();
            match key.as_str() {
                "rename" => {
                    name = value
                        .strip_prefix('"')
                        .unwrap()
                        .strip_suffix('"')
                        .unwrap()
                        .to_string();
                }
                _ => {}
            }
        } else {
            match attr {
                _ => {}
            }
        }
    }

    let type_string = field.ty.to_token_stream().to_string();
    //println!("{}", type_string);
    let type_def = parse_type_string(&type_string);
    StructFieldDefinition { name, ty: type_def }
}
