use chrono::Utc;
use common::id::NanoId;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, QueryOrder, QuerySelect, Set};
use std::sync::Arc;

use crate::domain;
use crate::domain::model::session::{CreateSessionInput, Session};

//
// Entity model definition for `session` table (root database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub created_at: i64,
}

impl From<Model> for Session {
    fn from(value: Model) -> Self {
        Session {
            id: value.id.into(),
            project_id: value.project_id.into(),
            created_at: value.created_at,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project_repo_impl::Entity",
        from = "Column::ProjectId",
        to = "super::project_repo_impl::Column::Id"
    )]
    Project,
}

impl Related<super::project_repo_impl::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
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
    async fn create(&self, input: &CreateSessionInput) -> domain::Result<Session> {
        let model = (ActiveModel {
            id: Set(NanoId::new().to_string()),
            project_id: Set(input.project_id.to_string()),
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
            .all(self.conn.as_ref())
            .await?;

        Ok(result.into_iter().map(|item| item.into()).collect())
    }
}
