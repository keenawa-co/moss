mod api;
mod config;

pub use config::{Config, CONF};

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let conf = CONF.get().unwrap();
    let router = api::router();
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    println!("Listening on {}", conf.bind);

    axum_server::bind(conf.bind)
        .serve(router.clone().into_make_service())
        .await?;

    return Ok(());
}
