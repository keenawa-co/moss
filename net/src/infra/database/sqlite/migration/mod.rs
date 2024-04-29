use sea_orm_migration::prelude::*;

mod m20220101_000001_initial_schema_up;

#[derive(Debug, Clone)]
pub struct ProjectMigrator;

#[async_trait]
impl MigratorTrait for ProjectMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_initial_schema_up::Migration)]
    }
}
