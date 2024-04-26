use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};

#[derive(Deserialize, Debug)]
pub struct Config {
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
