pub mod config;
mod domain;
mod infra;

pub use config::{Config, CONF};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

use axum::Extension;
use std::sync::Arc;
use tokio_util::sync::CancellationToken as TokioCancellationToken;
use tower::ServiceBuilder;
use tower_http::{
    compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, Predicate,
    },
    request_id::MakeRequestUuid,
    ServiceBuilderExt,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{
    domain::service::{ConfigService, PortalService, ProjectService, ServiceLocator},
    infra::database::SQLiteClient,
};

pub async fn bind(_: TokioCancellationToken) -> Result<(), domain::Error> {
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let conf = CONF
        .get()
        .ok_or_else(|| domain::error_configuration("configuration was not defined"))?;

    let sqlite_db = SQLiteClient::new(conf.conn.clone());
    let service_locator = ServiceLocator {
        portal_service: Arc::new(PortalService::new(sqlite_db.project_repo())),
        config_service: Arc::new(ConfigService::new(conf.preference.clone())),
        project_service: Arc::new(ProjectService::new(sqlite_db.project_repo())),
    };

    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let service = service
        .layer(Extension(infra::graphql::build_schema(&service_locator)))
        .layer(
            CompressionLayer::new().compress_when(
                SizeAbove::new(512) // don't compress below 512 bytes
                    .and(NotForContentType::IMAGES), // don't compress images
            ),
        );

    let router = infra::web::router(service); // TODO: consider to use Cow<T>

    warn!(
        "{} has been successfully launched on {}",
        moss_core::constant::APP_NAME,
        conf.bind
    );

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    Ok(())
}
