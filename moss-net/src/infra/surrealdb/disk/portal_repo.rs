use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::{self, model::portal::RecentProject};

#[derive(Debug)]
pub(crate) struct PortalRepositoryImpl {
    client: Arc<Surreal<Client>>,
    table_name: String,
}

impl PortalRepositoryImpl {
    pub(super) fn new(client: Arc<Surreal<Client>>, recent_table: &str) -> Self {
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
    ) -> domain::Result<Vec<RecentProject>> {
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

        Ok(result)
    }
}
