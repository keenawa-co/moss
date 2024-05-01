mod domain;
mod infra;

pub mod config;

pub use config::{Config, CONF};
pub use infra::graphql::sdl;

use analysis::policy_engine::PolicyEngine;
use bus::topic::TopicConfig;
use common::APP_NAME;
use fs::{fw::FileWatcher, real, FS};
use std::sync::Arc;
use tokio::sync::RwLock;
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
        ConfigService, MetricService, ProjectMetaService, ServiceLocator, SessionService,
    },
    infra::database::sqlite::RootDatabaseClient,
};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

const MIX_COMPRESS_SIZE: u16 = 512; // TODO: this value should be used from a net_conf.toml file

pub async fn bind(_: TokioCancellationToken) -> Result<(), domain::Error> {
    let conf = CONF
        .get()
        .ok_or_else(|| internal!("configuration was not defined"))?;

    let b = bus::Bus::new();

    let realfs = Arc::new(real::FileSystem::new());
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

    let sqlite_db = RootDatabaseClient::new(conf.conn.clone());
    let service_locator = ServiceLocator {
        session_service: RwLock::new(SessionService::new(
            realfs.clone(),
            sqlite_db.session_repo(),
            sqlite_db.project_meta_repo(),
        )),
        config_service: ConfigService::new(conf.preference.clone()),
        project_meta_service: ProjectMetaService::new(sqlite_db.project_meta_repo()),
        metric_service: MetricService::new(Arc::new(pe)),
        project_service: RwLock::new(None),
    };

    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let schema = infra::graphql::build_schema(service_locator);
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
