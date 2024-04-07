use async_graphql::Object;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentProject {
    pub id: Option<Thing>,
    pub path: String,
    pub updated: i64,
}

#[Object]
impl RecentProject {
    pub async fn id(&self) -> Option<String> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }

    pub async fn path(&self) -> &String {
        &self.path
    }

    pub async fn timestamp(&self) -> i64 {
        self.updated
    }
}
