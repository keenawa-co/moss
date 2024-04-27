mod config_query;
mod explorer_query;
mod metric_query;
mod portal_query;
mod project_query;

use async_graphql::{MergedObject, MergedSubscription, Schema};
use std::rc::Rc;

use self::{
    config_query::ConfigQuery, explorer_query::ExplorerSubscription,
    metric_query::MetricSubscription, portal_query::PortalQuery, project_query::ProjectMutation,
};
use crate::domain::service::ServiceLocator;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ConfigQuery, PortalQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutation);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(ExplorerSubscription, MetricSubscription);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, RootSubscription>;

pub fn build_schema(service_locator: ServiceLocator) -> SchemaRoot {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        RootSubscription::default(),
    )
    .data(service_locator.config_service)
    .data(service_locator.portal_service)
    .data(service_locator.project_service)
    .data(service_locator.metric_service)
    .data(service_locator.context_service)
    .finish()
}
