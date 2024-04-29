use async_graphql::{InputObject, SimpleObject};
use common::id::MNID;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Project {
    pub id: MNID,
    pub source: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct CreateProjectInput {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct WatchProjectInput {
    pub project_id: MNID,
    pub ignore_list: Option<Vec<String>>,
}
