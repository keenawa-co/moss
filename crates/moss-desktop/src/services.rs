pub mod addon_service;
pub mod theme_service;

use anyhow::Result;
use async_trait::async_trait;
use fnv::FnvHashMap;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use strum::{EnumCount, EnumIter};
use tauri::AppHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub const MAX_ACTIVATION_POINTS: usize = ActivationPoint::COUNT;

#[derive(Debug, Eq, Hash, PartialEq, EnumIter, EnumCount)]
pub enum ActivationPoint {
    OnStartUp,
}

impl std::str::FromStr for ActivationPoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OnStartUp" => Ok(ActivationPoint::OnStartUp),
            _ => Err(format!("Unknown ActivationPoint: {}", s)),
        }
    }
}

impl std::fmt::Display for ActivationPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivationPoint::OnStartUp => write!(f, "OnStartUp"),
        }
    }
}

pub enum LifecycleEvent {
    Activation(ActivationPoint),
}

pub enum ServiceManagerEvent {
    Lifecycle(LifecycleEvent),
}

pub enum ServiceEvent {
    Activation,
}

#[async_trait]
pub trait AnyService: Any + Send + Sync {
    async fn on_event(&self, app_handle: AppHandle, event: ServiceEvent);
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

pub struct ServiceManager {
    services: Arc<RwLock<FnvHashMap<TypeId, Arc<dyn AnyService>>>>,
    activation_queue: Arc<RwLock<FnvHashMap<ActivationPoint, Vec<TypeId>>>>,
    service_lifecycle_tx: UnboundedSender<ServiceManagerEvent>,
    service_lifecycle_rx: Mutex<Option<UnboundedReceiver<ServiceManagerEvent>>>,
}

impl<'a> ServiceManager {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            services: Arc::new(RwLock::new(FnvHashMap::default())),
            activation_queue: Arc::new(RwLock::new(FnvHashMap::default())),
            service_lifecycle_tx: tx,
            service_lifecycle_rx: Mutex::new(Some(rx)),
        }
    }

    pub fn run(&self, app_handle: AppHandle) {
        let mut receiver_lock = self.service_lifecycle_rx.lock();
        let receiver = receiver_lock.take().expect("run can only be called once");

        let services = Arc::clone(&self.services);
        let activation_queue = Arc::clone(&self.activation_queue);

        tauri::async_runtime::spawn(async move {
            let mut receiver = receiver;
            while let Some(event) = receiver.recv().await {
                match &event {
                    ServiceManagerEvent::Lifecycle(lifecycle_event) => match lifecycle_event {
                        LifecycleEvent::Activation(activation_point) => {
                            let services_to_call = {
                                let services_lock = services.read();
                                if let Some(service_ids) =
                                    activation_queue.read().get(&activation_point).cloned()
                                {
                                    service_ids
                                        .into_iter()
                                        .filter_map(|service_type_id| {
                                            services_lock.get(&service_type_id).cloned()
                                        })
                                        .collect::<Vec<Arc<dyn AnyService>>>()
                                } else {
                                    Vec::new()
                                }
                            };

                            for service in services_to_call {
                                service
                                    .on_event(app_handle.clone(), ServiceEvent::Activation)
                                    .await;
                            }
                        }
                    },
                }
            }
        });
    }

    pub fn emit(&self, event: ServiceManagerEvent) -> Result<()> {
        Ok(self.service_lifecycle_tx.send(event)?)
    }

    pub fn register<T: AnyService>(
        &self,
        service: T,
        at_point: SmallVec<[ActivationPoint; MAX_ACTIVATION_POINTS]>,
    ) {
        let type_id = TypeId::of::<T>();
        self.services.write().insert(type_id, Arc::new(service));

        let mut activation_queue_lock = self.activation_queue.write();
        for activation_event in at_point {
            activation_queue_lock
                .entry(activation_event)
                .or_insert_with(Vec::new)
                .push(type_id);
        }
    }

    pub fn get<T: AnyService>(&self) -> Option<Arc<T>> {
        let any_service = self.services.read().get(&TypeId::of::<T>()).cloned()?;

        match any_service.downcast_arc::<T>() {
            Ok(arc_t) => Some(arc_t),
            Err(_) => None,
        }
    }

    pub fn get_unchecked<T: AnyService>(&self) -> Arc<T> {
        let any_service = self
            .services
            .read()
            .get(&TypeId::of::<T>())
            .cloned()
            .unwrap_or_else(|| panic!("Service not found"));

        any_service
            .downcast_arc::<T>()
            .unwrap_or_else(|_| panic!("Service type mismatch"))
    }
}
