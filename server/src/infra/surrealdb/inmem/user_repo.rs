use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{engine::local::Db, Surreal};

use crate::err::Error;

// #[derive(Clone, Debug)]
// pub struct UserRepository {
//     client: Surreal<Db>,
// }

// #[derive(Debug, Serialize, Deserialize, SimpleObject)]
// pub struct PreferenceItem {
//     pub name: String,
//     pub value: serde_json::Value,
// }

// #[derive(Debug, Serialize, Deserialize, SimpleObject)]
// pub struct PreferenceCategory {
//     pub name: String,
//     pub content: Vec<PreferenceItem>,
// }

// #[derive(Debug, Deserialize)]
// pub struct Record {
//     #[allow(dead_code)]
//     id: Thing,
// }

// impl UserRepository {
//     pub async fn create_preference_category(
//         &self,
//         category: PreferenceCategory,
//     ) -> Result<Vec<Record>, Error> {
//         let created: Vec<Record> = self
//             .client
//             .create("preference_category")
//             .content(category)
//             .await?;

//         Ok(created)
//     }

//     pub async fn select_all_preference_category(&self) -> Result<Vec<PreferenceCategory>, Error> {
//         let selected = self.client.select("preference_category").await?;

//         Ok(selected)
//     }
// }

// impl UserRepository {
//     pub fn new(client: Surreal<Db>) -> Self {
//         Self { client }
//     }
// }
