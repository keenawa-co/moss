mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use schemars::JsonSchema;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use surrealdb::{
    method::Query,
    opt::{IntoEndpoint, IntoQuery},
    Connection, Surreal,
};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

pub struct SurrealClient<C: Connection>(Surreal<C>);

impl<C: Connection> SurrealClient<C> {
    pub async fn new<P>(
        address: impl IntoEndpoint<P, Client = C>,
        ns: impl Into<String>,
        db: impl Into<String>,
    ) -> Result<Self> {
        let client = Surreal::new::<P>(address).await?;
        client.use_ns(ns).use_db(db).await?;

        Ok(Self(client))
    }

    pub fn query(&self, query: impl IntoQuery) -> Query<C> {
        self.query(query)
    }
}

pub struct AppState {
    pub project_service: ProjectService,
    pub session_service: SessionService,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SettingsSchema {
    pub window: WindowSettingsSchema,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename = "Window")]
pub struct WindowSettingsSchema {
    #[serde(rename = "window.restoreTab")]
    #[serde(default = "WindowSettingsSchema::default_restore_tab")]
    pub restore_tab: bool,
    #[serde(rename = "window.restoreFullScreen")]
    #[serde(default = "WindowSettingsSchema::default_restore_full_screen")]
    /// Test123
    pub restore_full_screen: bool,
}

impl WindowSettingsSchema {
    fn default_restore_tab() -> bool {
        true
    }

    fn default_restore_full_screen() -> bool {
        true
    }
}
