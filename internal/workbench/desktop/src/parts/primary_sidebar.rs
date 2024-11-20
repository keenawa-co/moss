use crate::{
    menu::{BuiltInMenuNamespaces, MenuItem},
    view::{TreeViewGroupLocation, TreeViewOutput},
};

use super::{AnyPart, PartId, Parts};
use anyhow::Result;
use hashbrown::HashMap;

#[derive(Debug, Serialize)]
pub struct DescribeSideBarPartOutput {
    pub views: HashMap<String, Vec<TreeViewOutput>>,
    pub menus: HashMap<String, Vec<MenuItem>>,
}

pub struct PrimarySideBarPart {}

impl PrimarySideBarPart {
    pub fn new() -> Self {
        Self {}
    }
}

impl AnyPart for PrimarySideBarPart {
    const ID: PartId = Parts::PRIMARY_SIDE_BAR;
    type DescribeOutput = DescribeSideBarPartOutput;

    fn describe(&self, registry: &crate::RegistryManager) -> Result<Self::DescribeOutput> {
        let mut views = HashMap::new();
        let views_registry_lock = registry.views.read();

        if let Some(containers) =
            views_registry_lock.get_groups_by_location(&TreeViewGroupLocation::PrimaryBar)
        {
            for container in containers {
                if let Some(view_descriptors) =
                    views_registry_lock.get_view_descriptors_by_group_id(&container.id)
                {
                    views
                        .entry(container.id.to_string())
                        .or_insert_with(Vec::new)
                        .extend(view_descriptors.iter().map(|descriptor| descriptor.into()))
                }
            }
        }

        let mut menus = HashMap::new();

        let menus_lock = registry.menus.read();
        menus.insert(
            BuiltInMenuNamespaces::ViewItemContext.to_string(),
            menus_lock
                .get_menu_items_by_namespace(&BuiltInMenuNamespaces::ViewItemContext.into())
                .cloned()
                .unwrap(),
        );

        menus.insert(
            BuiltInMenuNamespaces::ViewItem.to_string(),
            menus_lock
                .get_menu_items_by_namespace(&BuiltInMenuNamespaces::ViewItem.into())
                .cloned()
                .unwrap(),
        );

        menus.insert(
            BuiltInMenuNamespaces::ViewTitleContext.to_string(),
            menus_lock
                .get_menu_items_by_namespace(&BuiltInMenuNamespaces::ViewTitleContext.into())
                .cloned()
                .unwrap(),
        );

        Ok(DescribeSideBarPartOutput { views, menus })
    }
}
