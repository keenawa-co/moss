use hashbrown::HashSet;
use types::id::NanoId;

#[cfg(feature = "graphql")]
use async_graphql::SimpleObject;

#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct IgnoredSource {
    pub id: NanoId,
    pub source: String,
}

pub trait ToHashSet {
    fn to_hash_set(self) -> HashSet<IgnoredSource>;
}

impl ToHashSet for Vec<IgnoredSource> {
    fn to_hash_set(self) -> HashSet<IgnoredSource> {
        HashSet::from_iter(self.into_iter())
    }
}
