use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use moss_core::config;

use crate::domain::service::ConfigService;

#[derive(Default)]
pub struct ConfigQuery;

#[Object]
impl ConfigQuery {
    async fn get_preference(&self, ctx: &Context<'_>) -> Result<config::Preference> {
        let config_service = ctx.data::<Arc<ConfigService>>()?;
        let result: config::Preference = (*config_service.preferences).clone().into();

        Ok(result)
    }
}
