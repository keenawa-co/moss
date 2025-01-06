use std::sync::atomic::{AtomicU8, Ordering};

use crate::app::{lifecycle::LifecyclePhase, service::Service};

pub struct LifecycleService {
    phase: AtomicU8,
}

impl LifecycleService {
    pub fn new() -> Self {
        Self {
            phase: AtomicU8::new(LifecyclePhase::Starting.as_u8()),
        }
    }

    pub fn get_current_phase(&self) -> LifecyclePhase {
        self.phase.load(Ordering::SeqCst).into()
    }

    pub fn set_current_phase(&self, value: LifecyclePhase) {
        self.phase.store(value.as_u8(), Ordering::SeqCst);
    }
}

impl Service for LifecycleService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
