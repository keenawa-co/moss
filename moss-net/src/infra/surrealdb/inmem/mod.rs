use surrealdb::{engine::local::Db, Surreal};

pub struct SurrealInMem {}

impl SurrealInMem {
    pub fn new(_: Surreal<Db>) -> Self {
        Self {}
    }
}
