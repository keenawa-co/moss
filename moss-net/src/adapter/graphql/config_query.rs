use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::domain::service::ConfigService;

#[derive(Default)]
pub struct ConfigQuery;

#[Object]
impl ConfigQuery {
    async fn get_preference(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Arc<moss_core::config::Preference>> {
        let config_service = ctx.data::<Arc<ConfigService>>()?;
        let result = config_service.preferences.clone(); // FIXME

        Ok(result)
    }
}
