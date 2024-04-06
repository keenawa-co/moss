use async_graphql::{InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct Portal {
    recent: Vec<RecentItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentItem {
    pub id: Thing,
    pub path: String,
}

#[Object]
impl RecentItem {
    pub async fn id(&self) -> String {
        self.id.id.to_string()
    }

    pub async fn path(&self) -> &String {
        &self.path
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct RecentItemInput {
    pub path: String,
}
