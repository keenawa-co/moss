use crate::{
    view::{TreeView, TreeViewContainer, TreeViewContainerLocation},
    Contribution,
};

pub(crate) struct LaunchpadContribution;
impl Contribution for LaunchpadContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        let container_id = registry.views.register_container(
            TreeViewContainerLocation::PrimaryBar,
            TreeViewContainer {
                id: "launchpad",
                name: "Launchpad".to_string(),
                order: 1,
            },
        )?;

        registry.views.register_views(
            &container_id,
            vec![
                TreeView {
                    id: "launchpad.recents".to_string(),
                    name: "Recents".to_string(),
                    order: 1,
                    hide_by_default: false,
                    can_toggle_visibility: false,
                },
                TreeView {
                    id: "launchpad.links".to_string(),
                    name: "Links".to_string(),
                    order: 2,
                    hide_by_default: false,
                    can_toggle_visibility: true,
                },
            ],
        )?;

        Ok(())
    }
}
