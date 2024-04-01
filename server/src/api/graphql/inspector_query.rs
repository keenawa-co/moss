use async_graphql::{Context, Object, Result, SimpleObject};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(SimpleObject)]
pub struct FileInfo {
    name: String,
    size: u64,
    mod_time: i64,
    is_dir: bool,
}

impl FileInfo {
    pub fn new(name: String, size: u64, mod_time: SystemTime, is_dir: bool) -> Self {
        let mod_time = mod_time
            .duration_since(UNIX_EPOCH)
            .expect("File modification time is before UNIX EPOCH")
            .as_secs() as i64;

        FileInfo {
            name,
            size,
            mod_time,
            is_dir,
        }
    }
}

#[derive(Default)]
pub struct InspectorQuery;

#[Object]
impl InspectorQuery {
    async fn read_dir(&self, ctx: &Context<'_>, path: String) -> Result<Vec<FileInfo>> {
        let mut entries = tokio::fs::read_dir(path).await?;
        let mut files_info = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let size = metadata.len();
            let mod_time = metadata.modified()?;
            let is_dir = metadata.is_dir();

            files_info.push(FileInfo::new(name, size, mod_time, is_dir));
        }

        Ok(files_info)
    }

    async fn read_file(&self, ctx: &Context<'_>, path: String) -> Result<String> {
        let content = tokio::fs::read_to_string(path).await?;
        Ok(content)
    }
}
