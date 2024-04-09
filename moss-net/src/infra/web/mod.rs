pub mod gql;
pub mod status;

use axum::{extract::Request, response::IntoResponse, routing::Route};
use std::convert::Infallible;

pub fn router<L>(service: L) -> axum::Router
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

    axum::Router::new().nest("/api/v1", router).layer(service)
}