use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::{self, model::portal::RecentProject};

#[derive(Debug)]
pub struct PortalRepositoryImpl {
    client: Arc<Surreal<Client>>,
    table_name: String,
}

impl PortalRepositoryImpl {
    pub fn new(client: Arc<Surreal<Client>>, recent_table: &str) -> Self {
        Self {
            client,
            table_name: recent_table.into(),
        }
    }
}

#[async_trait]
impl domain::port::PortalRepository for PortalRepositoryImpl {
    async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u8,
    ) -> Result<Vec<RecentProject>, domain::Error> {
        let result = self
            .client
            .query(
                "
                SELECT * FROM type::table($table)
                WHERE updated > type::int($start_time)
                ORDER BY updated DESC
                LIMIT type::int($limit)
                ",
            )
            .bind(("table", &self.table_name))
            .bind(("start_time", start_time))
            .bind(("limit", limit))
            .await?
            .take(0)?;

        // let selected: Vec<RecentProject> = self.client.select(&self.table_name).await?;

        Ok(result)
    }

    // async fn create_resent(&self, item: RecentItemInput) -> Result<Vec<RecentItem>, domain::Error> {
    //     let created: Vec<RecentItem> = self
    //         .client
    //         .create(&self.recent_table)
    //         .content(RecentItem {
    //             id: None,
    //             path: item.path,
    //             timestamp: Utc::now().timestamp(),
    //         })
    //         .await?;

    //     Ok(created)
    // }

    // async fn delete_recent_by_id(&self, id: String) -> Result<Option<RecentItem>, domain::Error> {
    //     let deleted: Option<RecentItem> = self.client.delete((&self.recent_table, id)).await?;

    //     Ok(deleted)
    // }
}
