use moss_str::{localize, ReadOnlyStr};

use crate::{
    menu::{ActionMenuItem, CommandAction, MenuItem, SubmenuMenuItem},
    Contribution,
};

pub struct LayoutControlsContribution;
impl Contribution for LayoutControlsContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        /* ---------- Menus contributions ---------- */

        let mut menus_registry_lock = registry.menus.write();

        menus_registry_lock.append_menu_items(vec![
            //
            // Toggle Primary Side Bar
            //
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openPrimarySidebar".into(),
                        title: localize!("recents.togglePrimarySideBar", "Toggle Primary Side Bar"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closePrimarySidebar".into(),
                        title: localize!("recents.togglePrimarySideBar", "Toggle Primary Side Bar"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            //
            // Toggle Secondary Side Bar
            //
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openSecondarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closeSecondarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            //
            // Toggle Panel
            //
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openPanel".into(),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closePanel".into(),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
        ]);

        //
        // Customize Layout
        //
        let customize_layout_menu_id = ReadOnlyStr::from("layoutControls.customizeLayout");
        menus_registry_lock.append_menu_item(
            crate::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
            MenuItem::Submenu(SubmenuMenuItem {
                submenu_id: customize_layout_menu_id,
                default_action_id: None,
                title: Some(localize!(
                    "layoutControls.customizeLayout",
                    "Customize Layout"
                )),
                group: None,
                order: None,
                when: None,
            }),
        );

        drop(menus_registry_lock);

        Ok(())
    }
}
