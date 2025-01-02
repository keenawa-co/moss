use dashmap::{DashMap, DashSet};
use moss_text::ReadOnlyStr;
use std::sync::Arc;

use crate::command::CommandHandler;
use crate::contribution::Contribution;
use crate::models::application::ThemeDescriptor;
use crate::models::{actions::MenuItem, view::*};

#[derive(Default)]
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

    //  crate::contribution::CONTRIBUTIONS
    pub fn init<I>(self, contributions: I) -> Self
    where
        I: IntoIterator<Item = &'static Contribution>,
    {
        for contrib in contributions {
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
