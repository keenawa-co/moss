use fnv::FnvHashMap;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};
use strum::{EnumCount, EnumIter};
use tauri::AppHandle;

use super::{
    lifecycle::{LifecycleManager, LifecyclePhase},
    subscription::Subscription,
};

pub trait AnyService: Any + Send + Sync {
    fn start(&self, app_handle: &AppHandle);
    fn stop(&self, app_handle: &AppHandle);
    fn as_any(&self) -> &dyn Any;
}

impl dyn AnyService {
    pub fn downcast_arc<T: AnyService>(self: Arc<Self>) -> Result<Arc<T>, Arc<Self>> {
        if self.as_any().is::<T>() {
            let raw = Arc::into_raw(self) as *const T;
            Ok(unsafe { Arc::from_raw(raw) })
        } else {
            Err(self)
        }
    }
}

pub const MAX_ACTIVATION_POINTS: usize = ActivationPoint::COUNT;

#[derive(Debug, Eq, Hash, PartialEq, EnumIter, EnumCount)]
pub enum ActivationPoint {
    OnBootstrapping,
}

impl Into<LifecyclePhase> for ActivationPoint {
    fn into(self) -> LifecyclePhase {
        match self {
            ActivationPoint::OnBootstrapping => LifecyclePhase::Bootstrapping,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServiceState {
    Inactive = 0,
    Activating = 1,
    Active = 2,
    Deactivating = 3,
    Failed = 4,
}

// impl ServiceState {
//     fn as_u8(self) {

//     }
// }

#[derive(Clone)]
pub struct ServiceHandle {
    service: Arc<dyn AnyService>,
    lifecycle_sub: Arc<SmallVec<[Subscription; MAX_ACTIVATION_POINTS]>>,
    state: Arc<AtomicU8>,
}

pub struct ServiceManager2 {
    lifecycle_manager: Arc<LifecycleManager>,
    services: Arc<RwLock<FnvHashMap<TypeId, ServiceHandle>>>,
}

impl ServiceManager2 {
    pub fn new(lifecycle_manager: Arc<LifecycleManager>) -> Self {
        Self {
            lifecycle_manager,
            services: Arc::new(RwLock::new(FnvHashMap::default())),
        }
    }

    pub fn register<T: AnyService>(
        &self,
        service: T,
        activation_points: SmallVec<[ActivationPoint; MAX_ACTIVATION_POINTS]>,
    ) {
        let mut services_lock = self.services.write();
        let mut subscriptions = SmallVec::new();
        let service_arc = Arc::new(service);
        let service_state = Arc::new(AtomicU8::new(ServiceState::Inactive as u8));

        for (index, point) in activation_points.into_iter().enumerate() {
            let service_arc_clone = Arc::clone(&service_arc);
            let service_state_clone = Arc::clone(&service_state);

            subscriptions.insert(
                index,
                self.lifecycle_manager
                    .observe(point.into(), move |app_handle| {
                        let service_state_value = service_state_clone.load(Ordering::SeqCst);
                        if service_state_value == ServiceState::Active as u8
                            || service_state_value == ServiceState::Activating as u8
                        {
                            return;
                        }

                        service_state_clone.store(ServiceState::Activating as u8, Ordering::SeqCst);
                        trace!("Activating service");
                        service_arc_clone.start(app_handle);
                        service_state_clone.store(ServiceState::Active as u8, Ordering::SeqCst);
                    }),
            );
        }

        services_lock.insert(
            TypeId::of::<T>(),
            ServiceHandle {
                service: service_arc,
                lifecycle_sub: Arc::new(subscriptions),
                state: service_state,
            },
        );
    }

    pub fn get<T: AnyService>(&self) -> Option<Arc<T>> {
        if let Some(service_handle) = self.services.read().get(&TypeId::of::<T>()).cloned() {
            match service_handle.service.downcast_arc::<T>() {
                Ok(arc_t) => Some(arc_t),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn get_unchecked<T: AnyService>(&self) -> Arc<T> {
        let service_handle = self
            .services
            .read()
            .get(&TypeId::of::<T>())
            .cloned()
            .unwrap_or_else(|| panic!("Service not found"));

        service_handle
            .service
            .downcast_arc::<T>()
            .unwrap_or_else(|_| panic!("Service type mismatch"))
    }
}
