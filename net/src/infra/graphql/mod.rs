mod config_query;
mod explorer_query;
mod metric_query;
mod notification_query;
mod project_query;
mod session_query;

use async_graphql::{
    ErrorExtensionValues, ErrorExtensions, MergedObject, MergedSubscription, Schema,
};

use self::{
    config_query::ConfigQuery, explorer_query::ExplorerSubscription,
    metric_query::MetricSubscription, notification_query::NotificationSubscription,
    project_query::ProjectMutation, session_query::SessionMutation,
};
use crate::domain::{
    model::error::{Error, PreconditionError, ResourceError, SystemError},
    service::ServiceHub,
};

#[derive(MergedObject)]
pub struct QueryRoot(ConfigQuery);

#[derive(MergedObject)]
pub struct MutationRoot(ProjectMutation, SessionMutation);

#[derive(MergedSubscription)]
pub struct RootSubscription(
    ExplorerSubscription,
    MetricSubscription,
    NotificationSubscription,
);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, RootSubscription>;

pub fn build_schema(service_hub: ServiceHub) -> SchemaRoot {
    Schema::build(
        QueryRoot(ConfigQuery {
            config_service: service_hub.0,
        }),
        MutationRoot(
            ProjectMutation {
                project_meta_service: service_hub.1.clone(),
                project_service: service_hub.5.clone(),
                notification_service: service_hub.4.clone(),
            },
            SessionMutation {
                session_service: service_hub.3.clone(),
                project_service: service_hub.5.clone(),
            },
        ),
        RootSubscription(
            ExplorerSubscription::default(),
            MetricSubscription {
                metric_service: service_hub.2,
            },
            NotificationSubscription {
                notification_service: service_hub.4.clone(),
            },
        ),
    )
    .finish()
}

// pub fn sdl() -> String {
//     Schema::build(
//         QueryRoot::default(),
//         MutationRoot::default(),
//         RootSubscription::default(),
//     )
//     .finish()
//     .sdl()
// }

impl Error {
    fn extend_graphql_error(
        e: &mut ErrorExtensionValues,
        detail: &Option<String>,
        status_code: http::StatusCode,
        error_code: &Option<String>,
    ) {
        if let Some(_detail) = detail {
            e.set("detail", _detail);
        }

        if let Some(_error_code) = error_code {
            e.set("error_code", _error_code);
        }

        e.set("status_code", status_code.as_u16());
    }

    fn with_graphql_error(&self, e: &mut ErrorExtensionValues, detail: Option<String>) {
        match self {
            Error::Resource(err) => match err {
                ResourceError::Invalid {
                    detail: _,
                    error_code,
                } => Error::extend_graphql_error(
                    e,
                    &detail,
                    http::StatusCode::BAD_REQUEST,
                    &error_code,
                ),
                ResourceError::NotFound {
                    detail: _,
                    error_code,
                } => Error::extend_graphql_error(
                    e,
                    &detail,
                    http::StatusCode::NOT_FOUND,
                    &error_code,
                ),
                ResourceError::Precondition(err) => match err {
                    PreconditionError::Required {
                        detail: _,
                        error_code,
                    } => Error::extend_graphql_error(
                        e,
                        &detail,
                        http::StatusCode::PRECONDITION_FAILED,
                        &error_code,
                    ),
                    PreconditionError::Invalid {
                        detail: _,
                        error_code,
                    } => Error::extend_graphql_error(
                        e,
                        &detail,
                        http::StatusCode::PRECONDITION_FAILED,
                        &error_code,
                    ),
                },
            },
            Error::System(err) => match err {
                SystemError::Unexpected {
                    detail: _,
                    error_code,
                } => Error::extend_graphql_error(
                    e,
                    &detail,
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    &error_code,
                ),

                _ => (),
            },

            Error::Config(_) => unreachable!(),
        }
    }
}

impl From<Error> for async_graphql::ServerError {
    fn from(value: Error) -> Self {
        let (summary, detail) = value.decompose();

        let e = async_graphql::ServerError::new(summary, None);
        e.extend_with(|_, e| {
            Error::with_graphql_error(&value, e, detail);
        });
        e
    }
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        let (summary, detail) = self.decompose();

        async_graphql::Error::new(summary.to_string())
            .extend_with(|_, e| Error::with_graphql_error(self, e, detail))
    }
}
