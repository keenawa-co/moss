use common::{id::NanoId, thing::Thing};
use std::{fmt::Debug, path::PathBuf, sync::Arc};

use crate::ignored::IgnoredSource;

#[async_trait]
pub trait CacheAdapter: Debug + Send + Sync {
    async fn ignore_list_repo(&self) -> Arc<dyn IgnoreListRepository>;
}

#[async_trait]
pub trait IgnoreListRepository: Debug + Send + Sync {
    async fn create_from_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> anyhow::Result<Vec<IgnoredSource>>;
    async fn delete_by_id(&self, id: &NanoId) -> anyhow::Result<Option<Thing>>;
}
