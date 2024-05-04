use common::id::NanoId;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DatabaseConnection, Set};
use std::path::PathBuf;
use std::sync::Arc;

use crate::domain;

//
// Entity model definition for `ignore_list` table (project database)
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ignore_list")]
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
pub(crate) struct IgnoreListRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl IgnoreListRepositoryImpl {
    pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl domain::port::IgnoreListRepository for IgnoreListRepositoryImpl {
    async fn create(&self, input_list: &Vec<PathBuf>) -> domain::Result<()> {
        dbutl::transaction::weak_transaction(self.conn.as_ref(), |tx| async move {
            let models: Vec<ActiveModel> = input_list
                .iter()
                .map(|item| ActiveModel {
                    id: Set(NanoId::new().to_string()),
                    source: Set(item.to_string_lossy().to_string()),
                })
                .collect();

            Entity::insert_many(models)
                .on_conflict(
                    OnConflict::columns([Column::Source])
                        .do_nothing()
                        .to_owned(),
                )
                .exec_without_returning(&*tx)
                .await?;

            Ok(())
        })
        .await?;

        Ok(())
    }
}
