use std::sync::Arc;

use crate::domain::port::WatchListRepository;

pub struct SessionProjectService {
    watch_list_repo: Arc<dyn WatchListRepository>,
}

impl SessionProjectService {
    pub fn new(watch_list_repo: Arc<dyn WatchListRepository>) -> Self {
        Self { watch_list_repo }
    }
}
