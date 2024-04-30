use async_graphql::{InputObject, SimpleObject};
use common::id::NanoId;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Project {
    pub id: NanoId,
    pub source: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct CreateProjectInput {
    pub path: String,
    pub ignore_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct WatchProjectInput {
    pub project_id: NanoId,
    pub ignore_list: Option<Vec<String>>,
}
