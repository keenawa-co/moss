use sea_orm_migration::prelude::*;

mod m20220101_000001_initial_schema_up;

#[derive(Debug, Clone)]
pub(crate) struct CacheMigrator;

#[async_trait]
impl MigratorTrait for CacheMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_initial_schema_up::Migration)]
    }
}
