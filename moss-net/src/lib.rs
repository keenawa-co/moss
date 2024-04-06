mod adapter;
mod domain;

mod api;
mod config;
mod err;

pub mod infra;

pub use config::{Config, CONF};

use axum::Extension;
use tower::ServiceBuilder;
use tower_http::{
    compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, Predicate,
    },
    request_id::MakeRequestUuid,
    ServiceBuilderExt,
};

use crate::domain::service::config_service::ConfigService;

pub async fn init(
    // inmemdb: SurrealInMem,
    user_settings: Box<moss_core::config::behaver_preference::BehaverPreferenceConfig>,
) -> Result<(), err::Error> {
    let conf = CONF.get().unwrap();

    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let config_service = ConfigService::new(user_settings);

    let service = service
        .layer(Extension(config_service.clone()))
        .layer(Extension(adapter::graphql::build_schema(config_service)))
        .layer(
            CompressionLayer::new().compress_when(
                SizeAbove::new(512) // don't compress below 512 bytes
                    .and(NotForContentType::IMAGES), // don't compress images
            ),
        );

    let router = infra::web::router(service);

    println!("Listening on {}", conf.bind);

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    return Ok(());
}
