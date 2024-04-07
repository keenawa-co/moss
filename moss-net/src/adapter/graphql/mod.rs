mod config_query;
mod inspector_query;
mod portal_query;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use self::{
    config_query::ConfigQuery,
    inspector_query::InspectorQuery,
    portal_query::{PortalMutation, PortalQuery},
};
use crate::domain::service::ServiceLocator;

#[derive(MergedObject, Default)]
pub struct QueryRoot(InspectorQuery, ConfigQuery, PortalQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(PortalMutation);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(service_locator: &ServiceLocator) -> SchemaRoot {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(service_locator.config_service.clone())
    .data(service_locator.portal_service.clone())
    .finish()
}
