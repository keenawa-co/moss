mod api;
mod config;
mod err;

pub mod infra;

pub use config::{Config, CONF};

use axum::{Extension, Router};
use err::Error;
use infra::surrealdb::inmem::SurrealInMem;
use tower::ServiceBuilder;
use tower_http::{
    compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, Predicate,
    },
    request_id::MakeRequestUuid,
    ServiceBuilderExt,
};

use crate::{
    api::service::UserService,
    api::{gql, graphql, status},
};

pub async fn init(
    // inmemdb: SurrealInMem,
    user_settings: Box<mosscore::config::preference_file::BehaverPreferenceFile>,
) -> Result<(), Error> {
    let conf = CONF.get().unwrap();

    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let user_service = UserService::init(user_settings);

    let service = service
        .layer(Extension(user_service.clone()))
        .layer(Extension(graphql::build_schema(user_service)))
        .layer(
            CompressionLayer::new().compress_when(
                SizeAbove::new(512) // don't compress below 512 bytes
                    .and(NotForContentType::IMAGES), // don't compress images
            ),
        );

    let router = Router::new().merge(status::router()).merge(gql::router());
    let router = Router::new().nest("/api/v1", router).layer(service);

    println!("Listening on {}", conf.bind);

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    return Ok(());
}
