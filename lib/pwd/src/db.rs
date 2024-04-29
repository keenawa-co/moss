use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::Path;

pub(super) async fn open_cache_conn<P: AsRef<Path>>(
    project_path: P,
) -> anyhow::Result<DatabaseConnection> {
    // TODO: check if db is already exists

    let database_path = project_path
        .as_ref()
        .join(format!(".{}", common::APP_NAME))
        .join("cache")
        .join("cache.db");
    let database_url = format!(
        "sqlite://{}?mode=rwc",
        database_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("database path contains invalid characters"))?
    );

    Ok(Database::connect(&database_url).await?)
}
