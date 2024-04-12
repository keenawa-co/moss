use sea_orm::Database;
use serde::de::DeserializeOwned;
use sqlx::migrate::MigrateDatabase;
use std::{fs, path::Path};

pub(crate) fn load_toml_file<T: DeserializeOwned>(path: Box<Path>) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

pub(crate) async fn db_connection(
    path: &std::path::Path,
) -> anyhow::Result<sea_orm::DatabaseConnection> {
    if !path.exists() {
        sqlx::Sqlite::create_database(&format!("{}.db", moss_core::constant::APP_NAME)).await?;
    }

    Ok(Database::connect(&format!("sqlite://{}.db", moss_core::constant::APP_NAME)).await?)
}
