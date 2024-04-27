use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!(concat!(
                "../../../../migration/project/m20220101_000001_initial_project_schema.up.sql"
            )))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!(concat!(
                "../../../../migration/project/m20220101_000001_initial_project_schema.down.sql"
            )))
            .await?;

        Ok(())
    }
}
