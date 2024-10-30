use hashbrown::HashMap;
use once_cell::sync::Lazy;
use std::{any::Any, fmt::Debug, sync::Arc};

use crate::util::ReadOnlyId;

#[derive(Serialize, Debug, Clone)]
pub struct TreeViewGroup {
    pub id: ReadOnlyId,
    pub name: String,
    pub order: usize,
}

#[derive(Debug)]
pub struct TreeViewDescriptor {
    pub id: String,
    pub name: String,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
    pub collapsed: bool,
    pub model: Lazy<Arc<dyn Any + Send + Sync>>,
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TreeViewGroupLocation {
    PrimaryBar,
    SecondaryBar,
}

#[derive(Debug)]
pub struct ViewsRegistry {
    groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<ReadOnlyId, Vec<TreeViewDescriptor>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            groups: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn register_views(
        &mut self,
        id: ReadOnlyId,
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) {
        self.views.entry(id).or_insert_with(Vec::new).extend(batch);
    }

    pub(crate) fn get_view_descriptors_by_group_id(
        &self,
        id: &ReadOnlyId,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(id)
    }

    pub(crate) fn get_view_model<T: Send + Sync + Debug + 'static>(
        &self,
        group_id: impl Into<ReadOnlyId>,
        view_id: String,
    ) -> Option<Arc<T>> {
        self.views
            .get(&group_id.into())?
            .iter()
            .find(|item| item.id == view_id)
            .and_then(|item| Arc::downcast::<T>(Arc::clone(&item.model)).ok())
    }

    pub(crate) fn get_groups_by_location(
        &self,
        location: &TreeViewGroupLocation,
    ) -> Option<&Vec<TreeViewGroup>> {
        self.groups.get(location)
    }
}
