use moss_jsonlogic::raw_rule::*;
use moss_jsonlogic_macro::rule;
use moss_text::{localize, read_only_str};

use crate::{
    contribution::MenuDecl,
    contribution_point,
    models::{
        actions::{
            ActionMenuItem, CommandAction, CommandActionToggle, MenuGroup, MenuItem,
            MenuItemVisibility, SubmenuMenuItem,
        },
        constants,
    },
};

contribution_point!(LAYOUT_CONTROLS, {
    menus: [
        MenuDecl {
            namespace: constants::menu::MENU_NAMESPACE_ID_HEAD_ITEM,
            items: vec![
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: read_only_str!("workbench.action.toggleSecondarySidebar"),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: rule!(toggleSecondarySidebar == true),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(MenuGroup::unordered("layoutControls")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: read_only_str!("workbench.action.togglePrimarySideBar"),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: rule!(toggleSecondarySidebar == true),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(MenuGroup::unordered("layoutControls")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: read_only_str!("workbench.action.togglePanel"),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: Some(CommandActionToggle {
                            condition: rule!(togglePanel == true),
                            icon: None,
                            tooltip: None,
                            title: None,
                        }),
                    },
                    group: Some(MenuGroup::unordered("layoutControls")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
                //
                // Customize Layout
                //
                MenuItem::Submenu(SubmenuMenuItem {
                    submenu_id: read_only_str!("layoutControls.customizeLayout"),
                    default_action_id: None,
                    title: Some(localize!(
                        "layoutControls.customizeLayout",
                        "Customize Layout"
                    )),
                    group: Some(MenuGroup::unordered("layoutControls")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Compact,
                }),
            ],
        },
        MenuDecl {
            namespace: read_only_str!("layoutControls.customizeLayout"), // TODO: ref
            items: vec![
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
                    group: Some(MenuGroup::ordered(0, "visibility")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Classic,
                }),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: read_only_str!("workbench.action.primarySideBarVisibility"),
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
                    group: Some(MenuGroup::ordered(0, "visibility")),
                    order: None,
                    when: None,
                    visibility: MenuItemVisibility::Classic,
                }),
            ],
        },
    ],
});
