use types::{id::NanoId, thing::Thing};

use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use std::path::PathBuf;
use std::sync::Arc;

use crate::interface::cache::IgnoredListRepository;
use crate::model::ignored::IgnoredSource;

//
// Entity model definition for `ignore_list` table
//

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ignore_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub source: String,
}

impl From<Model> for IgnoredSource {
    fn from(value: Model) -> Self {
        IgnoredSource {
            id: value.id.into(),
            source: value.source,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//
// Repository implementation for project-related operations
//

#[derive(Debug)]
pub(super) struct IgnoreListRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl IgnoreListRepositoryImpl {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl IgnoredListRepository for IgnoreListRepositoryImpl {
    async fn create_from_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> anyhow::Result<Vec<IgnoredSource>> {
        let result =
            seaorm_utl::transaction::weak_transaction(self.conn.as_ref(), |tx| async move {
                let mut result = Vec::new();
                let models: Vec<ActiveModel> = input_list
                    .iter()
                    .map(|item| {
                        let item_id = NanoId::new();
                        let item_source = item.to_string_lossy().to_string();

                        result.push(IgnoredSource {
                            id: item_id.clone(),
                            source: item_source.clone(),
                        });

                        ActiveModel {
                            id: Set(item_id.to_string()),
                            source: Set(item_source),
                        }
                    })
                    .collect();

                Entity::insert_many(models)
                    .exec_without_returning(&*tx)
                    .await?;

                Ok(result)
            })
            .await?;

        Ok(result)
    }

    async fn fetch_list(&self) -> anyhow::Result<Vec<IgnoredSource>> {
        let result = Entity::find().all(self.conn.as_ref()).await?;

        Ok(result.into_iter().map(IgnoredSource::from).collect())
    }

    async fn delete_by_id(&self, id: &NanoId) -> anyhow::Result<Option<Thing<NanoId>>> {
        let result = Entity::delete_by_id(id.clone())
            .exec(self.conn.as_ref())
            .await?;

        Ok(if result.rows_affected > 0 {
            // Some(Thing::from(id.clone()))
            None
        } else {
            None
        })
    }
}
