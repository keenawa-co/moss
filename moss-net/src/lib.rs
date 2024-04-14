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

const MIX_COMPRESS_SIZE: u16 = 512;

use crate::{
    domain::service::{ConfigService, PortalService, ProjectService, ServiceLocator},
    infra::database::SQLiteClient,
};

pub async fn bind(_: TokioCancellationToken) -> Result<(), domain::Error> {
    let conf = CONF
        .get()
        .ok_or_else(|| domain::Error::Unknown("configuration was not defined".to_string()))?;

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

    let schema = infra::graphql::build_schema(&service_locator);
    let service = service.layer(
        CompressionLayer::new().compress_when(
            SizeAbove::new(MIX_COMPRESS_SIZE) // don't compress below 512 bytes
                .and(NotForContentType::IMAGES), // don't compress images
        ),
    );

    let router = infra::web::router(service, schema); // TODO: consider to use Cow<T>

    info!(
        "{} has been successfully launched on {}",
        moss_core::constant::APP_NAME,
        conf.bind
    );

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    Ok(())
}
