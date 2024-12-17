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

#[distributed_slice]
pub static CONTRIBUTIONS: [once_cell::sync::Lazy<Contribution>] = [..];

#[macro_export]
macro_rules! contribution_point {
    ($name:ident, {
        $( $field:ident : [ $( $item:expr ),* $(,)? ] ),* $(,)?
    }) => {
        paste::item! {
            #[linkme::distributed_slice($crate::state::CONTRIBUTIONS)]
            static [<__CONTRIBUTION_ $name __>]: once_cell::sync::Lazy<$crate::state::Contribution> = once_cell::sync::Lazy::new(|| {
                let mut commands = Vec::new();
                let mut menus = Vec::new();
                let mut tree_view_groups = Vec::new();
                let mut tree_views = Vec::new();

                $(
                    contribution_point!(@assign_field $field, [ $( $item ),* ], commands, menus, tree_view_groups, tree_views);
                )*

                $crate::state::Contribution {
                    source: concat!(module_path!(), "::", file!(), ":", line!(), ":", column!()),
                    commands: parking_lot::Mutex::new(commands),
                    menus: parking_lot::Mutex::new(menus),
                    tree_view_groups: parking_lot::Mutex::new(tree_view_groups),
                    tree_views: parking_lot::Mutex::new(tree_views),
                }
            });
        }
    };

    // Helper patterns for field assignment
    (@assign_field commands, [ $( $item:expr ),* ], $commands:ident, $menus:ident, $tree_view_groups:ident, $tree_views:ident) => {
        $commands.extend(vec![ $( $item ),* ]);
    };
    (@assign_field menus, [ $( $item:expr ),* ], $commands:ident, $menus:ident, $tree_view_groups:ident, $tree_views:ident) => {
        $menus.extend(vec![ $( $item ),* ]);
    };
    (@assign_field tree_view_groups, [ $( $item:expr ),* ], $commands:ident, $menus:ident, $tree_view_groups:ident, $tree_views:ident) => {
        $tree_view_groups.extend(vec![ $( $item ),* ]);
    };
    (@assign_field tree_views, [ $( $item:expr ),* ], $commands:ident, $menus:ident, $tree_view_groups:ident, $tree_views:ident) => {
        $tree_views.extend(vec![ $( $item ),* ]);
    };
    // Pattern for unknown fields — triggers a compilation error
    (@assign_field $unknown:ident, [ $( $item:expr ),* ], $commands:ident, $menus:ident, $tree_view_groups:ident, $tree_views:ident) => {
        compile_error!(concat!("Unknown field in contribution_point!: ", stringify!($unknown)));
    };
}

// #[derive(Debug)]
// pub struct ViewsRegistryOld {
//     groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
//     views: HashMap<GroupId, Vec<TreeViewDescriptor>>,
// }

// impl ViewsRegistryOld {
//     pub fn new() -> Self {
//         ViewsRegistryOld {
//             groups: HashMap::new(),
//             views: HashMap::new(),
//         }
//     }

//     pub(crate) fn append_view_group(
//         &mut self,
//         location: TreeViewGroupLocation,
//         group: TreeViewGroup,
//     ) {
//         self.groups
//             .entry(location)
//             .or_insert_with(Vec::new)
//             .push(group);
//     }

//     pub(crate) fn register_views(
//         &mut self,
//         id: ReadOnlyStr,
//         batch: impl IntoIterator<Item = TreeViewDescriptor>,
//     ) {
//         self.views.entry(id).or_insert_with(Vec::new).extend(batch);
//     }
// }

// pub struct MenuRegistryOld {
//     menus: HashMap<ReadOnlyStr, Vec<MenuItem>>,
// }

// impl MenuRegistryOld {
//     pub fn new() -> Self {
//         Self {
//             menus: HashMap::new(),
//         }
//     }

//     pub fn append_menu_item(&mut self, menu_id: ReadOnlyStr, item: MenuItem) {
//         self.menus
//             .entry(menu_id.into())
//             .or_insert_with(Vec::new)
//             .push(item);
//     }

//     pub fn append_menu_items<I>(&mut self, items: I)
//     where
//         I: IntoIterator<Item = (ReadOnlyStr, MenuItem)>,
//     {
//         for (menu_id, item) in items {
//             self.append_menu_item(menu_id, item);
//         }
//     }

//     pub fn get_menu_items_by_namespace(&self, namespace: &ReadOnlyStr) -> Option<&Vec<MenuItem>> {
//         self.menus.get(namespace)
//     }
// }

#[derive(Debug)]
pub struct MenuDecl {
    pub namespace: ReadOnlyStr,
    pub items: Vec<MenuItem>,
}

#[derive(Debug)]
pub struct TreeViewGroupDecl {
    pub location: TreeViewGroupLocation,
    pub items: Vec<TreeViewGroup>,
}

#[derive(Debug)]
pub struct TreeViewDecl {
    pub group_id: &'static str,
    pub items: Vec<TreeViewDescriptor>,
}

#[derive(Debug)]
pub struct Contribution {
    pub source: &'static str,
    pub commands: Mutex<Vec<CommandDecl>>,
    pub menus: Mutex<Vec<MenuDecl>>,
    pub tree_view_groups: Mutex<Vec<TreeViewGroupDecl>>,
    pub tree_views: Mutex<Vec<TreeViewDecl>>,
}

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
        let commands = DashMap::new();
        let menus: DashMap<ReadOnlyStr, Vec<MenuItem>> = DashMap::new();
        let tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>> = DashMap::new();
        let tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>> = DashMap::new();

        dbg!(&CONTRIBUTIONS.len());
        for contrib in CONTRIBUTIONS {
            dbg!(&contrib.source);

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

        dbg!(&menus);

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
            commands,
            menus,
            tree_view_groups,
            tree_views,
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
