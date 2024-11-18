use desktop_models::{
    actions::{
        ActionMenuItem, CommandAction, CommandActionToggle, MenuGroup, MenuItem,
        MenuItemVisibility, SubmenuMenuItem, SubmenuRef,
    },
    constants,
};
use moss_str::{localize, ReadOnlyStr};
use std::rc::Rc;

use crate::Contribution;

pub struct LayoutControlsContribution;
impl Contribution for LayoutControlsContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        /* ---------- Menus contributions ---------- */

        let mut menus_registry_lock = registry.menus.write();

        let head_item_menu_group_layout_controls =
            Rc::new(MenuGroup::new_unordered("layoutControls"));

        menus_registry_lock.append_menu_items(vec![
            (
                constants::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.action.togglePrimarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: "undefined".into(),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(Rc::clone(&head_item_menu_group_layout_controls)),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
            ),
            //
            // Toggle Secondary Side Bar
            //
            (
                constants::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.action.toggleSecondarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: "undefined".into(),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(Rc::clone(&head_item_menu_group_layout_controls)),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
            ),
            (
                constants::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.action.togglePanel".into(),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: "undefined".into(),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(Rc::clone(&head_item_menu_group_layout_controls)),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
            ),
        ]);

        //
        // Customize Layout
        //
        let customize_layout_menu_id = ReadOnlyStr::from("layoutControls.customizeLayout");
        menus_registry_lock.append_menu_item(
            constants::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
            MenuItem::Submenu(SubmenuMenuItem {
                submenu_id: customize_layout_menu_id.clone(),
                default_action_id: None,
                title: Some(localize!(
                    "layoutControls.customizeLayout",
                    "Customize Layout"
                )),
                group: Some(Rc::clone(&head_item_menu_group_layout_controls)),
                order: None,
                when: None,
                visibility: MenuItemVisibility::Compact,
            }),
        );

        // Customize Layout Submenu Items

        #[rustfmt::skip]
        let (
            customize_layout_group_visibility,
            _customize_layout_group_panel_alignment, // TODO: add the corresponding menu items that will utilize this group.
            _customize_layout_group_primary_sidebar_position, // TODO: add the corresponding menu items that will utilize this group.
            _customize_layout_group_panel_modes, // TODO: add the corresponding menu items that will utilize this group.
        ) = {
            let visibility = Rc::new(MenuGroup::new_ordered(0, "visibility"));
            let panel_alignment = Rc::new(MenuGroup::new_ordered(0, "panelAlignment"));
            let primary_sidebar_position = Rc::new(MenuGroup::new_ordered(0, "primarySideBarPosition"));
            let modes = Rc::new(MenuGroup::new_ordered(0, "modes"));

            (visibility, panel_alignment, primary_sidebar_position, modes)
        };

        menus_registry_lock.append_menu_items(vec![
            (
                SubmenuRef::from(&customize_layout_menu_id),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.action.activityBarVisibility".into(),
                        title: localize!("customizeLayout.activityBarVisibility", "Activity Bar"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: "undefined".into(),
                            title: None,
                            tooltip: None,
                            icon: None,
                        }),
                    },
                    group: Some(Rc::clone(&customize_layout_group_visibility)),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                SubmenuRef::from(&customize_layout_menu_id),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.action.primarySideBarVisibility".into(),
                        title: localize!(
                            "customizeLayout.primarySideBarVisibility",
                            "Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: "undefined".into(),
                            title: None,
                            tooltip: None,
                            icon: None,
                        }),
                    },
                    group: Some(Rc::clone(&customize_layout_group_visibility)),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        drop(menus_registry_lock);

        Ok(())
    }
}
