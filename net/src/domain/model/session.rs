use async_graphql::{InputObject, SimpleObject};
use common::id::MNID;

pub(crate) struct Session {
    pub id: MNID,
    pub project_id: MNID,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct CreateSessionInput {
    pub project_id: MNID,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct CreateSessionOutput {
    pub session_id: MNID,
}

impl From<Session> for CreateSessionOutput {
    fn from(value: Session) -> Self {
        CreateSessionOutput {
            session_id: value.id,
        }
    }
}
