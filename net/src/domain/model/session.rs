use async_graphql::{InputObject, SimpleObject};
use common::id::MNID;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct Session {
    pub id: MNID,
    pub project_id: MNID,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct CreateSessionInput {
    pub project_id: MNID,
}
