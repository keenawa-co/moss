use crate::{
    models::{constants, view::TreeViewDescriptor},
    state::AppState,
};
use anyhow::Result;
use moss_html::link::HtmlLink;
use moss_text::localize;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Arc;

use super::ContributionOld;

#[derive(Debug, Serialize)]
pub struct LinksViewContent(Vec<HtmlLink>);

pub struct LinksView;

impl LinksView {
    #[rustfmt::skip]
    pub fn content(&self) -> Result<LinksViewContent> {
        Ok(LinksViewContent(
            vec![
            HtmlLink::new("https://example.com", Some(localize!("links.docs", "Docs"))),
            HtmlLink::new("https://example.com", Some(localize!("links.releases", "Releases"))),
            HtmlLink::new("https://example.com", Some(localize!("links.gitHub", "GitHub"))),
            HtmlLink::new("https://example.com", Some(localize!("links.support", "Support"))),
        ]))
    }
}

pub(crate) struct LinksContribution;
impl ContributionOld for LinksContribution {
    fn contribute(registry: &mut AppState) -> anyhow::Result<()> {
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
