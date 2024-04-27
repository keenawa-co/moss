use async_graphql::Error as GraphqlError;
use async_graphql::{Context, FieldResult, Subscription};
use file::FileInfo;
use fs::{real::FileSystem, FS};
use futures::{Stream, StreamExt};
use std::path::PathBuf;

const READ_DIR_CHUNK_SIZE: usize = 10;

#[derive(Default)]
pub(super) struct ExplorerSubscription;

#[Subscription]
impl ExplorerSubscription {
    #[graphql(name = "explorerReadDir")]
    async fn read_dir(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<Vec<FileInfo>>>> {
        // FIXME: use service, not directly FS

        let path_buf = PathBuf::from(path);
        let stream = FileSystem::new()
            .read_dir(&path_buf)
            .await
            .map_err(|e| format!("Failed to read directory: {e}"))?;

        Ok(stream.chunks(READ_DIR_CHUNK_SIZE).then(|chunk| async move {
            let result: Result<Vec<_>, GraphqlError> = chunk
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .and_then(|paths| paths.into_iter().map(FileInfo::new).collect())
                .map_err(|e| GraphqlError::new(e.to_string()));

            result
        }))
    }
}
