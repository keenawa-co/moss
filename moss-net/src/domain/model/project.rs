use async_graphql::{InputObject, Object};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: Option<Thing>,
    pub path: String,
    pub updated: i64,
}

#[Object]
impl Project {
    pub async fn id(&self) -> Option<String> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }

    pub async fn path(&self) -> &String {
        &self.path
    }

    pub async fn updated(&self) -> i64 {
        self.updated
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct NewProjectInput {
    pub path: String,
}
