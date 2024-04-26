pub mod config;
mod domain;
mod infra;

use analysis::policy_engine::PolicyEngine;
use bus::topic::TopicConfig;
use common::APP_NAME;
pub use config::{Config, CONF};
use fs::{fw::FileWatcher, real, FS};
use tokio_stream::StreamExt;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

use std::{path::Path, sync::Arc, time::Duration};
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

    let b = bus::Bus::new();

    let real_fs = real::FileSystem::new();
    // let watch_stream = rfs
    //     .watch(
    //         Path::new("./testdata/helloworld.ts"),
    //         Duration::from_secs(1),
    //     )
    //     .await;

    // let mut stream = Box::pin(watch_stream);
    // while let Some(paths) = stream.next().await {
    //     dbg!(paths);
    // }

    let fw = FileWatcher::new(b.clone());

    b.create_topic("general", TopicConfig::default()).await;
    b.subscribe_topic::<String>("general", fw.clone()).await?;

    let pe = PolicyEngine::new(fw.clone(), b);

    let sqlite_db = SQLiteClient::new(conf.conn.clone());
    let service_locator = ServiceLocator {
        portal_service: Arc::new(PortalService::new(sqlite_db.project_repo())),
        config_service: Arc::new(ConfigService::new(conf.preference.clone())),
        project_service: Arc::new(ProjectService::new(
            Arc::new(real_fs),
            sqlite_db.project_repo(),
        )),
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
