use std::{
    net::SocketAddr,
    sync::{Arc, OnceLock},
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::infra;

pub use crate::infra::surrealdb::disk::TableSet as SurrealdbTableSet;

pub static CONF: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: SocketAddr,
    pub preference: Arc<moss_core::config::Preference>,
    pub surrealdb_client: Arc<Surreal<Client>>,
    pub surrealdb_tables: infra::surrealdb::disk::TableSet,
}
