use anyhow::Result;
use chrono::Utc;
use specta::Type;
use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};
use types::id::NanoId;

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct ProjectMeta {
    pub id: NanoId,
    pub source: String,
    pub repository: Option<String>,
    pub opened_at: usize,
    pub created_at: usize,
}

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct CreateProjectMetaInput {
    pub source: String,
}

pub struct ProjectService {
    conn: Arc<Surreal<Db>>,
}

impl ProjectService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }

    pub async fn create_project(&self, input: &CreateProjectMetaInput) -> Result<Vec<ProjectMeta>> {
        let time_now = Utc::now().timestamp() as usize;

        Ok(self
            .conn
            .create("project_meta")
            .content(ProjectMeta {
                id: NanoId::new(),
                source: input.source.clone(),
                repository: None,
                opened_at: time_now,
                created_at: time_now,
            })
            .await?)
    }

    pub async fn get_project_by_id(&self, id: NanoId) -> Result<Option<ProjectMeta>> {
        Ok(self.conn.select(("project_meta", id.to_string())).await?)
    }

    pub async fn delete_project_by_id(&self, id: NanoId) -> Result<Option<ProjectMeta>> {
        Ok(self.conn.delete(("project_meta", id.to_string())).await?)
    }
}
