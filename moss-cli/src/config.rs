use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub surrealdb: SurrealdbConfig,
}

#[derive(Deserialize, Debug)]
pub struct SurrealdbConfig {
    pub host: String,
    pub port: u16,
}

impl SurrealdbConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
