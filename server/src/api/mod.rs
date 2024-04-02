mod gql;
mod graphql;
mod status;

use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::{
    compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, Predicate,
    },
    request_id::MakeRequestUuid,
    ServiceBuilderExt,
};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let service = service.layer(Extension(graphql::build_schema())).layer(
        CompressionLayer::new().compress_when(
            SizeAbove::new(512) // don't compress below 512 bytes
                .and(NotForContentType::IMAGES), // don't compress images
        ),
    );

    let router = Router::new()
        .merge(status::router())
        .merge(gql::router())
        .layer(service);

    // TODO: setup the graceful shutdown

    return router;
}
