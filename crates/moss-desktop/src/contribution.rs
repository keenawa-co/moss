use linkme::distributed_slice;
use moss_text::ReadOnlyStr;
use parking_lot::Mutex;
use std::fmt::Debug;

use crate::command::CommandDecl;
use crate::models::{actions::MenuItem, view::*};

#[distributed_slice]
pub(crate) static CONTRIBUTIONS: [once_cell::sync::Lazy<Contribution>] = [..];

#[macro_export]
macro_rules! contribution_point {
    ($name:ident, {
        $( $field:ident : [ $( $item:expr ),* $(,)? ] ),* $(,)?
    }) => {
        paste::item! {
            #[linkme::distributed_slice($crate::contribution::CONTRIBUTIONS)]
            static [<__CONTRIBUTION_ $name __>]: once_cell::sync::Lazy<$crate::contribution::Contribution> = once_cell::sync::Lazy::new(|| {
                #[allow(unused_mut)] let mut commands = Vec::new();
                #[allow(unused_mut)] let mut menus = Vec::new();
                #[allow(unused_mut)] let mut tree_view_groups = Vec::new();
                #[allow(unused_mut)] let mut tree_views = Vec::new();

                $(
                    contribution_point!(@assign_field $field, [ $( $item ),* ], commands, menus, tree_view_groups, tree_views);
                )*

                $crate::contribution::Contribution {
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
    #[allow(dead_code)]
    pub source: &'static str,
    pub commands: Mutex<Vec<CommandDecl>>,
    pub menus: Mutex<Vec<MenuDecl>>,
    pub tree_view_groups: Mutex<Vec<TreeViewGroupDecl>>,
    pub tree_views: Mutex<Vec<TreeViewDecl>>,
}
