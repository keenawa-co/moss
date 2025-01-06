use dashmap::{DashMap, DashSet};
use moss_text::ReadOnlyStr;
use std::sync::Arc;

use crate::command::CommandHandler;
use crate::models::application::ThemeDescriptor;
use crate::models::{actions::MenuItem, view::*};

pub struct ContributionRegistry {
    pub themes: Arc<DashSet<ThemeDescriptor>>,
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl ContributionRegistry {
    pub fn new() -> Self {
        Self {
            themes: Arc::new(DashSet::new()),
            commands: DashMap::new(),
            menus: DashMap::new(),
            tree_view_groups: DashMap::new(),
            tree_views: DashMap::new(),
        }
    }

    pub fn init(self) -> Self {
        for contrib in crate::contribution::CONTRIBUTIONS {
            for decl in std::mem::take(&mut *contrib.commands.lock()) {
                self.commands
                    .insert(decl.name, Arc::new(decl.callback) as CommandHandler);
            }

            for decl in std::mem::take(&mut *contrib.menus.lock()) {
                let mut items = decl.items;

                self.menus
                    .entry(decl.namespace)
                    .and_modify(|existing_namespace| {
                        existing_namespace.append(&mut items);
                    })
                    .or_insert(items);
            }

            for decl in std::mem::take(&mut *contrib.tree_view_groups.lock()) {
                let mut items = decl.items;

                self.tree_view_groups
                    .entry(decl.location)
                    .and_modify(|existing_group| {
                        existing_group.append(&mut items);
                    })
                    .or_insert(items);
            }

            for decl in std::mem::take(&mut *contrib.tree_views.lock()) {
                let mut items = decl.items;

                self.tree_views
                    .entry(decl.group_id.into())
                    .and_modify(|existing_group| {
                        existing_group.append(&mut items);
                    })
                    .or_insert(items);
            }
        }

        self
    }
}

pub struct ContributionCollection {
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

pub(crate) struct ContributionCollector {}

impl ContributionCollector {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn collect(&self) -> ContributionCollection {
        let commands = DashMap::new();
        let menus: DashMap<ReadOnlyStr, Vec<MenuItem>> = DashMap::new();
        let tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>> = DashMap::new();
        let tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>> = DashMap::new();

        for contrib in crate::contribution::CONTRIBUTIONS {
            for decl in std::mem::take(&mut *contrib.commands.lock()) {
                commands.insert(decl.name, Arc::new(decl.callback) as CommandHandler);
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
