use crate::{
    view::{TreeViewContainer, TreeViewContainerLocation},
    Contribution,
};

pub(crate) mod launchpad {
    use super::*;

    pub const GROUP_ID: &'static str = "workbench.group.launchpad";

    pub(crate) struct LaunchpadGroupContribution;
    impl Contribution for LaunchpadGroupContribution {
        fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
            registry.views.register_container(
                TreeViewContainerLocation::PrimaryBar,
                TreeViewContainer {
                    id: GROUP_ID,
                    name: "Launchpad".to_string(),
                    order: 1,
                },
            )?;

            Ok(())
        }
    }
}
