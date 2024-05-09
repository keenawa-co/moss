mod domain;
mod infra;

pub mod config;

pub use config::{Config, CONF};
use domain::model::error::Error;
pub use infra::graphql::sdl;

use analysis::policy_engine::PolicyEngine;
use bus::topic::TopicConfig;
use common::APP_NAME;
use fs::{fw::FileWatcher, real, FS};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, RwLock};
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
    domain::{
        model::OptionExtension,
        service::{
            config_service::ConfigService,
            metric_service::MetricService,
            notification_service::NotificationService,
            project_meta_service::ProjectMetaService,
            session_service::{SessionService, SessionServiceConfig},
            ServiceHub,
        },
    },
    infra::adapter::sqlite::RootSQLiteAdapter,
};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

const MIX_COMPRESS_SIZE: u16 = 512; // TODO: this value should be used from a net_conf.toml file

pub async fn bind(_: TokioCancellationToken) -> Result<(), Error> {
    let conf = CONF
        .get()
        .ok_or_config_invalid("Configuration was not defined", None)?;

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

    let root_db = RootSQLiteAdapter::new(Arc::clone(&conf.conn));
    let service_hub = ServiceHub {
        session_service: RwLock::new(SessionService::new(
            root_db.session_repo(),
            root_db.project_meta_repo(),
            SessionServiceConfig {
                project_dir: PathBuf::from(".moss/cache"), // FIXME: This value must be obtained from the configuration file
                project_db_file: PathBuf::from("cache.db"), // FIXME: This value must be obtained from the configuration file
            },
        )),
        config_service: ConfigService::new(conf.preference.clone()),
        project_meta_service: ProjectMetaService::new(realfs.clone(), root_db.project_meta_repo()),
        metric_service: MetricService::new(Arc::new(pe)),
        project_service: RwLock::new(None),
        notification_service: NotificationService::new(),
    };

    let service = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id();

    let schema = infra::graphql::build_schema(service_hub);
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
