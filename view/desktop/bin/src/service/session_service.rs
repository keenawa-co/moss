use anyhow::{anyhow, Result};
use specta::Type;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::project_service::{ProjectDTO, ProjectEntity};

pub struct SessionService {
    conn: Arc<Surreal<Client>>,
}

#[derive(Debug, Deserialize)]
pub struct SessionEntity {
    pub id: Thing,
}

#[derive(Debug, Type, Serialize)]
pub struct SessionDTO {
    pub id: String,
}

impl From<SessionEntity> for SessionDTO {
    fn from(value: SessionEntity) -> Self {
        Self {
            id: value.id.id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SessionInfoEntity {
    pub created_at: i64,
    pub project: ProjectEntity,
    pub session: SessionEntity,
}

#[derive(Debug, Type, Serialize)]
pub struct SessionInfoDTO {
    #[specta(type = String)]
    pub created_at: i64,
    pub project: ProjectDTO,
    pub session: SessionDTO,
}

impl From<SessionInfoEntity> for SessionInfoDTO {
    fn from(value: SessionInfoEntity) -> Self {
        Self {
            created_at: value.created_at,
            project: value.project.into(),
            session: value.session.into(),
        }
    }
}

impl SessionService {
    pub fn new(conn: Arc<Surreal<Client>>) -> Self {
        Self { conn }
    }

    pub async fn restore_session(
        &self,
        mut project_source: Option<String>,
    ) -> Result<Option<SessionInfoEntity>> {
        if project_source.is_none() {
            if let Some(recently_viewed) = self.fetch_recently_viewed().await? {
                project_source = Some(recently_viewed.project.source);
            } else {
                return Err(anyhow!("unable to find a session to restore"));
            }
        }

        dbg!(&project_source);

        let surql = "
        BEGIN TRANSACTION;

        LET $project_record = (
            SELECT * FROM ONLY type::table('project') 
            WHERE source = type::string($project_source) 
            LIMIT 1
        );

        IF $project_record IS NOT NONE {
        let $has_session_record = (
                SELECT * FROM ONLY has_session 
                    WHERE in = $project_record.id
                    ORDER BY created_at DESC 
                    LIMIT 1
            );

            IF $has_session_record IS NONE {
                THROW \"session hasn't been defined\";
            };

            UPDATE ONLY has_session 
                SET created_at = time::unix() 
                WHERE id = $has_session_record.id;

            RETURN {
                created_at: $has_session_record.created_at,
                project: $project_record,
                session: (
                    SELECT * FROM ONLY session 
                    WHERE id = $has_session_record.out 
                    LIMIT 1
                )
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
            .bind(("project_source", project_source.unwrap()))
            .await
            .unwrap();

        let r = result.take(1).unwrap();

        Ok(r)
    }

    pub async fn create_session(&self, project_id: String) -> Result<Option<SessionInfoEntity>> {
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

    pub async fn fetch_recently_viewed(&self) -> Result<Option<SessionInfoEntity>> {
        let surql = "
        SELECT created_at, 
            (SELECT * FROM ONLY <-project LIMIT 1) as project, 
            (SELECT * FROM ONLY ->session LIMIT 1) as session
        FROM ONLY has_session
        ORDER BY created_at DESC
        LIMIT 1
        PARALLEL;
        ";

        let mut result = self.conn.query(surql).await?;

        Ok(result.take(0)?)
    }

    pub async fn fetch_recently_viewed_list(
        &self,
        start_time: i64,
        limit: u8,
    ) -> Result<Vec<SessionInfoEntity>> {
        let surql = "
        SELECT created_at, 
            (SELECT * FROM ONLY <-project LIMIT 1) as project, 
            (SELECT * FROM ONLY ->session LIMIT 1) as session
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
