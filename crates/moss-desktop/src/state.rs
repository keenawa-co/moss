use anyhow::Result;
use dashmap::DashMap;
use hashbrown::HashMap;
use linkme::distributed_slice;
use moss_text::ReadOnlyStr;
use parking_lot::{Mutex, RwLock};
use serde_json::Value;
use std::{fmt::Debug, sync::atomic::AtomicUsize, sync::Arc};
use tauri::{Emitter, EventTarget, Manager};

use crate::command::{CommandContext, CommandDecl, CommandHandler};
use crate::contribution_collector::{ContributionCollection, ContributionCollector};
use crate::models::{
    actions::MenuItem, appearance::theming::ThemeDescriptor, view::*, window::LocaleDescriptor,
};

// #[derive(Debug)]
// pub struct MenuDecl {
//     pub namespace: ReadOnlyStr,
//     pub items: Vec<MenuItem>,
// }

// #[derive(Debug)]
// pub struct TreeViewGroupDecl {
//     pub location: TreeViewGroupLocation,
//     pub items: Vec<TreeViewGroup>,
// }

// #[derive(Debug)]
// pub struct TreeViewDecl {
//     pub group_id: &'static str,
//     pub items: Vec<TreeViewDescriptor>,
// }

// #[derive(Debug)]
// pub struct Contribution {
//     pub source: &'static str,
//     pub commands: Mutex<Vec<CommandDecl>>,
//     pub menus: Mutex<Vec<MenuDecl>>,
//     pub tree_view_groups: Mutex<Vec<TreeViewGroupDecl>>,
//     pub tree_views: Mutex<Vec<TreeViewDecl>>,
// }

pub struct Preferences {
    pub theme: RwLock<ThemeDescriptor>,
    pub locale: RwLock<LocaleDescriptor>,
}

pub struct AppState {
    next_window_id: AtomicUsize,
    pub preferences: Preferences,
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl AppState {
    pub fn new() -> Self {
        // FIXME: This should be abstracted in the future.
        let contribution_collector = ContributionCollector::new();
        let contribution_collection = contribution_collector.collect();

        let state = Self {
            next_window_id: AtomicUsize::new(0),
            preferences: Preferences {
                theme: RwLock::new(ThemeDescriptor {
                    id: "theme-light".to_string(),
                    name: "Theme Light".to_string(),
                    source: "moss-light.css".to_string(),
                }),
                locale: RwLock::new(LocaleDescriptor {
                    code: "en".to_string(),
                    name: "English".to_string(),
                    direction: Some("ltr".to_string()),
                }),
            },
            commands: contribution_collection.commands,
            menus: contribution_collection.menus,
            tree_view_groups: contribution_collection.tree_view_groups,
            tree_views: contribution_collection.tree_views,
        };

        state
    }

    pub fn inc_next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
    }

    pub fn change_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = locale_descriptor;
    }

    pub fn change_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = theme_descriptor;
    }
}
