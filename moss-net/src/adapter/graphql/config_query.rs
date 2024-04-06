use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use moss_core::config::behaver_preference::BehaverPreferenceConfig;

use crate::domain::service::config_service::ConfigService;

#[derive(Default)]
pub struct ConfigQuery;

#[Object]
impl ConfigQuery {
    async fn get_all_preference_category(
        &self,
        ctx: &Context<'_>,
    ) -> Result<BehaverPreferenceConfig> {
        let config_service = ctx.data::<Arc<ConfigService>>()?;
        let result: BehaverPreferenceConfig = (*config_service.preferences).clone().into();

        Ok(result)
    }
}
