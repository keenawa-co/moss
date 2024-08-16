use fnv::FnvHashMap;
use std::any::{Any, TypeId};

#[derive(Default)]
pub struct ServiceGroup(FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl std::ops::Deref for ServiceGroup {
    type Target = FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for ServiceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ServiceGroup").finish()
    }
}

impl<'a> ServiceGroup {
    pub fn new() -> Self {
        Self(FnvHashMap::default())
    }

    pub fn has<T: Any + Send + Sync>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Any + Send + Sync>(&mut self, service: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get_unchecked<T: Any + Send + Sync>(&'a self) -> &'a T {
        self.get_opt::<T>().unwrap_or_else(|| {
            panic!(
                "Service with type `{}` does not exist",
                std::any::type_name::<T>()
            )
        })
    }

    pub fn get_opt<T: Any + Send + Sync>(&'a self) -> Option<&'a T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|d| d.downcast_ref::<T>())
    }
}
