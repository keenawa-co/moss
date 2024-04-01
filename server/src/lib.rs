mod api;
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

pub struct Server {
    addr: SocketAddr,
    router: Router,
}

impl Server {
    pub fn new(host: Ipv4Addr, port: u16) -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(host), port),
            router: api::router(),
        }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create database connection
        let db = Surreal::new::<Mem>(()).await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        println!("Listening on {}", self.addr);

        axum_server::bind(self.addr)
            .serve(self.router.clone().into_make_service())
            .await?;

        Ok(())
    }
}
