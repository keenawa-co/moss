use anyhow::{anyhow, Result};
use hcl::{expr::Variable, Expression};
use serde_json::Value as JsonValue;

pub trait Typeable {
    const NAME: &'static str;

    fn data() -> &'static TypeData;
}

#[derive(Debug)]
pub struct TypeData {
    pub name: &'static str,
}

#[derive(Debug)]
pub struct Type(&'static TypeData);

impl Type {
    pub fn of<T: Typeable>() -> Self {
        Type(T::data())
    }

    pub fn data(&self) -> &'static TypeData {
        self.0
    }
}

impl TryFrom<Expression> for Type {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Array(_vec) => unimplemented!(),
            Expression::Object(_vec_map) => unimplemented!(),
            Expression::Variable(variable) => Type::try_from(variable),
            _ => Err(anyhow!("unknown type")),
        }
    }
}

impl TryFrom<Variable> for Type {
    type Error = anyhow::Error;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            TypeNumber::NAME => Ok(Type::of::<TypeNumber>()),
            TypeString::NAME => Ok(Type::of::<TypeString>()),
            TypeBoolean::NAME => Ok(Type::of::<TypeBoolean>()),
            _ => Err(anyhow!("unknown type")),
        }
    }
}

pub fn default_json_value(typ: &Type) -> Result<JsonValue> {
    match typ.data().name {
        TypeNumber::NAME => Ok(0.into()),
        TypeString::NAME => Ok("".into()),
        TypeBoolean::NAME => Ok(false.into()),
        _ => Err(anyhow!("unknown type")),
    }
}

pub struct TypeNumber;

impl Typeable for TypeNumber {
    const NAME: &'static str = "number";

    fn data() -> &'static TypeData {
        &TypeData { name: Self::NAME }
    }
}

pub struct TypeString;

impl Typeable for TypeString {
    const NAME: &'static str = "string";

    fn data() -> &'static TypeData {
        &TypeData { name: Self::NAME }
    }
}

pub struct TypeBoolean;

impl Typeable for TypeBoolean {
    const NAME: &'static str = "bool";

    fn data() -> &'static TypeData {
        &TypeData { name: Self::NAME }
    }
}
