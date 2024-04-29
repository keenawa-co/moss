use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::domain;

//
// Entity model definition for `watch_list` table (project database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "watch_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub source: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//
// Repository implementation for project-related operations
//

#[derive(Debug)]
pub(crate) struct WatchListRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl WatchListRepositoryImpl {
    pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

impl domain::port::IgnoreRepository for WatchListRepositoryImpl {}
