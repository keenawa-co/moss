use chrono::Utc;
use common::id::NanoId;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, QueryOrder, QuerySelect, Set};
use std::sync::Arc;

use crate::domain::model::session::SessionEntity;
use crate::domain::port;
use crate::domain::{
    self, model::project::ProjectMeta, model::result::Result, model::session::SessionInfoEntity,
};

//
// Entity model definition for `session` table (root database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_meta_id: String,
    pub created_at: i64,
}

impl From<(Model, Option<super::project_meta_repo_impl::Model>)> for SessionEntity {
    fn from(data: (Model, Option<super::project_meta_repo_impl::Model>)) -> Self {
        SessionEntity {
            id: data.0.id.into(),
            project_meta: match data.1 {
                Some(project_meta) => Some(ProjectMeta {
                    id: project_meta.id.into(),
                    source: project_meta.source,
                    created_at: project_meta.created_at,
                }),
                None => None,
            },
            created_at: data.0.created_at,
        }
    }
}

impl From<Model> for SessionInfoEntity {
    fn from(value: Model) -> Self {
        SessionInfoEntity {
            id: value.id.into(),
            project_meta_id: value.project_meta_id.into(),
            created_at: value.created_at,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project_meta_repo_impl::Entity",
        from = "Column::ProjectMetaId",
        to = "super::project_meta_repo_impl::Column::Id"
    )]
    ProjectMeta,
}

impl Related<super::project_meta_repo_impl::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectMeta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//
// Repository implementation for project-related operations
//

#[derive(Debug)]
pub(crate) struct SessionRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl SessionRepositoryImpl {
    pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl port::rootdb::SessionRepository for SessionRepositoryImpl {
    async fn create(&self, project_id: &NanoId) -> Result<SessionInfoEntity> {
        let model = (ActiveModel {
            id: Set(NanoId::new().to_string()),
            project_meta_id: Set(project_id.to_string()),
            created_at: Set(Utc::now().timestamp()),
        })
        .insert(self.conn.as_ref())
        .await?;

        Ok(model.into())
    }

    async fn fetch_list_by_start_time(
        &self,
        start_time: i64,
        limit: u64,
    ) -> Result<Vec<SessionEntity>> {
        let result = Entity::find()
            .filter(Column::CreatedAt.gte(start_time))
            .order_by_desc(Column::CreatedAt)
            .limit(limit)
            .find_also_related(super::project_meta_repo_impl::Entity)
            .all(self.conn.as_ref())
            .await?;

        Ok(result
            .into_iter()
            .map(|(session, project_meta)| (session, project_meta).into())
            .collect())
    }

    async fn get_by_id(&self, session_id: &NanoId) -> Result<Option<SessionEntity>> {
        let result = Entity::find_by_id(session_id.clone())
            .find_also_related(super::project_meta_repo_impl::Entity)
            .one(self.conn.as_ref())
            .await?;

        match result {
            Some((session, project_meta)) => Ok(Some(SessionEntity::from((session, project_meta)))),
            None => Ok(None),
        }
    }
}
