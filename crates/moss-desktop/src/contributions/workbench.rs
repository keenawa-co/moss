use moss_desktop_macro::register_contribution;

use super::ContributionOld;
use crate::{
    command::{self, CommandDecl},
    contribution_point,
    models::{
        constants,
        view::{TreeViewGroup, TreeViewGroupLocation},
    },
    state::{change_color_theme, change_language_pack, AppState, Contribution},
};
use moss_text::localize;

pub struct WorkbenchContribution;
impl ContributionOld for WorkbenchContribution {
    fn contribute(registry: &mut AppState) -> anyhow::Result<()> {
        let mut views_registry_lock = registry.views.write();

        views_registry_lock.append_view_group(
            TreeViewGroupLocation::PrimaryBar,
            TreeViewGroup {
                id: constants::view::VIEW_GROUP_ID_LAUNCHPAD,
                name: localize!("launchpad.group.name", "Launchpad"),
                order: 1,
            },
        );

        Ok(())
    }
}

// #[contribution_point]
// fn init() -> Result<Contribution, ContributionError> {
//     ContributionBuilder::new("workbench_contribution")
//         .add_commands(vec![
//             Command {
//                 key: "workbench.changeColorTheme",
//                 handler: change_color_theme,
//             },
//             Command {
//                 key: "workbench.changeLanguagePack",
//                 handler: change_language_pack,
//             },
//         ])?
//         .build()
// }

// register_contribution! {
//     fn example_contribution_commands() -> Contribution {
//         let mut builder = ContributionBuilder::new("example_contribution_commands");

//         // Добавляем команды или меню если нужно
//         // В данном случае просто вызываем build()

//         builder.build()
//     }
// }

// contribution_point!("", {
//     commands: [
//         Command {
//             key: "workbench.changeColorTheme",
//             handler: change_color_theme,
//         },
//         Command {
//             key: "workbench.changeLanguagePack",
//             handler: change_language_pack,
//         },
//     ]
// });

contribution_point!("my_contribution", {
    commands: [
        CommandDecl {
            key: "workbench.changeColorTheme",
            handler: change_color_theme,
        },
        CommandDecl {
            key: "workbench.changeLanguagePack",
            handler: change_language_pack,
        },
    ]
});
