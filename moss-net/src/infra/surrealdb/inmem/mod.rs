use surrealdb::{engine::local::Db, Surreal};

pub struct SurrealInMem {}

impl SurrealInMem {
    pub fn new(client: Surreal<Db>) -> Self {
        Self {}
    }
}
