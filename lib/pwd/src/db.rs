use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::Path;

pub(super) async fn open_project_conn<P: AsRef<Path>>(
    project_path: P,
) -> anyhow::Result<DatabaseConnection> {
    // TODO: check if db is already exists

    let database_path = project_path
        .as_ref()
        .join(format!(".{}", common::APP_NAME))
        .join("project.db");
    let is_new_db = !database_path.exists();
    let database_url = format!(
        "sqlite://{}?mode=rwc",
        database_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("database path contains invalid characters"))?
    );

    let conn = Database::connect(&database_url).await?;

    if is_new_db {
        info!(
            "Running migrations for the new project database at '{}'",
            database_path.display()
        );
        crate::migration::Migrator::up(&conn, None).await?;
    }

    Ok(conn)
}

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
