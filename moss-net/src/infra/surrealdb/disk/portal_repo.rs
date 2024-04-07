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
}

impl PortalRepositoryImpl {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl domain::port::PortalRepository for PortalRepositoryImpl {
    async fn select_resent_list(&self) -> Result<Vec<RecentItem>, domain::Error> {
        self.client.use_ns("cache").use_db("portal").await?;

        let selected: Vec<RecentItem> = self.client.select("recent").await?;
        Ok(selected)
    }

    async fn create_resent(&self, item: RecentItemInput) -> Result<Vec<RecentItem>, domain::Error> {
        self.client.use_ns("cache").use_db("portal").await?;

        let created: Vec<RecentItem> = self
            .client
            .create("recent")
            .content(RecentItem {
                id: None,
                path: item.path,
                timestamp: Utc::now().timestamp(),
            })
            .await?;

        Ok(created)
    }

    async fn delete_recent_by_id(&self, id: String) -> Result<Option<RecentItem>, domain::Error> {
        self.client.use_ns("cache").use_db("portal").await?;

        let deleted: Option<RecentItem> = self.client.delete(("recent", id)).await?;

        Ok(deleted)
    }
}
