pub mod config;
mod domain;
mod infra;

use analysis::policy_engine::PolicyEngine;
pub use config::{Config, CONF};
use dl::APP_NAME;
use fs::fw::FileWatcher;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

use std::{rc::Rc, sync::Arc};
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

use crate::{
    domain::service::{
        ConfigService, MetricService, PortalService, ProjectService, ServiceLocator,
    },
    infra::database::SQLiteClient,
};

const MIX_COMPRESS_SIZE: u16 = 512; // TODO: this value should be used from a net_conf.toml file

pub async fn bind(_: TokioCancellationToken) -> Result<(), domain::Error> {
    let conf = CONF
        .get()
        .ok_or_else(|| domain::Error::Unknown("configuration was not defined".to_string()))?;

    let fw = FileWatcher::new();

    let b = bus::Bus::new();
    b.create_topic("general").await;
    b.subscribe_topic::<String>("general", fw.clone()).await;

    let pe = PolicyEngine::new(fw.clone(), b);

    let sqlite_db = SQLiteClient::new(conf.conn.clone());
    let service_locator = ServiceLocator {
        portal_service: Arc::new(PortalService::new(sqlite_db.project_repo())),
        config_service: Arc::new(ConfigService::new(conf.preference.clone())),
        project_service: Arc::new(ProjectService::new(sqlite_db.project_repo())),
        metric_service: Arc::new(MetricService::new(Arc::new(pe))),
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
        APP_NAME, conf.bind
    );

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    Ok(())
}
