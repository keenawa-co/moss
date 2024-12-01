use anyhow::Result;
use desktop_models::{
    actions::{
        ActionMenuItem, CommandAction, MenuGroup, MenuItem, MenuItemVisibility, SubmenuMenuItem,
        SubmenuRef,
    },
    constants,
    view::TreeViewDescriptor,
};
use moss_jsonlogic::raw_rule::*;
use moss_jsonlogic_macro::rule;
use moss_str::{localize, ReadOnlyStr};
use once_cell::sync::Lazy;
use quote::quote;
use static_str_ops::static_format;
use std::{rc::Rc, sync::Arc};

use crate::Contribution;

#[derive(Debug, Serialize)]
pub struct RecentsViewTreeItem {
    pub path: String,
    pub last_modification: String,
}

#[derive(Debug, Serialize)]
pub struct RecentsViewContent {
    pub data: Vec<RecentsViewTreeItem>,
    pub html: String,
}

#[derive(Debug, Serialize)]
pub struct RecentsViewModel {}

impl RecentsViewModel {
    pub fn content(&self) -> Result<RecentsViewContent> {
        let tokens = quote! { <p className="text-sm">"Hello, World!"</p> };

        Ok(RecentsViewContent {
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
            constants::view::VIEW_GROUP_ID_LAUNCHPAD,
            vec![TreeViewDescriptor {
                id: recents_view_id.to_string(),
                name: localize!("recents.view.name", "Recents"),
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

        let recents_rule = rule!(view == val!(recents_view_id));

        #[rustfmt::skip]
        let (
            view_title_context_menu_group_this,
            view_title_context_menu_group_views,
            view_title_context_menu_group_inline,
        ) = {
            let this = Rc::new(MenuGroup::new_ordered(0, constants::menu::MENU_GROUP_ID_THIS));
            let views = Rc::new(MenuGroup::new_ordered(1, constants::menu::MENU_GROUP_ID_VIEWS));
            let inline = Rc::new(MenuGroup::new_ordered(2, constants::menu::MENU_GROUP_ID_INLINE));

            (this, views, inline)
        };

        menus_registry_lock.append_menu_items(vec![
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.hideRecentsView".into(),
                        title: localize!("recents.hideRecentsView", "Hide 'Recents'"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_title_context_menu_group_this)),
                    order: Some(1),
                    when: Some(recents_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "workbench.view.recents".into(),
                        title: localize!("recents.title", "Recents"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_title_context_menu_group_views)),
                    order: Some(1),
                    when: Some(recents_rule),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        // View Item

        let recents_item_rule = rule!(view == val!(recents_view_id) && viewItem == "recents.item");

        menus_registry_lock.append_menu_items(vec![
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.remove".into(),
                        title: localize!("recents.remove", "Remove"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_title_context_menu_group_inline)),
                    order: Some(1),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.preview".into(),
                        title: localize!("recents.preview", "Preview"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_title_context_menu_group_inline)),
                    order: Some(2),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        // View Item Context

        let open_with_profile_menu_id = ReadOnlyStr::from("recents.openWithProfileSubmenu");
        menus_registry_lock.append_menu_items(vec![
            (
                SubmenuRef::from(&open_with_profile_menu_id),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "profile.default".into(),
                        title: localize!("profile.default", "Default"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: None,
                    order: Some(1),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                SubmenuRef::from(&open_with_profile_menu_id),
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "profile.custom".into(),
                        title: localize!("profile.custom", "Custom"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: None,
                    order: None,
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        #[rustfmt::skip]
        let (
            view_item_context_menu_group_navigation,
            view_item_context_menu_group_preview,
            view_item_context_menu_group_remove,
        ) = {
            let navigation = Rc::new(MenuGroup::new_ordered(0, constants::menu::MENU_GROUP_ID_NAVIGATION));
            let preview = Rc::new(MenuGroup::new_ordered(1, constants::menu::MENU_GROUP_ID_PREVIEW));
            let remove = Rc::new(MenuGroup::new_ordered(2, constants::menu::MENU_GROUP_ID_REMOVE));

            (navigation, preview, remove)
        };

        menus_registry_lock.append_menu_items(vec![
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.Open".into(),
                        title: localize!("recents.open", "Open"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_item_context_menu_group_navigation)),
                    order: Some(1),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.openInNewWindow".into(),
                        title: localize!("recents.openInNewWindow", "Open in New Window"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_item_context_menu_group_navigation)),
                    order: Some(2),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        menus_registry_lock.append_menu_items(vec![(
            constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT,
            MenuItem::Submenu(SubmenuMenuItem {
                submenu_id: open_with_profile_menu_id,
                default_action_id: None,
                title: Some(localize!("recents.openWithProfile", "Open with Profile")),
                group: Some(Rc::clone(&view_item_context_menu_group_navigation)),
                order: Some(3),
                when: Some(recents_item_rule.clone()),
                visibility: MenuItemVisibility::Classic,
            }),
        )]);

        menus_registry_lock.append_menu_items(vec![
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.preview".into(),
                        title: localize!("recents.preview", "Preview"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_item_context_menu_group_preview)),
                    order: Some(1),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
            (
                constants::menu::MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT,
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "recents.removeFromRecents".into(),
                        title: localize!("recents.removeFromRecents", "Remove from Recents"),
                        tooltip: None,
                        description: None,
                        icon: None,
                        toggled: None,
                    },
                    group: Some(Rc::clone(&view_item_context_menu_group_remove)),
                    order: Some(1),
                    when: Some(recents_item_rule.clone()),
                    visibility: MenuItemVisibility::Classic,
                }),
            ),
        ]);

        drop(menus_registry_lock);

        Ok(())
    }
}
