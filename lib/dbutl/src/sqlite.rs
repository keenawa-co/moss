use std::path::PathBuf;

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub async fn conn<T: MigratorTrait>(path: &PathBuf) -> anyhow::Result<DatabaseConnection> {
    let is_new_database = !path.exists();
    let database_url = format!("sqlite://{}?mode=rwc", path.to_string_lossy());
    let conn = Database::connect(&database_url).await?;
    if is_new_database {
        info!("Running {} migrations", path.display());
        T::up(&conn, None).await?;
    }

    Ok(conn)
}
