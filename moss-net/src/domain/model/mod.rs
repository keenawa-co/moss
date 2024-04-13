pub mod project;

use async_graphql::SimpleObject;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct RecordObject<T>
where
    T: std::marker::Sync + async_graphql::OutputType,
{
    pub(crate) id: T,
}

impl<T> RecordObject<T>
where
    T: std::marker::Sync + async_graphql::OutputType,
{
    pub(crate) fn new(id: T) -> Self {
        RecordObject { id }
    }
}
