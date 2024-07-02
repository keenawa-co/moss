use anyhow::Result;
use specta::Type;
use std::sync::Arc;
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use super::project_service::Project;

pub struct SessionService {
    conn: Arc<Surreal<Db>>,
}

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct Session {
    #[specta(type = String)]
    pub id: Thing,
}

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct SessionInfo {
    #[specta(type = String)]
    pub created_at: i64,
    pub project: Project,
    pub session: Session,
}

impl SessionService {
    pub async fn create_session(&self, project_id: String) -> Result<Option<SessionInfo>> {
        let surql = "
        BEGIN TRANSACTION;

        LET $project_record = (SELECT * FROM ONLY type::thing('project', $project_id));

        IF $project_record IS NOT NONE {
            LET $session_record = (CREATE ONLY type::table($table));

            LET $relation = (
                RELATE ONLY (type::thing('project', $project_id)) -> has_session -> $session_record CONTENT {
                    created_at: time::unix()
                }
            );

            RETURN {
                created_at: $relation.created_at,
                project: $project_record,
                session: $session_record,
            };
        }
        ELSE {
            THROW \"project hasn't been defined\";
        };

        COMMIT TRANSACTION;
        ";

        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "session"))
            .bind(("project_id", project_id))
            .await?;

        Ok(result.take(1)?)
    }

    pub async fn fetch_recently_viewed(
        &self,
        start_time: i64,
        limit: u8,
    ) -> Result<Vec<SessionInfo>> {
        let surql = "
        SELECT created_at, 
            (array::first(SELECT * FROM <-project)) as project, 
            (array::first(SELECT * FROM ->session)) as session
        FROM has_session
        WHERE created_at >= $start_time
        ORDER BY created_at DESC
        LIMIT $limit
        PARALLEL;
        ";

        let mut result = self
            .conn
            .query(surql)
            .bind(("start_time", start_time))
            .bind(("limit", limit))
            .await?;

        Ok(result.take(0)?)
    }
}
