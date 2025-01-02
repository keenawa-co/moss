use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use fnv::FnvHashMap;
use parking_lot::RwLock;
use std::{
    any::{Any, TypeId},
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};
use strum_macros::FromRepr;
use tauri::AppHandle;

use super::instantiation::InstantiationType;

pub trait Service: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn dispose(&self);
    fn as_any(&self) -> &dyn Any;
}

impl dyn Service {
    pub fn downcast_arc<T: Service>(self: Arc<Self>) -> Result<Arc<T>, Arc<Self>> {
        if self.as_any().is::<T>() {
            let raw = Arc::into_raw(self) as *const T;
            Ok(unsafe { Arc::from_raw(raw) })
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct ServiceHandle<T>
where
    T: Service,
{
    #[deref]
    #[deref_mut]
    service: Arc<T>,

    #[allow(unused)]
    metadata: Arc<ServiceMetadata>,
}

#[repr(u8)]
#[derive(Debug, Eq, Hash, PartialEq, FromRepr)]
enum ServiceInstantiationMode {
    Pending = 0,
    Active = 1,
}

impl ServiceInstantiationMode {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

impl From<InstantiationType> for ServiceInstantiationMode {
    fn from(value: InstantiationType) -> Self {
        match value {
            InstantiationType::Instant => ServiceInstantiationMode::Active,
            InstantiationType::Delayed => ServiceInstantiationMode::Pending,
        }
    }
}

impl Into<AtomicU8> for ServiceInstantiationMode {
    fn into(self) -> AtomicU8 {
        AtomicU8::new(self as u8)
    }
}

#[derive(Debug)]
struct ServiceMetadata {
    service_name: &'static str,
    instantiation_mode: AtomicU8,
}

impl ServiceMetadata {
    fn set_instantiation_mode(&self, mode: ServiceInstantiationMode) {
        self.instantiation_mode
            .store(mode.as_u8(), Ordering::SeqCst);
    }

    fn get_instantiation_mode(&self) -> ServiceInstantiationMode {
        ServiceInstantiationMode::from_repr(self.instantiation_mode.load(Ordering::SeqCst))
            .expect("Valid value for ServiceInstantiationMode: {value}")
    }
}

struct ServiceCollectionState {
    services: FnvHashMap<TypeId, Arc<dyn Service>>,
    pending_services: FnvHashMap<TypeId, Box<dyn FnOnce(&AppHandle) -> Arc<dyn Service>>>,
    known_services: FnvHashMap<TypeId, Arc<ServiceMetadata>>,
}

impl Default for ServiceCollectionState {
    fn default() -> Self {
        Self {
            services: Default::default(),
            pending_services: Default::default(),
            known_services: Default::default(),
        }
    }
}

pub struct ServiceCollection {
    app_handle: AppHandle,
    state: RwLock<ServiceCollectionState>,
}

impl ServiceCollection {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            state: RwLock::new(ServiceCollectionState::default()),
        }
    }

    pub fn register<T, F>(&self, creation_fn: F, activation_type: InstantiationType)
    where
        T: Service + 'static,
        F: FnOnce(&AppHandle) -> T + 'static,
    {
        let type_id = TypeId::of::<T>();
        let service_name = std::any::type_name::<T>();
        let mut state_lock = self.state.write();

        match activation_type {
            InstantiationType::Instant => {
                let service = creation_fn(&self.app_handle);

                state_lock
                    .services
                    .insert(type_id.clone(), Arc::new(service));

                debug!("Service {service_name} was activated");
            }
            InstantiationType::Delayed => {
                state_lock.pending_services.insert(
                    type_id.clone(),
                    Box::new(move |app_handle| {
                        let service = creation_fn(app_handle);

                        Arc::new(service)
                    }),
                );
            }
        }

        state_lock.known_services.insert(
            type_id,
            Arc::new(ServiceMetadata {
                service_name,
                instantiation_mode: ServiceInstantiationMode::from(activation_type).into(),
            }),
        );
    }

    fn get_internal(
        &self,
        service_metadata: Arc<ServiceMetadata>,
        type_id: TypeId,
    ) -> Result<Arc<dyn Service>> {
        match service_metadata.get_instantiation_mode() {
            ServiceInstantiationMode::Active => {
                let state_lock = self.state.write();
                let service = state_lock.services.get(&type_id).ok_or_else(|| {
                    anyhow!(
                        "The service {} was not found among the activated ones",
                        service_metadata.service_name
                    )
                })?;

                Ok(Arc::clone(service))
            }
            ServiceInstantiationMode::Pending => {
                let mut state_lock = self.state.write();
                let creation_fn =
                    state_lock
                        .pending_services
                        .remove(&type_id)
                        .ok_or_else(|| {
                            anyhow!(
                                "The service {} was not found among those awaiting activation",
                                service_metadata.service_name
                            )
                        })?;

                let service = creation_fn(&self.app_handle);

                state_lock.services.insert(type_id, Arc::clone(&service));
                service_metadata.set_instantiation_mode(ServiceInstantiationMode::Active);
                debug!(
                    "Service {} was activated upon request",
                    service_metadata.service_name
                );

                Ok(service)
            }
        }
    }

    pub fn get<T: Service>(&self) -> Result<ServiceHandle<T>> {
        let type_id = TypeId::of::<T>();
        let service_metadata = self
            .state
            .write()
            .known_services
            .get(&type_id)
            .cloned()
            .ok_or_else(|| {
                anyhow!(
                    "The service {} must be registered before it can be used",
                    std::any::type_name::<T>()
                )
            })?;

        // A panic here is likely an indicator of a bug in the program because if
        // the service was "known", this panic should not occur in this location.
        let any_service = self
            .get_internal(Arc::clone(&service_metadata), type_id)
            .unwrap();

        if let Ok(service) = any_service.downcast_arc::<T>() {
            Ok(ServiceHandle {
                service,
                metadata: service_metadata,
            })
        } else {
            Err(anyhow!(
                "Failed to cast service {} to the required type",
                service_metadata.service_name,
            ))
        }
    }

    pub fn get_unchecked<T: Service>(&self) -> ServiceHandle<T> {
        let type_id = TypeId::of::<T>();
        let service_metadata = self
            .state
            .write()
            .known_services
            .get(&type_id)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "The service {} must be registered before it can be used",
                    std::any::type_name::<T>()
                )
            });

        // A panic here is likely an indicator of a bug in the program because if the service was
        // known and activated, this panic should not occur in this location.
        let any_service = self
            .get_internal(Arc::clone(&service_metadata), type_id)
            .unwrap();

        let service = any_service.downcast_arc::<T>().unwrap_or_else(|_| {
            panic!(
                "Failed to cast service {} to the required type",
                service_metadata.service_name,
            )
        });

        ServiceHandle {
            service,
            metadata: service_metadata,
        }
    }
}
