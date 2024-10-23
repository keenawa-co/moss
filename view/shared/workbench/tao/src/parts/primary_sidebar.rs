use crate::view::{TreeView, TreeViewContainer, TreeViewContainerLocation};

use super::{AnyPart, PartId, Parts};
use anyhow::Result;
use hashbrown::HashMap;

#[derive(Debug, Serialize)]
pub struct DescribeSideBarPartOutput {
    pub views: HashMap<String, Vec<TreeView>>,
}

pub struct PrimarySideBarPart {}

impl PrimarySideBarPart {
    pub fn new() -> Self {
        Self {}
    }
}

impl AnyPart for PrimarySideBarPart {
    const ID: PartId = Parts::PRIMARY_SIDE_BAR;
    type DescribeOutput = DescribeSideBarPartOutput;

    fn describe(&self, registry: &crate::RegistryManager) -> Result<Self::DescribeOutput> {
        let mut views = HashMap::new();

        if let Some(containers) = registry
            .views
            .get_containers_by_location(&TreeViewContainerLocation::PrimaryBar)
        {
            for container in containers {
                if let Some(view_descriptors) =
                    registry.views.get_views_by_container_id(&container.id)
                {
                    views
                        .entry(container.id.to_string())
                        .or_insert_with(Vec::new)
                        .extend(view_descriptors.clone())
                }
            }
        }

        Ok(DescribeSideBarPartOutput { views })
    }
}
