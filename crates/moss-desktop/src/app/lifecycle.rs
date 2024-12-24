use parking_lot::Mutex;
use std::time::Duration;
use tauri::AppHandle;

use super::subscription::{SubscriberSet, Subscription};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecyclePhase {
    Starting,
    Bootstrapping,
    Running { uptime: Duration },
    Stopping,
}

type Listener = Box<dyn Fn(&AppHandle) + Send + 'static>;

pub struct LifecycleManager {
    phase: Mutex<LifecyclePhase>,
    listeners: SubscriberSet<LifecyclePhase, Listener>,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            phase: Mutex::new(LifecyclePhase::Starting),
            listeners: SubscriberSet::new(),
        }
    }

    pub fn observe(
        &self,
        phase: LifecyclePhase,
        callback: impl Fn(&AppHandle) + Send + 'static,
    ) -> Subscription {
        let (subscription, activate) = self.listeners.insert(phase, Box::new(callback));
        activate();

        subscription
    }

    pub fn set_phase(&self, app_handle: &AppHandle, next_phase: LifecyclePhase) {
        let mut phase_lock = self.phase.lock();
        *phase_lock = next_phase;

        self.listeners.retain(&next_phase, |subscriber_callback| {
            subscriber_callback(app_handle);

            true
        });
    }
}
