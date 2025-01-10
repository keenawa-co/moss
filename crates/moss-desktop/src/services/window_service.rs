use std::sync::atomic::AtomicUsize;

use crate::app::service::Service;

const INITIAL_WINDOW_ID: usize = 0;
const WINDOW_ID_INCREMENT: usize = 1;

pub struct WindowService {
    next_window_id: AtomicUsize,
}

impl WindowService {
    pub fn new() -> Self {
        Self {
            next_window_id: AtomicUsize::new(INITIAL_WINDOW_ID),
        }
    }

    pub fn next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(WINDOW_ID_INCREMENT, std::sync::atomic::Ordering::SeqCst)
    }
}

impl Service for WindowService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
