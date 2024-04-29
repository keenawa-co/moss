use chrono::Utc;
use common::id::MNID;
use common::thing::Thing;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, QueryOrder, QuerySelect, Set};
use std::sync::Arc;

use crate::domain::{
    self,
    model::project::{CreateProjectInput, Project, RecentProject},
};

//
// Entity model definition for `project` table (root database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub source: String,
    pub last_used_at: i64,
    pub created_at: i64,
}

impl Into<domain::model::project::RecentProject> for Model {
    fn into(self) -> domain::model::project::RecentProject {
        domain::model::project::RecentProject {
            id: self.id.into(),
            source: self.source,
            last_used_at: self.last_used_at,
        }
    }
}

impl From<Model> for Project {
    fn from(value: Model) -> Self {
        Project {
            id: value.id.into(),
            source: value.source,
            last_used_at: value.last_used_at,
            created_at: value.created_at,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::session_repo_impl::Entity")]
    Session,
}

impl ActiveModelBehavior for ActiveModel {}

//
// Repository implementation for project-related operations
//

#[derive(Debug)]
pub(crate) struct ProjectRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl ProjectRepositoryImpl {
    pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl domain::port::ProjectRepository for ProjectRepositoryImpl {
    async fn create_project(&self, input: &CreateProjectInput) -> domain::Result<Project> {
        let current_timestamp = Utc::now().timestamp();
        let model = (ActiveModel {
            id: Set(MNID::new().to_string()),
            source: Set(input.path.to_string()),
            last_used_at: Set(current_timestamp),
            created_at: Set(current_timestamp),
        })
        .insert(self.conn.as_ref())
        .await?;

        Ok(model.into())
    }

    async fn get_project_by_id(&self, id: String) -> domain::Result<Option<Project>> {
        let model_option = Entity::find_by_id(id).one(self.conn.as_ref()).await?;

        Ok(model_option.map(Project::from))
    }

    async fn delete_by_id(&self, id: String) -> domain::Result<Thing> {
        let result = Entity::delete_by_id(&id).exec(self.conn.as_ref()).await?;

        match result.rows_affected {
            0 => Err(sea_orm::DbErr::RecordNotFound(id).into()),
            _ => Ok(Thing::from(id)),
        }
    }

    async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u64,
    ) -> domain::Result<Vec<RecentProject>> {
        let result = Entity::find()
            .filter(Column::LastUsedAt.gte(start_time))
            .order_by_desc(Column::LastUsedAt)
            .limit(limit)
            .all(self.conn.as_ref())
            .await?;

        Ok(result.into_iter().map(|item| item.into()).collect())
    }
}
