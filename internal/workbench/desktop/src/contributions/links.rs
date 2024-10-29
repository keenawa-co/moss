use once_cell::sync::Lazy;

use crate::{
    view::{AnyContentProvider, TreeViewDescriptor},
    Contribution,
};

pub struct LinksViewModel;

pub(crate) struct LinksContribution;
impl Contribution for LinksContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        let mut views_registry_lock = registry.views.write();
        views_registry_lock.register_views(
            &super::tree_view_groups::launchpad::GROUP_ID,
            vec![TreeViewDescriptor {
                id: "workbench.view.linksView".to_string(),
                name: "Links".to_string(),
                order: 2,
                hide_by_default: false,
                can_toggle_visibility: true,
                collapsed: false,
                model: Lazy::new(|| Box::new(LinksViewModel {})),
            }],
        )?;

        drop(views_registry_lock);

        Ok(())
    }
}
