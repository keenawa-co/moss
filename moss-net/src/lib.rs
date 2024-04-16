pub mod config;
mod domain;
mod infra;

pub use config::{Config, CONF};
use tokio::sync::broadcast;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tracing;

use std::{path::Path, sync::Arc};
use tokio::time::{self, Duration};
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
    domain::service::{
        ConfigService, MetricService, PortalService, ProjectService, ServiceLocator,
    },
    infra::database::SQLiteClient,
};

pub async fn bind(_: TokioCancellationToken) -> Result<(), domain::Error> {
    let conf = CONF
        .get()
        .ok_or_else(|| domain::Error::Unknown("configuration was not defined".to_string()))?;

    let (txRaw, _) = broadcast::channel::<String>(16);
    let tx = Arc::new(txRaw);

    let sqlite_db = SQLiteClient::new(conf.conn.clone());
    let service_locator = ServiceLocator {
        portal_service: Arc::new(PortalService::new(sqlite_db.project_repo())),
        config_service: Arc::new(ConfigService::new(conf.preference.clone())),
        project_service: Arc::new(ProjectService::new(sqlite_db.project_repo())),
        metric_service: Arc::new(MetricService::new(tx.clone())),
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

    // moss_core::runtime::isolate::lua_call().unwrap();

    let scheduler = Arc::new(was::scheduler::Scheduler {
        session_id: "mock_session_id".to_string(),
        project_id: "mock_project_id".to_string(),
        // session_id: "session123".to_string(),
        // context: lua,
        // storage,
        tx: tx.clone(),
    });

    let scheduler_clone = scheduler.clone();
    tokio::spawn(async move {
        let data = vec!["World", "Rust", "Lua"];
        time::sleep(Duration::from_secs(10)).await;

        for value in data {
            scheduler_clone.exec(value).await.unwrap();
            println!("Sent!");
            time::sleep(Duration::from_secs(3)).await;
        }
    });

    // tokio::spawn(async move {
    //     let data = vec![
    //         metric::FeedItem {
    //             source: "/src/lua/ML0001.lua".to_string(),
    //             timestamp: Utc::now().timestamp(),
    //             value: 0.5,
    //         },
    //         metric::FeedItem {
    //             source: "/src/lua/ML0001.lua".to_string(),
    //             timestamp: Utc::now().timestamp() + 10,
    //             value: 0.6,
    //         },
    //         metric::FeedItem {
    //             source: "/src/lua/ML0001.lua".to_string(),
    //             timestamp: Utc::now().timestamp() + 20,
    //             value: 0.4,
    //         },
    //     ];
    //     time::sleep(Duration::from_secs(5)).await;

    //     for value in data {
    //         scheduler_clone.exec(value).await.unwrap();
    //         println!("Sent!");
    //         time::sleep(Duration::from_secs(3)).await;
    //     }

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
