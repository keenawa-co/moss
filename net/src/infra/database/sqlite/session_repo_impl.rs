use chrono::Utc;
use common::id::NanoId;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, QueryOrder, QuerySelect, Set};
use std::sync::Arc;

use crate::domain;
use crate::domain::model::project::ProjectMeta;
use crate::domain::model::session::{Session, SessionInfo};

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

impl From<(Model, super::project_meta_repo_impl::Model)> for Session {
    fn from(data: (Model, super::project_meta_repo_impl::Model)) -> Self {
        Session {
            id: data.0.id.into(),
            project_meta: ProjectMeta {
                id: data.1.id.into(),
                source: data.1.source,
                created_at: data.1.created_at,
            },
            created_at: data.0.created_at,
        }
    }
}

impl From<Model> for SessionInfo {
    fn from(value: Model) -> Self {
        SessionInfo {
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
impl domain::port::SessionRepository for SessionRepositoryImpl {
    async fn create(&self, project_id: NanoId) -> domain::Result<SessionInfo> {
        let model = (ActiveModel {
            id: Set(NanoId::new().to_string()),
            project_meta_id: Set(project_id.to_string()),
            created_at: Set(Utc::now().timestamp()),
        })
        .insert(self.conn.as_ref())
        .await?;

        Ok(model.into())
    }

    async fn get_recent_list(&self, start_time: i64, limit: u64) -> domain::Result<Vec<Session>> {
        let result = Entity::find()
            .filter(Column::CreatedAt.gte(start_time))
            .order_by_desc(Column::CreatedAt)
            .limit(limit)
            .find_also_related(super::project_meta_repo_impl::Entity)
            .all(self.conn.as_ref())
            .await?;

        Ok(result
            .into_iter()
            .map(|(session, project_meta)| (session, project_meta.unwrap()).into())
            .collect())
    }
}
