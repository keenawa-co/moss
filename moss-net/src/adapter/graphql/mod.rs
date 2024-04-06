mod config_query;
mod inspector_query;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::domain::service::ConfigService;

use self::{config_query::ConfigQuery, inspector_query::InspectorQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(InspectorQuery, ConfigQuery);

pub type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(config_service: Arc<ConfigService>) -> SchemaRoot {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(config_service)
        .finish()
}
