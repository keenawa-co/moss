use crate::{
    view::{AnyContentProvider, TreeViewDescriptor},
    Contribution,
};
use anyhow::Result;
use once_cell::sync::Lazy;
use quote::quote;

pub struct RecentsContentProvider {}

#[derive(Debug, Serialize)]
pub struct RecentsViewTreeItem {
    pub path: String,
    pub last_modification: String,
}

pub struct RecentsViewContentProviderOutput {
    pub data: Vec<RecentsViewTreeItem>,
    pub html: String,
}

impl AnyContentProvider for RecentsContentProvider {
    type ContentOutput = Result<RecentsViewContentProviderOutput>;

    fn content(&self) -> Self::ContentOutput {
        let tokens = quote! { <p className="text-sm">"Hello, World!"</p> };

        Ok(RecentsViewContentProviderOutput {
            html: tokens.to_string(),
            data: vec![
                RecentsViewTreeItem {
                    path: "~/keenawa/moss".to_string(),
                    last_modification: "14 min ago".to_string(),
                },
                RecentsViewTreeItem {
                    path: "~/zigland/zig".to_string(),
                    last_modification: "18 hours ago".to_string(),
                },
            ],
        })
    }
}

pub struct RecentsViewModel {}

pub(crate) struct RecentsContribution;
impl Contribution for RecentsContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        registry.views.register_views(
            &super::tree_view_groups::launchpad::GROUP_ID,
            vec![TreeViewDescriptor {
                id: "workbench.view.recentsView".to_string(),
                name: "Recents".to_string(),
                order: 1,
                hide_by_default: false,
                can_toggle_visibility: false,
                collapsed: false,
                model: Lazy::new(|| Box::new(RecentsViewModel {})),
            }],
        )?;

        Ok(())
    }
}
