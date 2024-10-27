use hashbrown::HashMap;
use once_cell::sync::{Lazy, OnceCell};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Debug,
};
use thiserror::Error;

pub type GroupId = &'static str;
pub type ViewId = &'static str;

pub trait AnyContentProvider {
    type ContentOutput;

    fn content(&self) -> Self::ContentOutput;
}

#[derive(Serialize, Debug, Clone)]
pub struct TreeViewGroup {
    pub id: GroupId,
    pub name: String,
    pub order: usize,
}

pub struct TreeViewDescriptor {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
    pub collapsed: bool,
    pub model: Lazy<Box<dyn Any>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TreeViewOutput {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
    pub collapsed: bool,
}

impl From<&TreeViewDescriptor> for TreeViewOutput {
    fn from(value: &TreeViewDescriptor) -> Self {
        TreeViewOutput {
            id: value.id.clone(),
            name: value.name.clone(),
            order: value.order,
            hide_by_default: value.hide_by_default,
            can_toggle_visibility: value.can_toggle_visibility,
            collapsed: value.collapsed,
        }
    }
}

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

pub struct ViewsRegistry {
    view_groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<GroupId, Vec<TreeViewDescriptor>>,
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
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) -> Result<(), ViewsRegistryError> {
        let group_views = self
            .views
            .get_mut(id)
            .ok_or_else(|| ViewsRegistryError::GroupNotFound(id))?;
        group_views.extend(batch);

        Ok(())
    }

    pub(crate) fn get_view_descriptors_by_group_id(
        &self,
        id: &GroupId,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(id)
    }

    pub(crate) fn get_view_model<T: 'static>(
        &self,
        group_id: GroupId,
        view_id: String,
    ) -> Option<&T> {
        self.views
            .get(&group_id)?
            .iter()
            .find(|item| item.id == view_id)
            .and_then(|item| item.model.downcast_ref::<T>())
    }

    pub(crate) fn get_groups_by_location(
        &self,
        location: &TreeViewGroupLocation,
    ) -> Option<&Vec<TreeViewGroup>> {
        self.view_groups.get(location)
    }
}
