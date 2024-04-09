use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(super) surrealdb: Surrealdb,
}

#[derive(Deserialize, Debug)]
pub struct Surrealdb {
    pub(super) endpoint: SurrealdbEndpoint,
    #[allow(dead_code)]
    pub(super) config: Option<SurrealdbConfig>,
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
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) namespace: String,
    pub(super) database: String,
}

impl SurrealdbEndpoint {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
