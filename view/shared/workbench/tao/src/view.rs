use hashbrown::HashMap;
use std::fmt::Debug;
use thiserror::Error;

/// Represents a group that holds views.
#[derive(Serialize, Debug, Clone)]
pub struct TreeViewGroup {
    pub id: GroupId,
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
pub type GroupId = &'static str;

/// Custom errors for ViewsRegistry operations.

#[derive(Debug, Error)]
pub enum ViewsRegistryError {
    #[error("Group '{0}' already exists.")]
    GroupAlreadyExists(GroupId),

    #[error("Group '{0}' does not exist.")]
    GroupNotFound(GroupId),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TreeViewGroupLocation {
    PrimaryBar,
    SecondaryBar,
}

#[derive(Debug)]
pub struct ViewsRegistry {
    view_groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<GroupId, Vec<TreeView>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            view_groups: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn register_group(
        &mut self,
        location: TreeViewGroupLocation,
        group: TreeViewGroup,
    ) -> Result<(), ViewsRegistryError> {
        let group_id = group.id;

        self.view_groups
            .entry(location)
            .or_insert_with(Vec::new)
            .push(group);
        self.views.entry(group_id).or_insert_with(Vec::new);

        Ok(())
    }

    pub(crate) fn register_views(
        &mut self,
        id: &GroupId,
        batch: impl IntoIterator<Item = TreeView>,
    ) -> Result<(), ViewsRegistryError> {
        let group_views = self
            .views
            .get_mut(id)
            .ok_or_else(|| ViewsRegistryError::GroupNotFound(id))?;
        group_views.extend(batch);

        Ok(())
    }

    pub(crate) fn get_views_by_group_id(&self, id: &GroupId) -> Option<&Vec<TreeView>> {
        self.views.get(id)
    }

    pub(crate) fn get_groups_by_location(
        &self,
        location: &TreeViewGroupLocation,
    ) -> Option<&Vec<TreeViewGroup>> {
        self.view_groups.get(location)
    }
}
