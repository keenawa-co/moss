use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use surrealdb::sql::Thing;
use surrealdb::{engine::local::Db, Surreal};

use crate::err::Error;

mod user_repo;

pub struct SurrealInMem {
    // pub user_repo: UserRepository,
}

impl SurrealInMem {
    pub fn new(client: Surreal<Db>) -> Self {
        Self {
            // user_repo: UserRepository::new(client),
        }
    }
}
