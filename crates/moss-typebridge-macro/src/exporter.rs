use crate::parser::struct_parser::{StructDefinition, StructFieldDefinition};
use crate::{TypeBase, TypeDefinition};

pub trait Exporter {
    fn export_struct(struct_definition: &StructDefinition) -> String;
    fn export_struct_field(field_definition: &StructFieldDefinition) -> String;

    fn export_type_definition(type_definition: &TypeDefinition) -> String;
    fn export_type_base(type_base: &TypeBase) -> String;
}

pub struct TSExporter {}
impl Exporter for TSExporter {
    fn export_struct(struct_definition: &StructDefinition) -> String {
        let mut struct_ts: Vec<String> = vec![];
        struct_ts.extend(
            struct_definition
                .fields
                .iter()
                .map(|field| Self::export_struct_field(field)),
        );
        struct_ts.insert(0, format!("interface {} {{", struct_definition.name));
        struct_ts.push("}".to_string());
        struct_ts.join("\n")
    }

    fn export_struct_field(field_definition: &StructFieldDefinition) -> String {
        format!(
            "\t{}: {};",
            field_definition.name,
            Self::export_type_definition(&field_definition.ty)
        )
    }

    fn export_type_definition(type_definition: &TypeDefinition) -> String {
        format!(
            "{}{}",
            Self::export_type_base(&type_definition.base),
            if type_definition.is_nullable {
                " | null"
            } else {
                ""
            }
        )
    }

    fn export_type_base(type_base: &TypeBase) -> String {
        match type_base {
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
                format!("{}[]", Self::export_type_definition(base))
            }
            TypeBase::Vec(ref base) => {
                format!("{}[]", Self::export_type_definition(base))
            }
            TypeBase::Tuple(ref fields) => {
                format!(
                    "[{}]",
                    fields
                        .iter()
                        .map(|ty| Self::export_type_definition(ty))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypeBase::Custom(type_name) => type_name.to_string(),
            TypeBase::Unknown => "any".to_string(),
            _ => "any".to_string(),
        }
    }
}
