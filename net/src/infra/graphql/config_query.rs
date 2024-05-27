use async_graphql::{Context, Object};
use conf::pref::Preference;
use std::sync::Arc;

use crate::domain::service::config_service::ConfigService;

pub(super) struct ConfigQuery {
    pub config_service: Arc<ConfigService>,
}

#[Object]
impl ConfigQuery {
    async fn get_preference(&self, _ctx: &Context<'_>) -> async_graphql::Result<Arc<Preference>> {
        unimplemented!()
    }
}
