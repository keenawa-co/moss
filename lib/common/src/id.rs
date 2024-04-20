use anyhow::{anyhow, Ok};
use nanoid::nanoid;
use std::marker::PhantomData;
use std::ops::Deref;

#[cfg(feature = "graphql")]
use async_graphql::{Scalar, ScalarType};

const MNID20: usize = 20;

const CHAR_SET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// TODO: implement all traits to use MNID as a SEA ORM model type

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MNID(BoundedString<MNID20>);

impl MNID {
    pub fn new() -> Self {
        let id = BoundedString::new(&nanoid!(MNID20, &CHAR_SET)).unwrap();
        MNID(id)
    }
}

impl std::fmt::Display for MNID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.inner)
    }
}

impl From<&str> for MNID {
    fn from(value: &str) -> Self {
        MNID(BoundedString::new(value).unwrap())
    }
}

impl From<String> for MNID {
    fn from(value: String) -> Self {
        MNID(BoundedString::new(value).unwrap())
    }
}

impl AsRef<str> for MNID {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for MNID {
    fn into(self) -> String {
        self.0.inner
    }
}

#[cfg(feature = "graphql")]
#[Scalar]
impl ScalarType for MNID {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            BoundedString::<20>::new(value)
                .map(MNID)
                .map_err(|e| async_graphql::InputValueError::custom(e.to_string()))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.inner.clone())
    }
}

// Bounded String implementation

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedString<const N: usize> {
    inner: String,
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
