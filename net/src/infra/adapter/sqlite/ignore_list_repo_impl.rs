// use common::id::NanoId;
// use common::thing::Thing;
// use project::cache::IgnoreListRepository;
// use project::ignored::IgnoredSource;
// use sea_orm::entity::prelude::*;
// use sea_orm::{DatabaseConnection, Set};
// use std::path::PathBuf;
// use std::sync::Arc;

// // use crate::domain::model::project::IgnoredSource;
// use crate::domain::model::result::Result;

// //
// // Entity model definition for `ignore_list` table (project database)
// //

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
// #[sea_orm(table_name = "ignore_list")]
// pub struct Model {
//     #[sea_orm(primary_key, auto_increment = false)]
//     pub id: String,
//     pub source: String,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl ActiveModelBehavior for ActiveModel {}

// //
// // Repository implementation for project-related operations
// //

// #[derive(Debug)]
// pub(crate) struct IgnoreListRepositoryImpl {
//     conn: Arc<DatabaseConnection>,
// }

// impl IgnoreListRepositoryImpl {
//     pub(super) fn new(conn: Arc<DatabaseConnection>) -> Self {
//         Self { conn }
//     }
// }

// #[async_trait]
// impl IgnoreListRepository for IgnoreListRepositoryImpl {
//     async fn create_from_list(
//         &self,
//         input_list: &Vec<PathBuf>,
//     ) -> anyhow::Result<Vec<IgnoredSource>> {
//         let result = dbutl::transaction::weak_transaction(self.conn.as_ref(), |tx| async move {
//             let mut result = Vec::new();
//             let models: Vec<ActiveModel> = input_list
//                 .iter()
//                 .map(|item| {
//                     let item_id = NanoId::new();
//                     let item_source = item.to_string_lossy().to_string();

//                     result.push(IgnoredSource {
//                         id: item_id.clone(),
//                         source: item_source.clone(),
//                     });

//                     ActiveModel {
//                         id: Set(item_id.to_string()),
//                         source: Set(item_source),
//                     }
//                 })
//                 .collect();

//             Entity::insert_many(models)
//                 .exec_without_returning(&*tx)
//                 .await?;

//             Ok(result)
//         })
//         .await?;

//         Ok(result)
//     }

//     async fn delete_by_id(&self, id: &NanoId) -> anyhow::Result<Option<Thing>> {
//         let result = Entity::delete_by_id(id.clone())
//             .exec(self.conn.as_ref())
//             .await?;

//         Ok(if result.rows_affected > 0 {
//             Some(Thing::from(id.to_string()))
//         } else {
//             None
//         })
//     }
// }
