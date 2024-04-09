use std::net::{IpAddr, SocketAddr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(super) surrealdb: Surrealdb,
    pub(super) net: Net,
}

#[derive(Deserialize, Debug)]
pub struct Net {
    pub(super) endpoint: NetEndpoint,
}

impl Net {
    pub fn endpoint_addr(&self) -> SocketAddr {
        SocketAddr::new(self.endpoint.host, self.endpoint.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct NetEndpoint {
    pub(super) host: IpAddr,
    pub(super) port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Surrealdb {
    pub(super) endpoint: SurrealdbEndpoint,
    #[allow(dead_code)]
    pub(super) config: Option<SurrealdbConfig>,
}

impl Surrealdb {
    pub fn endpoint_addr(&self) -> SocketAddr {
        SocketAddr::new(self.endpoint.host, self.endpoint.port)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SurrealdbConfig {
    #[serde(default)]
    pub(super) strict: bool,
    #[serde(default)]
    pub(super) notifications: bool,
}

#[derive(Deserialize, Debug)]
pub struct SurrealdbEndpoint {
    pub(super) host: IpAddr,
    pub(super) port: u16,
    pub(super) namespace: String,
    pub(super) database: String,
}
