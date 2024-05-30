use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct AbstractEvent {
    pub route: String,
    pub data: serde_json::Value,
}
