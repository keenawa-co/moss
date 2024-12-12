use desktop_models::{
    actions::MenuItem,
    view::{GroupId, TreeViewDescriptor, TreeViewGroup, TreeViewGroupLocation},
};
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::sync::Arc;

pub struct MenuRegistry {
    menus: HashMap<ReadOnlyStr, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: ReadOnlyStr, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (ReadOnlyStr, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items_by_namespace(&self, namespace: &ReadOnlyStr) -> Option<&Vec<MenuItem>> {
        self.menus.get(namespace)
    }
}

#[derive(Debug)]
pub struct ViewsRegistry {
    groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            groups: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn append_view_group(
        &mut self,
        location: TreeViewGroupLocation,
        group: TreeViewGroup,
    ) {
        self.groups
            .entry(location)
            .or_insert_with(Vec::new)
            .push(group);
    }

    pub(crate) fn register_views(
        &mut self,
        id: ReadOnlyStr,
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) {
        self.views.entry(id).or_insert_with(Vec::new).extend(batch);
    }

    pub(crate) fn get_view_descriptors_by_group_id(
        &self,
        id: &ReadOnlyStr,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(id)
    }

    pub(crate) fn get_view_model<T: Send + Sync + Debug + 'static>(
        &self,
        group_id: impl Into<ReadOnlyStr>,
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

pub struct RegistryManager {
    pub views: Arc<RwLock<ViewsRegistry>>,
    pub menus: Arc<RwLock<MenuRegistry>>,
}

impl RegistryManager {
    pub fn new() -> Self {
        let views = Arc::new(RwLock::new(ViewsRegistry::new()));
        let menus = Arc::new(RwLock::new(MenuRegistry::new()));

        Self { views, menus }
    }
}
