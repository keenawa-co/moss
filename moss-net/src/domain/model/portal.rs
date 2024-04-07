use async_graphql::{InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Portal {
    recent: Vec<RecentItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentItem {
    pub id: Option<Thing>,
    pub path: String,
    pub timestamp: i64,
}

#[Object]
impl RecentItem {
    pub async fn id(&self) -> Option<String> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }

    pub async fn path(&self) -> &String {
        &self.path
    }

    pub async fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct RecentItemInput {
    pub path: String,
}
