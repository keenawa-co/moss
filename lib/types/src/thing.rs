#[cfg(feature = "graphql")]
use async_graphql::SimpleObject;

use crate::id::NanoId;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
pub struct Thing {
    pub id: String,
}

impl From<String> for Thing {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}

impl From<NanoId> for Thing {
    fn from(value: NanoId) -> Self {
        Self {
            id: value.to_string(),
        }
    }
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
