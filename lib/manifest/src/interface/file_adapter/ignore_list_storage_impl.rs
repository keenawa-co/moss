use smol::{
    fs::File,
    io::{AsyncSeekExt, AsyncWriteExt},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use crate::{interface::file::IgnoredListStorage, model::file::RootFile};

#[derive(Debug)]
pub(super) struct IgnoredListStorageImpl {
    state: Arc<Mutex<RootFile>>,
    file: Mutex<File>,
}

impl IgnoredListStorageImpl {
    pub fn new(state: Arc<Mutex<RootFile>>, file: Mutex<File>) -> Self {
        Self { state, file }
    }
}

#[async_trait]
impl IgnoredListStorage for IgnoredListStorageImpl {
    async fn create_from_list(&self, input_list: &Vec<PathBuf>) -> anyhow::Result<()> {
        let mut state_lock = self.state.lock().await;
        for item in input_list {
            state_lock
                .ignored_list
                .push(item.to_string_lossy().to_string());
        }

        let content = serde_json::to_string_pretty(&*state_lock)?;
        drop(state_lock);

        dbg!(&content);

        let mut file_lock = self.file.lock().await;
        file_lock.seek(smol::io::SeekFrom::Start(0)).await?;
        file_lock.write_all(content.as_bytes()).await?;
        file_lock.flush().await?;

        file_lock.flush().await?;
        Ok(())
    }

    async fn delete(&self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
