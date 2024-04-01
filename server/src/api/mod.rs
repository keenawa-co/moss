mod gql;
mod graphql;
mod status;

use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::{request_id::MakeRequestUuid, ServiceBuilderExt};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let service = service.layer(Extension(graphql::build_schema()));

    let router = Router::new()
        .merge(status::router())
        .merge(gql::router())
        .layer(service);

    // TODO: setup the graceful shutdown

    return router;
}
