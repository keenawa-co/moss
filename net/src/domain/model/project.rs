use async_graphql::{InputObject, SimpleObject};
use graphql_utl::path::Path as PathGraphQL;
use types::id::NanoId;

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
