use async_graphql::{InputObject, SimpleObject};
use common::id::NanoId;
use gqlutl::path::Path as PathGraphQL;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct ProjectMeta {
    pub id: NanoId,
    pub source: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct CreateProjectInput {
    pub path: PathGraphQL,
    pub ignore_list: Option<Vec<String>>,
}
