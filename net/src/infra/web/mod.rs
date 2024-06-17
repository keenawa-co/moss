pub mod gql;
pub mod status;

use app::context::AppContext;
use axum::{extract::Request, response::IntoResponse, routing::Route, Extension};
use std::convert::Infallible;

use crate::infra::graphql::SchemaRoot;

pub fn router<L>(ctx: &AppContext, service: L, schema: SchemaRoot) -> axum::Router
where
    L: tower::Layer<Route> + Clone + Send + 'static,
    L::Service: tower::Service<Request> + Clone + Send + 'static,
    <L::Service as tower::Service<Request>>::Response: IntoResponse + 'static,
    <L::Service as tower::Service<Request>>::Error: Into<Infallible> + 'static,
    <L::Service as tower::Service<Request>>::Future: Send + 'static,
{
    let router = axum::Router::new()
        .merge(status::router())
        .merge(gql::router());

    axum::Router::new()
        .nest("/api", router)
        .layer(service)
        .layer(Extension(schema))
        .layer(Extension(ctx.into_async()))
}
