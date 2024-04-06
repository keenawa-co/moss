use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::domain::{
    model::portal::{RecentItem, RecentItemInput},
    service::PortalService,
};

#[derive(Default)]
pub struct PortalQuery;

#[Object]
impl PortalQuery {
    async fn select_resent_list(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<RecentItem>> {
        let portal_service = ctx.data::<Arc<PortalService>>()?;
        let result = portal_service.select_resent_list().await?;

        Ok(result)
    }
}

#[derive(Default)]
pub struct PortalMutation;

#[Object]
impl PortalMutation {
    async fn create_resent(
        &self,
        ctx: &Context<'_>,
        item: RecentItemInput,
    ) -> async_graphql::Result<Vec<RecentItem>> {
        let portal_service = ctx.data::<Arc<PortalService>>()?;
        let result = portal_service.crate_recent(item).await?;

        Ok(result)
    }

    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<RecentItem> {
        let portal_service = ctx.data::<Arc<PortalService>>()?;

        Ok(portal_service.delete_by_id(id).await?)
    }
}
