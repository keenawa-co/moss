use super::Contribution;
use crate::{
    models::{
        constants,
        view::{TreeViewGroup, TreeViewGroupLocation},
    },
    state::AppState,
};
use moss_text::localize;

pub struct WorkbenchContribution;
impl Contribution for WorkbenchContribution {
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
