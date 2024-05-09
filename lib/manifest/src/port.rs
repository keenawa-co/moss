use std::fmt::Debug;

#[async_trait]
pub(crate) trait RootManifest: Debug + Send + Sync {}
