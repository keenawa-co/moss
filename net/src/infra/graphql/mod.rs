mod config_query;
mod explorer_query;
mod metric_query;
mod project_query;
mod session_query;

use async_graphql::{
    ErrorExtensionValues, ErrorExtensions, MergedObject, MergedSubscription, Schema,
};

use self::{
    config_query::ConfigQuery, explorer_query::ExplorerSubscription,
    metric_query::MetricSubscription, project_query::ProjectMutation,
    session_query::SessionMutation,
};
use crate::domain::{
    model::error::{Error, PreconditionError, ResourceError, SystemError},
    service::ServiceLocator,
};

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

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        let (summary, detail) = self.decompose();

        async_graphql::Error::new(summary.to_string()).extend_with(|_, e| {
            let extend = |e: &mut ErrorExtensionValues,
                          status_code: http::StatusCode,
                          error_code: &Option<String>| {
                if let Some(_detail) = detail {
                    e.set("detail", _detail);
                }

                if let Some(_error_code) = error_code {
                    e.set("error_code", _error_code);
                }

                e.set("status_code", status_code.as_u16());
            };

            match self {
                Error::Resource(err) => match err {
                    ResourceError::Invalid {
                        detail: _,
                        error_code,
                    } => extend(e, http::StatusCode::BAD_REQUEST, error_code),
                    ResourceError::NotFound {
                        detail: _,
                        error_code,
                    } => extend(e, http::StatusCode::NOT_FOUND, error_code),
                    ResourceError::Precondition(err) => match err {
                        PreconditionError::Required {
                            detail: _,
                            error_code,
                        } => extend(e, http::StatusCode::NOT_FOUND, error_code),
                        PreconditionError::Invalid {
                            detail: _,
                            error_code,
                        } => extend(e, http::StatusCode::NOT_FOUND, error_code),
                    },
                },

                Error::System(err) => match err {
                    SystemError::Unexpected {
                        detail: _,
                        error_code,
                    } => extend(e, http::StatusCode::INTERNAL_SERVER_ERROR, error_code),

                    _ => (),
                },

                Error::Config(_) => unreachable!(),
            }
        })
    }
}
