mod config_query;
mod inspector_query;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::domain::service::ConfigService;

use self::{config_query::ConfigQuery, inspector_query::InspectorQuery};

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(InspectorQuery, ConfigQuery);

pub(crate) type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) fn build_schema(config_service: Arc<ConfigService>) -> SchemaRoot {
    return Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(config_service)
        .finish();
}
