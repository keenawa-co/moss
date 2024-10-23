use hashbrown::HashMap;
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
pub struct TreeView {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TreeViewContainerLocation {
    PrimaryBar,
    SecondaryBar,
}

#[derive(Debug)]
pub struct ViewsRegistry {
    view_containers: HashMap<TreeViewContainerLocation, Vec<TreeViewContainer>>,
    views: HashMap<ContainerId, Vec<TreeView>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            view_containers: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn register_container(
        &mut self,
        location: TreeViewContainerLocation,
        container: TreeViewContainer,
    ) -> Result<ContainerId, ViewsRegistryError> {
        let container_id = container.id;

        self.view_containers
            .entry(location)
            .or_insert_with(Vec::new)
            .push(container);
        self.views.entry(container_id).or_insert_with(Vec::new);

        Ok(container_id)
    }

    pub(crate) fn register_views(
        &mut self,
        id: &ContainerId,
        batch: impl IntoIterator<Item = TreeView>,
    ) -> Result<(), ViewsRegistryError> {
        let container_views = self
            .views
            .get_mut(id)
            .ok_or_else(|| ViewsRegistryError::ContainerNotFound(id))?;
        container_views.extend(batch);

        Ok(())
    }

    pub(crate) fn get_views_by_container_id(
        &self,
        container_id: &ContainerId,
    ) -> Option<&Vec<TreeView>> {
        self.views.get(container_id)
    }

    pub(crate) fn get_containers_by_location(
        &self,
        location: &TreeViewContainerLocation,
    ) -> Option<&Vec<TreeViewContainer>> {
        self.view_containers.get(location)
    }
}
