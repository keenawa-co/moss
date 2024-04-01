mod inspector_query;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use self::inspector_query::InspectorQuery;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(InspectorQuery);

pub(crate) type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) fn build_schema() -> SchemaRoot {
    return Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish();
}
