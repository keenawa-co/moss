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
use crate::models::{
    actions::MenuItem, appearance::theming::ThemeDescriptor, view::*, window::LocaleDescriptor,
};

pub struct ContributionCollection {
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

pub(crate) struct ContributionCollector {}

impl ContributionCollector {
    pub(crate) fn new() -> Self {
        return Self {};
    }

    pub(crate) fn collect(&self) -> ContributionCollection {
        let commands = DashMap::new();
        let menus: DashMap<ReadOnlyStr, Vec<MenuItem>> = DashMap::new();
        let tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>> = DashMap::new();
        let tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>> = DashMap::new();

        for contrib in crate::contribution::CONTRIBUTIONS {
            for decl in std::mem::take(&mut *contrib.commands.lock()) {
                commands.insert(
                    ReadOnlyStr::from(decl.key),
                    Arc::new(decl.handler) as CommandHandler,
                );
            }

            for decl in std::mem::take(&mut *contrib.menus.lock()) {
                let mut items = decl.items;

                menus
                    .entry(decl.namespace)
                    .and_modify(|existing_namespace| {
                        existing_namespace.append(&mut items);
                    })
                    .or_insert(items);
            }

            for decl in std::mem::take(&mut *contrib.tree_view_groups.lock()) {
                let mut items = decl.items;

                tree_view_groups
                    .entry(decl.location)
                    .and_modify(|existing_group| {
                        existing_group.append(&mut items);
                    })
                    .or_insert(items);
            }

            for decl in std::mem::take(&mut *contrib.tree_views.lock()) {
                let mut items = decl.items;

                tree_views
                    .entry(decl.group_id.into())
                    .and_modify(|existing_group| {
                        existing_group.append(&mut items);
                    })
                    .or_insert(items);
            }
        }

        ContributionCollection {
            commands,
            menus,
            tree_view_groups,
            tree_views,
        }
    }
}
