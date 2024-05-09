use chrono::Utc;
use common::id::NanoId;
use common::thing::Thing;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use std::path::PathBuf;
use std::sync::Arc;

use crate::domain::port;
use crate::domain::{
    model::project::{CreateProjectInput, ProjectMeta},
    model::result::Result,
};

//
// Entity model definition for `project_meta` table (root database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_meta")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub source: String,
    pub created_at: i64,
}

impl From<Model> for ProjectMeta {
    fn from(value: Model) -> Self {
        ProjectMeta {
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
pub(crate) struct ProjectMetaRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl ProjectMetaRepositoryImpl {
    pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl port::rootdb::ProjectMetaRepository for ProjectMetaRepositoryImpl {
    async fn create(&self, input: &CreateProjectInput) -> Result<ProjectMeta> {
        let current_timestamp = Utc::now().timestamp();
        let model = (ActiveModel {
            id: Set(NanoId::new().to_string()),
            source: Set(input.path.canonicalize()?.to_string_lossy().to_string()),
            created_at: Set(current_timestamp),
        })
        .insert(self.conn.as_ref())
        .await?;

        Ok(model.into())
    }

    async fn get_by_id(&self, id: &NanoId) -> Result<Option<ProjectMeta>> {
        let model_option = Entity::find_by_id(id.clone())
            .one(self.conn.as_ref())
            .await?;

        Ok(model_option.map(ProjectMeta::from))
    }

    async fn get_by_source(&self, source: &PathBuf) -> Result<Option<ProjectMeta>> {
        let model_option = Entity::find()
            .filter(Column::Source.eq(source.to_str()))
            .one(self.conn.as_ref())
            .await?;

        Ok(model_option.map(ProjectMeta::from))
    }

    async fn delete_by_id(&self, id: &NanoId) -> Result<Option<Thing>> {
        let rows_affected = Entity::delete_by_id(id.clone())
            .exec(self.conn.as_ref())
            .await?
            .rows_affected; // FIXME: remove this call

        Ok(if rows_affected > 0 {
            Some(Thing::from(id.to_string()))
        } else {
            None
        })
    }

    async fn get_list_by_ids(&self, ids: &Vec<NanoId>) -> Result<Vec<ProjectMeta>> {
        let result_list = Entity::find()
            .filter(Column::Id.is_in(ids.clone()))
            .all(self.conn.as_ref())
            .await?;

        Ok(result_list.into_iter().map(|item| item.into()).collect())
    }
}
