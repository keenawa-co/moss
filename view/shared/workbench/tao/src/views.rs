use hashbrown::{HashMap, HashSet};
use std::fmt::Debug;
use thiserror::Error;

/// Represents a container that holds views.
#[derive(Serialize, Debug, Clone)]
pub struct TreeViewContainer {
    pub id: ContainerId,
    pub name: String,
    pub order: usize,
}

/// Describes a single view.
#[derive(Serialize, Debug, Clone)]
pub struct TreeViewDescriptor {
    pub id: String,
    pub title: String,
    pub order: usize,
}

pub trait Registry<K, V> {
    fn register(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn all(&self) -> Vec<&V>;
}

// Type aliases for better readability.

pub type GroupKey = &'static str;
pub type ContainerId = &'static str;

/// Custom errors for ViewsRegistry operations.

#[derive(Debug, Error)]
pub enum ViewsRegistryError {
    #[error("Container group '{0}' already exists.")]
    GroupAlreadyExists(GroupKey),

    #[error("Container group '{0}' does not exist.")]
    GroupNotFound(GroupKey),

    #[error("Container '{0}' already exists.")]
    ContainerAlreadyExists(ContainerId),

    #[error("Container '{0}' does not exist.")]
    ContainerNotFound(ContainerId),
}

#[derive(Debug)]
pub struct ViewsRegistry {
    view_container_groups: HashMap<GroupKey, HashSet<ContainerId>>,
    view_containers: HashMap<ContainerId, TreeViewContainer>,
    views: HashMap<ContainerId, Vec<TreeViewDescriptor>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            view_container_groups: HashMap::new(),
            view_containers: HashMap::new(),
            views: HashMap::new(),
        }
    }

    /// Registers a new container group.
    ///
    /// Returns an error if the group already exists.
    pub(crate) fn register_container_group(
        &mut self,
        id: GroupKey,
    ) -> Result<(), ViewsRegistryError> {
        if self.view_container_groups.contains_key(&id) {
            return Err(ViewsRegistryError::GroupAlreadyExists(id));
        }
        self.view_container_groups.insert(id, HashSet::new());

        Ok(())
    }

    /// Registers a new container.
    ///
    /// Returns an error if the container already exists.
    pub(crate) fn register_container(
        &mut self,
        container: TreeViewContainer,
    ) -> Result<(), ViewsRegistryError> {
        let container_id = container.id;
        if self.view_containers.contains_key(container_id) {
            return Err(ViewsRegistryError::ContainerAlreadyExists(container_id));
        }

        self.view_containers.insert(container_id, container);
        self.views.entry(container_id).or_insert_with(Vec::new);

        Ok(())
    }

    /// Associates a container with a specific group.
    ///
    /// Returns an error if either the group or container does not exist.
    pub(crate) fn add_container_to_group(
        &mut self,
        group_id: &GroupKey,
        container_id: &ContainerId,
    ) -> Result<(), ViewsRegistryError> {
        let group = self
            .view_container_groups
            .get_mut(group_id)
            .ok_or_else(|| ViewsRegistryError::GroupNotFound(group_id))?;
        if !self.view_containers.contains_key(container_id) {
            return Err(ViewsRegistryError::ContainerNotFound(container_id));
        }
        group.insert(container_id);

        Ok(())
    }

    /// Registers a batch of views for a specific container.
    ///
    /// Returns an error if the container does not exist.
    pub(crate) fn register_batch_view(
        &mut self,
        id: &ContainerId,
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) -> Result<(), ViewsRegistryError> {
        let container_views = self
            .views
            .get_mut(id)
            .ok_or_else(|| ViewsRegistryError::ContainerNotFound(id))?;
        container_views.extend(batch);

        Ok(())
    }

    /// Retrieves the views associated with a specific container.
    pub(crate) fn get_views_by_container_id(
        &self,
        container_id: &ContainerId,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(container_id)
    }

    /// Retrieves all containers associated with a specific group.
    pub(crate) fn get_containers_by_group_id(
        &self,
        group_id: &GroupKey,
    ) -> Option<Vec<TreeViewContainer>> {
        self.view_container_groups
            .get(group_id)
            .map(|container_ids| {
                container_ids
                    .iter()
                    .filter_map(|id| self.view_containers.get(id).cloned())
                    .collect()
            })
    }
}
