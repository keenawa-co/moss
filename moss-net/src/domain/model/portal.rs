use async_graphql::Object;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RecentProject {
    pub id: Option<Thing>,
    pub path: String,
    pub updated: i64,
}

#[Object]
impl RecentProject {
    pub(crate) async fn id(&self) -> Option<String> {
        self.id.as_ref().map(|thing| thing.id.to_string())
    }

    pub(crate) async fn path(&self) -> &String {
        &self.path
    }

    pub(crate) async fn timestamp(&self) -> i64 {
        self.updated
    }
}
