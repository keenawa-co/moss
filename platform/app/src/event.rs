use std::{borrow::Cow, io};

use async_graphql::{Enum, Scalar, ScalarType, SimpleObject};

use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq)]
pub struct PlatformAction<'a> {
    workspace: Cow<'a, str>,
    partition: Cow<'a, str>,
    operation: Cow<'a, str>,
}

impl<'a> ToString for PlatformAction<'a> {
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.workspace, self.partition, self.operation)
    }
}

impl<'a> Serialize for PlatformAction<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'a> TryFrom<&str> for PlatformAction<'a> {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let segments: Vec<&str> = value.split(':').collect();

        // TODO: implement FieldCounter proc macros to count struct fields automatically
        if segments.len() != 3 {
            return Err(std::io::Error::new(
                io::ErrorKind::Other,
                "Invalid format, expected '{workspace}:{partition}:{operation}'",
            ));
        }

        Ok(PlatformAction {
            workspace: Cow::Owned(segments[0].to_string()),
            partition: Cow::Owned(segments[1].to_string()),
            operation: Cow::Owned(segments[2].to_string()),
        })
    }
}

#[Scalar]
impl<'a> ScalarType for PlatformAction<'a> {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            match PlatformAction::try_from(s.as_str()) {
                Ok(v) => Ok(v),
                Err(e) => Err(async_graphql::InputValueError::custom(e)),
            }
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.to_string())
    }
}

impl<'a> PlatformAction<'a> {
    pub fn new<'b: 'a>(workspace: &'b str, partition: &'b str, operation: &'b str) -> Self {
        Self {
            workspace: Cow::Borrowed(workspace),
            partition: Cow::Borrowed(partition),
            operation: Cow::Borrowed(operation),
        }
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum Severity {
    #[graphql(name = "hint")]
    Hint,
    #[graphql(name = "info")]
    Info,
    #[graphql(name = "warn")]
    Warn,
    #[graphql(name = "error")]
    Error,
}

#[derive(Debug, SimpleObject)]
pub struct PlatformEvent<'a> {
    pub action: PlatformAction<'a>,
    pub data: serde_json::Value,
    pub severity: Severity,
    pub timestamp: i64,
}
