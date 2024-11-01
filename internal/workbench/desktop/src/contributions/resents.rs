use anyhow::Result;
use once_cell::sync::Lazy;
use quote::quote;
use static_str_ops::static_format;
use std::sync::Arc;

use crate::{
    menu::{
        ActionMenuItem, BuiltInMenuGroups, BuiltInMenuNamespaces, CommandAction, MenuGroup,
        MenuItem, SubmenuMenuItem,
    },
    util::ReadOnlyStr,
    view::{BuiltInGroups, TreeViewDescriptor},
    Contribution,
};

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

#[derive(Debug, Serialize)]
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
        let mut views_registry_lock = registry.views.write();

        let recents_view_id = "workbench.view.recentsView";
        views_registry_lock.register_views(
            BuiltInGroups::Launchpad.into(),
            vec![TreeViewDescriptor {
                id: recents_view_id.to_string(),
                name: "Recents".to_string(),
                order: 1,
                hide_by_default: false,
                can_toggle_visibility: false,
                collapsed: false,
                model: Lazy::new(|| Arc::new(RecentsViewModel {})),
            }],
        );

        drop(views_registry_lock);

        /* ---------- Menus contributions ---------- */

        let mut menus_registry_lock = registry.menus.write();

        // View Title Context

        let recents_context = static_format!("view == '{recents_view_id}'");

        menus_registry_lock.append_menu_items(vec![
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.hideRecentsView".into(),
                        title: "Hide 'Recents'".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(0, BuiltInMenuGroups::This)),
                    order: Some(1),
                    when: Some(recents_context.into()),
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.view.recents".into(),
                        title: "Recents".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(1, BuiltInMenuGroups::Views)),
                    order: Some(1),
                    when: Some(recents_context.into()),
                    toggled: Some("viewState == 'mockState'".into()),
                }),
            ),
        ]);

        let recents_item_context =
            static_format!("view == '{recents_view_id}' && viewItem == 'recents.item'");

        // View Item

        menus_registry_lock.append_menu_items(vec![
            (
                BuiltInMenuNamespaces::ViewItem.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.remove".into(),
                        title: "Remove".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_unordered(BuiltInMenuGroups::Inline)),
                    order: Some(1),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewItem.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.preview".into(),
                        title: "Preview".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_unordered(BuiltInMenuGroups::Inline)),
                    order: Some(2),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
        ]);

        // View Item Context

        let open_with_profile_menu_id = ReadOnlyStr::new("recents.openWithProfileSubmenu");
        menus_registry_lock.append_menu_items(vec![
            (
                open_with_profile_menu_id.clone(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "profile.default".into(),
                        title: "Default".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: None,
                    order: Some(1),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
            (
                open_with_profile_menu_id.clone(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "profile.custom".into(),
                        title: "Custom".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: None,
                    order: None,
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
        ]);

        menus_registry_lock.append_menu_items(vec![
            (
                BuiltInMenuNamespaces::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.Open".into(),
                        title: "Open".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(0, BuiltInMenuGroups::Navigation)),
                    order: Some(1),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.openInNewWindow".into(),
                        title: "Open in New Window".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(0, BuiltInMenuGroups::Navigation)),
                    order: Some(2),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
        ]);

        menus_registry_lock.append_menu_items(vec![(
            BuiltInMenuNamespaces::ViewItemContext.into(),
            MenuItem::Submenu(SubmenuMenuItem {
                submenu_id: open_with_profile_menu_id,
                title: "Open with Profile".to_string(),
                group: Some(MenuGroup::new_ordered(0, BuiltInMenuGroups::Navigation)),
                order: Some(3),
                when: Some(recents_item_context.into()),
            }),
        )]);

        menus_registry_lock.append_menu_items(vec![
            (
                BuiltInMenuNamespaces::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.preview".into(),
                        title: "Preview".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(1, BuiltInMenuGroups::Preview)),
                    order: Some(1),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewItemContext.into(),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.removeFromRecents".into(),
                        title: "Remove from Recents".to_string(),
                        tooltip: None,
                        description: None,
                    },
                    group: Some(MenuGroup::new_ordered(2, BuiltInMenuGroups::Remove)),
                    order: Some(1),
                    when: Some(recents_item_context.into()),
                    toggled: None,
                }),
            ),
        ]);

        drop(menus_registry_lock);

        Ok(())
    }
}
