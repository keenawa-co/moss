use async_graphql::{InputObject, SimpleObject};
use common::id::MNID;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct Project {
    pub id: MNID,
    pub source: String,
    pub last_used_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct RecentProject {
    pub id: MNID,
    pub source: String,
    pub last_used_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct NewProjectInput {
    pub path: String,
}
