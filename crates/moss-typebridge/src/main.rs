use moss_typebridge_macro::type_bridge;
use serde::Serialize;

#[type_bridge(language = "TypeScript", output_path = "CustomEnum.ts")]
enum CustomEnum {}

#[type_bridge(language = "TypeScript", output_path = "TestStruct.ts")]
struct TestStruct {
    #[serde(rename = "Integer")]
    integer: i32,
    idx: usize,
    float: f64,
    boolean: bool,
    character: char,
    string: String,
    option: Option<String>,
    vector: Vec<i32>,
    tuple: (String, i32, bool),
    complex: Vec<(Option<String>, i32)>,
    custom_enum: CustomEnum,
}

pub fn main() {}
