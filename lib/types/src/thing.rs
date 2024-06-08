use serde::{Deserialize, Serialize};

#[cfg(feature = "graphql")]
use async_graphql::{OutputType, SimpleObject};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
pub struct Thing<T>
where
    T: std::fmt::Display + Clone + PartialEq + PartialOrd + std::fmt::Debug,
    T: OutputType,
{
    pub id: T,
}

impl<T> Thing<T>
where
    T: std::fmt::Display + Clone + PartialEq + PartialOrd + std::fmt::Debug,
    T: OutputType,
{
    #[must_use]
    pub fn new(id: T) -> Thing<T> {
        Self { id }
    }
}
