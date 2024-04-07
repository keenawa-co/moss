mod config_query;
mod inspector_query;
mod portal_query;
mod project_query;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use self::{
    config_query::ConfigQuery, inspector_query::InspectorQuery, portal_query::PortalQuery,
    project_query::ProjectMutation,
};
use crate::domain::service::ServiceLocator;

#[derive(MergedObject, Default)]
pub struct QueryRoot(InspectorQuery, ConfigQuery, PortalQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutation);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(service_locator: &ServiceLocator) -> SchemaRoot {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(service_locator.config_service.clone())
    .data(service_locator.portal_service.clone())
    .data(service_locator.project_service.clone())
    .finish()
}
