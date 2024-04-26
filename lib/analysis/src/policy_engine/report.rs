use async_graphql::SimpleObject;

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
pub struct Report {
    pub source: String,
}
