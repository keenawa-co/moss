pub enum ConfigurationScope {
    Platform,
    Machine,
    Window,
    Resource,
}

pub struct ConfigurationRegistry {}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Copy, Clone)]
enum N {
    U64(u64),
    I64(i64),
    F64(f64),
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::U64(a), N::U64(b)) => a == b,
            (N::I64(a), N::I64(b)) => a == b,
            (N::F64(a), N::F64(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Number {
    this: N,
}

pub enum ConfigurationNodeType {
    String,
    Bool,
    Number,
    Array,
    Object,
}

pub struct ConfigurationSchemaProperty {
    scope: ConfigurationScope,
}

// type PropertiesDictionary = hashbrown::HashMap<String, >

pub struct ConfigurationNode {
    id: Option<String>,
    order: Option<usize>,
    typ: ConfigurationNodeType,
    title: Option<String>,
    description: Option<String>,
}
