use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
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

pub trait ServiceMetadata {
    fn service_brand() -> &'static str {
        std::any::type_name::<Self>()
    }
}

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
enum ServiceStatus {
    Inactive = 0,
    Activating = 1,
    Active = 2,
    Deactivating = 3,
    Failed = 4,
}

impl ServiceStatus {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

struct ServiceHandleState {
    activation_subscriptions: Mutex<SmallVec<[Subscription; MAX_ACTIVATION_POINTS]>>,
    status: AtomicU8,
}

impl ServiceHandleState {
    fn is_activation_possible(&self) -> bool {
        self.status.load(Ordering::SeqCst) == ServiceStatus::Inactive.as_u8()
    }

    fn is_active(&self) -> bool {
        self.status.load(Ordering::SeqCst) == ServiceStatus::Active.as_u8()
    }

    fn set_status(&self, new_status: ServiceStatus) {
        self.status.store(new_status.as_u8(), Ordering::SeqCst);
    }
}

#[derive(Clone)]
pub struct ServiceHandle {
    service: Arc<dyn AnyService>,
    state: Arc<ServiceHandleState>,
}

pub struct ServiceManager {
    lifecycle_manager: Arc<LifecycleManager>,
    services: Arc<RwLock<FnvHashMap<TypeId, ServiceHandle>>>,
}

impl ServiceManager {
    pub fn new(lifecycle_manager: Arc<LifecycleManager>) -> Self {
        Self {
            lifecycle_manager,
            services: Arc::new(RwLock::new(FnvHashMap::default())),
        }
    }

    pub fn register<T: AnyService + ServiceMetadata>(
        &self,
        service: T,
        activation_points: SmallVec<[ActivationPoint; MAX_ACTIVATION_POINTS]>,
    ) {
        let service_arc = Arc::new(service);
        let service_state = Arc::new(ServiceHandleState {
            activation_subscriptions: Mutex::new(SmallVec::new()),
            status: AtomicU8::new(ServiceStatus::Inactive.as_u8()),
        });

        let activation_callback = |app_handle: &AppHandle,
                                   service: Arc<T>,
                                   service_state: Arc<ServiceHandleState>,
                                   phase: &LifecyclePhase| {
            if !service_state.is_activation_possible() {
                return;
            }

            trace!(
                "Starting activation process for service: {} during phase: {:?}",
                T::service_brand(),
                phase
            );

            service_state.set_status(ServiceStatus::Activating);
            service.start(app_handle);
            service_state.set_status(ServiceStatus::Active);

            // Once the service is successfully activated, all other activation
            // callbacks are no longer required and will be removed.
            service_state.activation_subscriptions.lock().clear();
        };

        let service_state_clone = Arc::clone(&service_state);
        let mut activation_subscriptions_lock = service_state_clone.activation_subscriptions.lock();

        for (index, point) in activation_points.into_iter().enumerate() {
            let service_arc_clone = Arc::clone(&service_arc);
            let service_state_clone = Arc::clone(&service_state);
            let phase: LifecyclePhase = point.into();

            activation_subscriptions_lock.insert(
                index,
                self.lifecycle_manager
                    .observe(phase.clone(), move |app_handle| {
                        activation_callback(
                            app_handle,
                            service_arc_clone.to_owned(),
                            service_state_clone.to_owned(),
                            &phase,
                        );
                    }),
            );
        }

        self.services.write().insert(
            TypeId::of::<T>(),
            ServiceHandle {
                service: service_arc,
                state: service_state,
            },
        );
    }

    pub fn get<T: AnyService + ServiceMetadata>(&self) -> Option<Arc<T>> {
        let Some(service_handle) = self.services.read().get(&TypeId::of::<T>()).cloned() else {
            return None;
        };

        if !service_handle.state.is_active() {
            warn!(
                "Attempting to retrieve service {} which has not yet been activated",
                T::service_brand(),
            );
            return None;
        }

        match service_handle.service.downcast_arc::<T>() {
            Ok(service_arc) => Some(service_arc),
            Err(_) => {
                debug!(
                    "Failed to cast service {} to the required type",
                    T::service_brand(),
                );

                None
            }
        }
    }

    pub fn get_unchecked<T: AnyService + ServiceMetadata>(&self) -> Arc<T> {
        let service_handle = self
            .services
            .read()
            .get(&TypeId::of::<T>())
            .cloned()
            .unwrap_or_else(|| panic!(
                "Service {} could not be found. Ensure it has been properly registered before attempting to retrieve it.", 
                T::service_brand(),
            ));

        if !service_handle.state.is_active() {
            panic!(
                "Attempting to retrieve service {} which has not yet been activated",
                T::service_brand(),
            );
        }

        service_handle
            .service
            .downcast_arc::<T>()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to cast service {} to the required type",
                    T::service_brand(),
                )
            })
    }
}
