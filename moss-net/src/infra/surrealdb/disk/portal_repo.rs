use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::{
    self,
    model::portal::{RecentItem, RecentItemInput},
};

#[derive(Debug)]
pub struct PortalRepositoryImpl {
    client: Arc<Surreal<Client>>,
    recent_table: String,
}

impl PortalRepositoryImpl {
    pub fn new(client: Arc<Surreal<Client>>, recent_table: &str) -> Self {
        Self {
            client,
            recent_table: recent_table.into(),
        }
    }
}

#[async_trait]
impl domain::port::PortalRepository for PortalRepositoryImpl {
    async fn select_resent_list(&self) -> Result<Vec<RecentItem>, domain::Error> {
        let selected: Vec<RecentItem> = self.client.select(&self.recent_table).await?;

        Ok(selected)
    }

    async fn create_resent(&self, item: RecentItemInput) -> Result<Vec<RecentItem>, domain::Error> {
        let created: Vec<RecentItem> = self
            .client
            .create(&self.recent_table)
            .content(RecentItem {
                id: None,
                path: item.path,
                timestamp: Utc::now().timestamp(),
            })
            .await?;

        Ok(created)
    }

    async fn delete_recent_by_id(&self, id: String) -> Result<Option<RecentItem>, domain::Error> {
        let deleted: Option<RecentItem> = self.client.delete((&self.recent_table, id)).await?;

        Ok(deleted)
    }
}
