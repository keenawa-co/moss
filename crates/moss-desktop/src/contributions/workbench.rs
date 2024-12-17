use moss_jsonlogic::raw_rule::*;
use moss_jsonlogic_macro::rule;
use moss_text::{localize, ReadOnlyStr};
use serde_json::Value;
use tauri::{Emitter, EventTarget, Manager};

use crate::{
    command::{CommandContext, CommandDecl},
    contribution_point,
    models::{
        actions::*, appearance::theming::ThemeDescriptor, constants, view::*,
        window::LocaleDescriptor,
    },
    state::{AppState, MenuDecl, TreeViewGroupDecl},
};

// TODO:
// pub struct WorkbenchContribution;
// impl ContributionOld for WorkbenchContribution {
//     fn contribute(registry: &mut AppState) -> anyhow::Result<()> {
//         let mut views_registry_lock = registry.views.write();

//         views_registry_lock.append_view_group(
//             TreeViewGroupLocation::PrimaryBar,
// TreeViewGroup {
//     id: constants::view::VIEW_GROUP_ID_LAUNCHPAD,
//     name: localize!("launchpad.group.name", "Launchpad"),
//     order: 1,
// },
//         );

//         Ok(())
//     }
// }

contribution_point!(TEST1, {
    commands: [
        CommandDecl {
            key: "workbench.changeColorTheme",
            handler: change_color_theme,
        },
        CommandDecl {
            key: "workbench.changeLanguagePack",
            handler: change_language_pack,
        },
    ],
    tree_view_groups: [
        TreeViewGroupDecl {
            location:  TreeViewGroupLocation::PrimaryBar,
            items: vec![
                TreeViewGroup {
                    id: constants::view::VIEW_GROUP_ID_LAUNCHPAD,
                    name: localize!("launchpad.group.name", "Launchpad"),
                    order: 1,
                },
            ]
        },
    ]
});

pub fn change_color_theme(ctx: CommandContext, app_state: &AppState) -> Result<Value, String> {
    let theme_descriptor_arg = ctx.take_arg::<ThemeDescriptor>("themeDescriptor")?;

    app_state.change_color_theme(theme_descriptor_arg.clone());

    for (label, _) in ctx.app_handle.webview_windows() {
        if ctx.window.label() == &label {
            continue;
        }

        ctx.app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://color-theme-changed",
                theme_descriptor_arg.clone(),
            )
            .unwrap();
    }

    Ok(Value::Null)
}

pub fn change_language_pack(ctx: CommandContext, app_state: &AppState) -> Result<Value, String> {
    let locale_descriptor_arg = ctx.take_arg::<LocaleDescriptor>("localeDescriptor")?;

    app_state.change_language_pack(locale_descriptor_arg.clone());

    for (label, _) in ctx.app_handle.webview_windows() {
        if ctx.window.label() == &label {
            continue;
        }

        ctx.app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://language-pack-changed",
                locale_descriptor_arg.clone(),
            )
            .unwrap();
    }

    Ok(Value::Null)
}
