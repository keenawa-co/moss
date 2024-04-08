use async_graphql::{InputObject, Object};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Project {
    pub id: Option<Thing>,
    pub path: String,
    pub updated: i64,
}

#[Object]
impl Project {
    pub(crate) async fn id(&self) -> Option<String> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }

    pub(crate) async fn path(&self) -> &String {
        &self.path
    }

    pub(crate) async fn updated(&self) -> i64 {
        self.updated
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct NewProjectInput {
    pub path: String,
}
