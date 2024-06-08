mod config_query;
mod metric_query;
mod notification_query;
mod project_query;
mod session_query;

use std::sync::Arc;

use async_graphql::{
    ErrorExtensionValues, ErrorExtensions, MergedObject, MergedSubscription, Schema,
};
use tokio::sync::RwLock;

use self::{
    config_query::ConfigQuery,
    metric_query::MetricSubscription,
    notification_query::NotificationSubscription,
    project_query::{ProjectMutation, ProjectSubscription},
    session_query::{SessionMutation, SessionQuery},
};
use crate::domain::{
    model::error::{Error, PreconditionError, ResourceError, SystemError},
    service::ServiceRoot,
};

#[derive(MergedObject)]
pub struct QueryRoot(SessionQuery, ConfigQuery);

#[derive(MergedObject)]
pub struct MutationRoot<'a>(ProjectMutation<'a>, SessionMutation<'a>);

#[derive(MergedSubscription)]
pub struct SubscriptionRoot<'a>(
    ProjectSubscription<'a>,
    MetricSubscription<'a>,
    NotificationSubscription,
);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot<'static>, SubscriptionRoot<'static>>;

pub fn build_schema(service_root: Arc<ServiceRoot>) -> SchemaRoot {
    Schema::build(
        QueryRoot(
            SessionQuery {
                session_service: Arc::clone(&service_root.3),
            },
            ConfigQuery {
                config_service: Arc::clone(&service_root.0),
            },
        ),
        MutationRoot(
            ProjectMutation {
                project_meta_service: Arc::clone(&service_root.1),
                project_service: Arc::clone(&service_root.5),
                notification_service: Arc::clone(&service_root.4),
            },
            SessionMutation {
                session_service: Arc::clone(&service_root.3),
                project_service: Arc::clone(&service_root.5),
                workspace_service: Arc::clone(&service_root.6),
            },
        ),
        SubscriptionRoot(
            ProjectSubscription {
                project_service: service_root.5.clone(),
            },
            MetricSubscription {
                project_service: service_root.5.clone(),
            },
            NotificationSubscription {
                notification_service: service_root.4.clone(),
            },
        ),
    )
    .finish()
}

//TODO: move to pkg module
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
