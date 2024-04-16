use async_graphql::SimpleObject;

#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)] // TODO: use cfg derive for SimpleObject
pub struct FeedItem {
    pub source: String,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedUpdateInput {
    pub session_id: String,
    pub project_id: String,
    // pub feed: MetricFeedItem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub session_id: String,
    pub project_id: String,
    // pub feed: MetricFeedItem,
    pub chunk_num: usize,
    pub chunk_total: usize,
    pub created_at: i64,
}
