use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use serde::de::DeserializeOwned;
use sqlx::migrate::MigrateDatabase;
use std::{fs, path::PathBuf};

pub(crate) fn load_toml_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

pub(crate) async fn db_connection(path: &PathBuf) -> anyhow::Result<DatabaseConnection> {
    let database_filename = format!("{}.db", moss_core::constant::APP_NAME);
    let database_path = path.join(&database_filename);
    let database_url = format!("sqlite://{}", database_path.to_str().unwrap());

    let is_new = !database_path.exists();
    if is_new {
        info!("Creating a new database at '{}'", database_path.display());
        sqlx::Sqlite::create_database(database_path.to_str().unwrap()).await?;
    }

    let conn = Database::connect(&database_url).await?;

    if is_new {
        info!(
            "Running migrations for the new database at '{}'",
            database_path.display()
        );
        crate::migration::Migrator::up(&conn, None).await?;
    }

    Ok(conn)
}
