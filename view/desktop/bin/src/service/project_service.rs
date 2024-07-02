use anyhow::Result;
use specta::Type;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct Project {
    #[specta(type = String)]
    pub id: Thing,
    pub source: String,
    pub repository: Option<String>,
    #[specta(type = String)]
    pub created_at: i64,
}

pub struct ProjectService {
    conn: Arc<Surreal<Client>>,
}

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct CreateProjectInput {
    pub source: String,
    pub repository: Option<String>,
}

impl ProjectService {
    pub fn new(conn: Arc<Surreal<Client>>) -> Self {
        Self { conn }
    }

    pub async fn create_project(&self, input: &CreateProjectInput) -> Result<Option<Project>> {
        let surql = "
            CREATE ONLY type::table($table) CONTENT {
                source: $input.source,
                repository: $input.repository,
                created_at: time::unix(),
            };
            ";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project"))
            .bind(("input", input))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn get_project_by_id(&self, id: String) -> Result<Option<Project>> {
        let surql = "SELECT * FROM ONLY type::thing($table, $id)";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project"))
            .bind(("id", id))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn delete_project_by_id(&self, id: String) -> Result<Option<Project>> {
        let surql = "DELETE ONLY type::thing($table, $id)";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project"))
            .bind(("id", id))
            .await?;

        Ok(result.take(0)?)
    }
}
