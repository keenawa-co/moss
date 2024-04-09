use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub surrealdb: Surrealdb,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Surrealdb {
    pub endpoint: SurrealdbEndpoint,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SurrealdbConfig {
    #[serde(default)]
    pub strict: bool,
    #[serde(default)]
    pub notifications: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SurrealdbEndpoint {
    pub host: String,
    pub port: u16,
    pub namespace: String,
    pub database: String,
}

impl SurrealdbEndpoint {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
