use sea_orm_migration::prelude::*;

mod m20220101_000001_initial_schema_up;

pub struct RootMigrator;

#[async_trait::async_trait]
impl MigratorTrait for RootMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_initial_schema_up::Migration)]
    }
}
