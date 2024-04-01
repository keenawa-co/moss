use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::api::graphql::QueryRoot;

pub type ApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn graphql_schema() -> ApiSchema {
    return Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish();
}
