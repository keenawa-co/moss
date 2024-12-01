use moss_typebridge_macro::type_bridge;
use serde::Serialize;

#[type_bridge]
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
}

pub fn main() {}
