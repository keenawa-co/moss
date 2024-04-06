use std::{
    net::SocketAddr,
    sync::{Arc, OnceLock},
};

pub static CONF: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: SocketAddr,
    pub preference: Arc<moss_core::config::Preference>,
}
