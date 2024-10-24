use crate::{view::TreeView, Contribution};

pub(crate) struct LinksContribution;
impl Contribution for LinksContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        registry.views.register_views(
            &super::tree_view_groups::launchpad::GROUP_ID,
            vec![TreeView {
                id: "workbench.view.links".to_string(),
                name: "Links".to_string(),
                order: 2,
                hide_by_default: false,
                can_toggle_visibility: true,
            }],
        )?;

        Ok(())
    }
}
