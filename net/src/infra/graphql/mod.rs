mod config_query;
mod explorer_query;
mod metric_query;
mod project_query;
mod session_query;

use async_graphql::{MergedObject, MergedSubscription, Schema};

use self::{
    config_query::ConfigQuery, explorer_query::ExplorerSubscription,
    metric_query::MetricSubscription, project_query::ProjectMutation,
    session_query::SessionMutation,
};
use crate::domain::service::ServiceLocator;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ConfigQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutation, SessionMutation);

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
    .data(service_locator.project_meta_service)
    .data(service_locator.project_service)
    .data(service_locator.metric_service)
    .data(service_locator.session_service)
    .finish()
}

pub fn sdl() -> String {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        RootSubscription::default(),
    )
    .finish()
    .sdl()
}
