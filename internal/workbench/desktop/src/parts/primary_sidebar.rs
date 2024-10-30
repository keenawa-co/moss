use std::sync::Arc;

use crate::{
    menu::{BuiltInMenus, MenuItem, MenuService},
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

pub struct PrimarySideBarPart {
    menu_service: Arc<MenuService>,
}

impl PrimarySideBarPart {
    pub fn new(menu_service: Arc<MenuService>) -> Self {
        Self { menu_service }
    }
}

impl AnyPart for PrimarySideBarPart {
    const ID: PartId = Parts::PRIMARY_SIDE_BAR;
    type DescribeOutput = DescribeSideBarPartOutput;

    fn describe(&self, registry: &crate::RegistryManager) -> Result<Self::DescribeOutput> {
        let mut views = HashMap::new();
        let views_registry_lock = registry.views.read();

        dbg!(&views_registry_lock);

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
            BuiltInMenus::ViewItemContext.to_string(),
            menus_lock
                .get_menu_items(&BuiltInMenus::ViewItemContext.into())
                .cloned()
                .unwrap(),
        );

        menus.insert(
            BuiltInMenus::ViewItem.to_string(),
            menus_lock
                .get_menu_items(&BuiltInMenus::ViewItem.into())
                .cloned()
                .unwrap(),
        );

        menus.insert(
            BuiltInMenus::ViewTitleContext.to_string(),
            menus_lock
                .get_menu_items(&BuiltInMenus::ViewTitleContext.into())
                .cloned()
                .unwrap(),
        );

        Ok(DescribeSideBarPartOutput { views, menus })
    }
}
