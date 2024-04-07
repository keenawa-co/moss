pub mod config;
mod domain;
mod infra;

pub use config::{Config, CONF};

use axum::Extension;
use std::sync::Arc;
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
    domain::service::{ConfigService, PortalService, ProjectService, ServiceLocator},
    infra::surrealdb::disk::SurrealOnDisk,
};

pub async fn bind() -> Result<(), domain::Error> {
    let conf = match CONF.get() {
        Some(conf) => conf,
        None => return Err(domain::Error::Configuration),
    };

    let surreal_disk = SurrealOnDisk::new(conf.surrealdb_client.clone(), &conf.surrealdb_tables);
    let service_locator = ServiceLocator {
        portal_service: Arc::new(PortalService::new(surreal_disk.portal_repo())),
        config_service: Arc::new(ConfigService::new(conf.preference.clone())),
        project_service: Arc::new(ProjectService::new(surreal_disk.project_repo())),
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

    let router = infra::web::router(service);

    println!("Listening on {}", conf.bind);

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    Ok(())
}
