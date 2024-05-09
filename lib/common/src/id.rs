use anyhow::anyhow;
use nanoid::nanoid;
use serde::Serializer;
use std::marker::PhantomData;
use std::ops::Deref;

#[cfg(feature = "graphql")]
use async_graphql::{Scalar, ScalarType};

const NANO_ID20: usize = 20;
const CHAR_SET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// TODO: implement all traits to use MNID as a SEA ORM model type

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoId(BoundedString<NANO_ID20>);

impl NanoId {
    pub fn new() -> Self {
        let id = BoundedString::new(&nanoid!(NANO_ID20, &CHAR_SET)).unwrap();
        NanoId(id)
    }
}

impl std::fmt::Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.inner)
    }
}

impl Into<sea_orm::Value> for NanoId {
    fn into(self) -> sea_orm::Value {
        sea_orm::Value::String(Some(Box::new(self.0.to_string())))
    }
}

impl From<&str> for NanoId {
    fn from(value: &str) -> Self {
        NanoId(BoundedString::new(value).unwrap())
    }
}

impl From<String> for NanoId {
    fn from(value: String) -> Self {
        NanoId(BoundedString::new(value).unwrap())
    }
}

impl AsRef<str> for NanoId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for NanoId {
    fn into(self) -> String {
        self.0.inner
    }
}

#[cfg(feature = "graphql")]
#[Scalar]
impl ScalarType for NanoId {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            BoundedString::<20>::new(value)
                .map(NanoId)
                .map_err(|e| async_graphql::InputValueError::custom(e.to_string()))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.inner.clone())
    }
}

pub mod nanoid_serde {
    use std::fmt;

    use serde::{
        de::{self, Visitor},
        Deserializer,
    };

    use super::*;

    pub fn serialize<S>(value: &NanoId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.0.inner)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NanoId, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> Visitor<'de> for StringVisitor {
            type Value = NanoId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NanoId(BoundedString {
                    inner: v.to_string(),
                    _marker: PhantomData,
                }))
            }
        }

        deserializer.deserialize_string(StringVisitor)
    }
}

// Bounded String implementation

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedString<const N: usize> {
    inner: String,

    #[serde(skip)]
    _marker: PhantomData<[u8; N]>,
}

impl<const N: usize> BoundedString<N> {
    pub fn new<S: AsRef<str>>(input: S) -> anyhow::Result<Self> {
        let input_ref = input.as_ref();
        if input_ref.chars().count() <= N {
            Ok(BoundedString {
                inner: input_ref.to_string(),
                _marker: PhantomData,
            })
        } else {
            Err(anyhow!("invalid id format, allowed length is {N}"))
        }
    }
}

impl<const N: usize> Deref for BoundedString<N> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
