mod inspector_query;
mod user_query;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::api::service::UserService;

use self::{inspector_query::InspectorQuery, user_query::UserQuery};

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(InspectorQuery, UserQuery);

pub(crate) type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) fn build_schema(us: UserService) -> SchemaRoot {
    return Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(Arc::new(us))
        .finish();
}
