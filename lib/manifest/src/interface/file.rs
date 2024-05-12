use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) trait FileAdapter: Debug + Send + Sync {
    fn ignored_list_storage(&self) -> Arc<dyn IgnoredListStorage>;
}

#[async_trait]
pub trait IgnoredListStorage: Debug + Send + Sync {
    async fn create_from_list(&self, input_list: &Vec<PathBuf>) -> anyhow::Result<()>;
    async fn delete(&self) -> anyhow::Result<()>;
}
