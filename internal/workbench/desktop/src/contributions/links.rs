use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::Arc;
use workbench_models::primitive::navigation::HtmlLink;

use crate::{
    view::{BuiltInViewGroups, TreeViewDescriptor},
    Contribution,
};

#[derive(Debug, Serialize)]
pub struct LinksViewContent(Vec<HtmlLink>);

pub struct LinksView;

impl LinksView {
    pub fn content(&self) -> Result<LinksViewContent> {
        Ok(LinksViewContent(vec![
            HtmlLink::new("https://example.com", "Docs"),
            HtmlLink::new("https://example.com", "Releases"),
            HtmlLink::new("https://example.com", "GitHub"),
            HtmlLink::new("https://example.com", "Support"),
        ]))
    }
}

pub(crate) struct LinksContribution;
impl Contribution for LinksContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        let mut views_registry_lock = registry.views.write();
        views_registry_lock.register_views(
            BuiltInViewGroups::Launchpad.into(),
            vec![TreeViewDescriptor {
                id: "workbench.view.linksView".to_string(),
                name: "Links".to_string(),
                order: 2,
                hide_by_default: false,
                can_toggle_visibility: true,
                collapsed: false,
                model: Lazy::new(|| Arc::new(LinksView {})),
            }],
        );

        drop(views_registry_lock);

        Ok(())
    }
}
