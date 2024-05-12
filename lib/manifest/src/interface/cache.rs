use std::{fmt::Debug, path::PathBuf, sync::Arc};
use types::{id::NanoId, thing::Thing};

use crate::model::ignored::IgnoredSource;

#[async_trait]
pub(crate) trait CacheAdapter: Debug + Send + Sync {
    fn ignored_list_repo(&self) -> Arc<dyn IgnoredListRepository>;
}

#[async_trait]
pub trait IgnoredListRepository: Debug + Send + Sync {
    async fn create_from_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> anyhow::Result<Vec<IgnoredSource>>;

    async fn delete_by_id(&self, id: &NanoId) -> anyhow::Result<Option<Thing>>;

    async fn fetch_list(&self) -> anyhow::Result<Vec<IgnoredSource>>;
}
