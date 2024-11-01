use crate::{
    view::{TreeViewGroup, TreeViewGroupLocation},
    Contribution,
};

pub(crate) mod launchpad {
    use super::*;

    pub const GROUP_ID: &'static str = "workbench.group.launchpad";

    pub(crate) struct LaunchpadGroupContribution;
    impl Contribution for LaunchpadGroupContribution {
        fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
            registry.views.register_group(
                TreeViewGroupLocation::PrimaryBar,
                TreeViewGroup {
                    id: GROUP_ID,
                    name: "Launchpad".to_string(),
                    order: 1,
                },
            )?;

            Ok(())
        }
    }
}