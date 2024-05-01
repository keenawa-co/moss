use std::sync::Arc;

use crate::domain::port::IgnoreRepository;

pub struct ProjectService {
    ignore_repo: Arc<dyn IgnoreRepository>,
}

impl ProjectService {
    pub fn new(ignore_repo: Arc<dyn IgnoreRepository>) -> Self {
        Self {
            ignore_repo: ignore_repo,
        }
    }
}
