use std::{
    env,
    path::{Path, PathBuf},
};

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let file_path = {
            let migration_file_path = Path::new("m20220101_000001_initial_project_schema.up.sql");
            let migration_dir = Path::new("migration/app");
            let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

            root_dir.join(migration_dir).join(migration_file_path)
        };

        let migration_content = tokio::fs::read_to_string(file_path)
            .await
            .expect("Failed to read migration file");

        manager
            .get_connection()
            .execute_unprepared(&migration_content)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let file_path = {
            let migration_file_path = Path::new("m20220101_000001_initial_project_schema.down.sql");
            let migration_dir = Path::new("migration/app");
            let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

            root_dir.join(migration_dir).join(migration_file_path)
        };

        let migration_content = tokio::fs::read_to_string(file_path)
            .await
            .expect("Failed to read migration file");

        manager
            .get_connection()
            .execute_unprepared(&migration_content)
            .await?;

        Ok(())
    }
}
