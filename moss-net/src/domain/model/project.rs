use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct Project {
    pub id: String,
    pub source: String,
    pub last_used_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct RecentProject {
    pub id: String,
    pub source: String,
    pub last_used_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct NewProjectInput {
    pub path: String,
}
