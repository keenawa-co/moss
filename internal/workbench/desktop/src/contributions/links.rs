use crate::Contribution;
use anyhow::Result;
use desktop_models::{constants, view::TreeViewDescriptor};
use moss_str::localize;
use once_cell::sync::Lazy;
use std::sync::Arc;
use uikit_models::html::link::HtmlLink;

#[derive(Debug, Serialize)]
pub struct LinksViewContent(Vec<HtmlLink>);

pub struct LinksView;

impl LinksView {
    pub fn content(&self) -> Result<LinksViewContent> {
        Ok(LinksViewContent(vec![
            HtmlLink::new("https://example.com", "Docs"), // TODO: localize!("links.docs", "Docs")
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
            constants::view::VIEW_GROUP_ID_LAUNCHPAD,
            vec![TreeViewDescriptor {
                id: "workbench.view.linksView".to_string(),
                name: localize!("links.view.name", "Links"),
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
