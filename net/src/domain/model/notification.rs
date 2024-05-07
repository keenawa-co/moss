use async_graphql::SimpleObject;
use common::id::NanoId;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Notification {
    pub id: NanoId,
    pub project_id: NanoId,
    pub session_id: NanoId,
    pub summary: String,
}
