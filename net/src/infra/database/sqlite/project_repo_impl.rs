use chrono::Utc;
use common::id::MNID;
use common::thing::Thing;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use std::sync::Arc;

use crate::domain::{
    self,
    model::project::{CreateProjectInput, Project},
};

//
// Entity model definition for `project` table (root database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub source: String,
    pub created_at: i64,
}

impl From<Model> for Project {
    fn from(value: Model) -> Self {
        Project {
            id: value.id.into(),
            source: value.source,
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
    async fn create(&self, input: &CreateProjectInput) -> domain::Result<Project> {
        let current_timestamp = Utc::now().timestamp();
        let model = (ActiveModel {
            id: Set(MNID::new().to_string()),
            source: Set(input.path.to_string()),
            created_at: Set(current_timestamp),
        })
        .insert(self.conn.as_ref())
        .await?;

        Ok(model.into())
    }

    async fn get_by_id(&self, id: MNID) -> domain::Result<Option<Project>> {
        let model_option = Entity::find_by_id(id).one(self.conn.as_ref()).await?;

        Ok(model_option.map(Project::from))
    }

    async fn delete_by_id(&self, id: MNID) -> domain::Result<Thing> {
        let result = Entity::delete_by_id(id.to_string())
            .exec(self.conn.as_ref())
            .await?;

        match result.rows_affected {
            0 => Err(sea_orm::DbErr::RecordNotFound(id.to_string()).into()),
            _ => Ok(Thing::from(id)),
        }
    }

    async fn get_list_by_ids(&self, ids: &Vec<MNID>) -> domain::Result<Vec<Project>> {
        let result_list = Entity::find()
            .filter(Column::Id.is_in(ids.clone()))
            .all(self.conn.as_ref())
            .await?;

        Ok(result_list.into_iter().map(|item| item.into()).collect())
    }
}
