use crate::{
    menu::{
        ActionMenuItem, CommandAction, MenuId, MenuItem, Menus, SubmenuMenuItem, ToggledMenuItem,
    },
    view::{AnyContentProvider, TreeViewDescriptor},
    Contribution,
};
use anyhow::Result;
use once_cell::sync::Lazy;
use quote::quote;
use static_str_ops::static_format;

#[derive(Debug, Serialize)]
pub struct RecentsViewTreeItem {
    pub path: String,
    pub last_modification: String,
}

#[derive(Debug, Serialize)]
pub struct RecentsViewContentProviderOutput {
    pub data: Vec<RecentsViewTreeItem>,
    pub html: String,
}

pub struct RecentsViewModel {}

impl RecentsViewModel {
    pub fn content(&self) -> Result<RecentsViewContentProviderOutput> {
        let tokens = quote! { <p className="text-sm">"Hello, World!"</p> };

        Ok(RecentsViewContentProviderOutput {
            html: tokens.to_string(),
            data: vec![
                RecentsViewTreeItem {
                    path: "~/keenawa/moss".to_string(),
                    last_modification: "14 min ago".to_string(),
                },
                RecentsViewTreeItem {
                    path: "~/zigland/zig".to_string(),
                    last_modification: "18 hours ago".to_string(),
                },
            ],
        })
    }
}

pub(crate) struct RecentsContribution;
impl Contribution for RecentsContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        let recents_view_id = "workbench.view.recentsView";
        registry.views.register_views(
            &super::tree_view_groups::launchpad::GROUP_ID,
            vec![TreeViewDescriptor {
                id: recents_view_id.to_string(),
                name: "Recents".to_string(),
                order: 1,
                hide_by_default: false,
                can_toggle_visibility: false,
                collapsed: false,
                model: Lazy::new(|| Box::new(RecentsViewModel {})),
            }],
        )?;

        // Menus contributions

        let recents_context = static_format!("view == '{recents_view_id}'");

        registry.menus.append_menu_item(
            Menus::ViewTitleContext.into(),
            MenuItem::Action(ActionMenuItem {
                command: CommandAction {
                    id: "someId_1".to_string(),
                    title: "Hide 'Recents'".to_string(),
                    tooltip: None,
                    description: None,
                },
                group: Some("0_self".to_string()),
                order: Some(1),
                when: recents_context,
            }),
        );

        registry.menus.append_menu_item(
            Menus::ViewTitleContext.into(),
            MenuItem::Toggled(ToggledMenuItem {
                command: CommandAction {
                    id: "someId_1".to_string(),
                    title: "Recents".to_string(),
                    tooltip: None,
                    description: None,
                },
                group: Some("1_views".to_string()),
                order: Some(1),
                when: recents_context,
                toggled: static_format!("viewState == 'mockState'"),
            }),
        );

        let recents_item_context =
            static_format!("view == '{recents_view_id}' && viewItem == 'recents.item'");

        let open_with_profile_menu_id = MenuId::new("recents.openWithProfileSubmenu");
        registry.menus.append_menu_items(vec![
            (
                open_with_profile_menu_id.clone(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_1".to_string(),
                        title: "Default".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("0_profiles".to_string()),
                    order: Some(1),
                    when: recents_item_context,
                }),
            ),
            (
                open_with_profile_menu_id.clone(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_2".to_string(),
                        title: "Custom".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("0_profiles".to_string()),
                    order: None,
                    when: recents_item_context,
                }),
            ),
        ]);

        registry.menus.append_menu_items(vec![
            (
                Menus::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_1".to_string(),
                        title: "Open".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("1_open".to_string()),
                    order: Some(1),
                    when: recents_item_context,
                }),
            ),
            (
                Menus::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_2".to_string(),
                        title: "Open in New Window".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("1_open".to_string()),
                    order: Some(2),
                    when: recents_item_context,
                }),
            ),
        ]);

        registry.menus.append_menu_items(vec![(
            Menus::ViewItemContext.into(),
            MenuItem::Submenu(SubmenuMenuItem {
                submenu_id: open_with_profile_menu_id,
                title: "Open with Profile".to_string(),
                group: Some("1_open".to_string()),
                order: Some(3),
                when: recents_item_context,
            }),
        )]);

        registry.menus.append_menu_items(vec![
            (
                Menus::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_4".to_string(),
                        title: "Preview".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("2_preview".to_string()),
                    order: Some(1),
                    when: recents_item_context,
                }),
            ),
            (
                Menus::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "someId_5".to_string(),
                        title: "Remove from Recents".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some("3_remove".to_string()),
                    order: Some(1),
                    when: recents_item_context,
                }),
            ),
        ]);

        Ok(())
    }
}
