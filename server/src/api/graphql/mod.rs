use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn version(&self) -> &str {
        "1.0"
    }
}
