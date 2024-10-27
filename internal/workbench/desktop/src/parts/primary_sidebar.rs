use crate::view::{TreeViewGroupLocation, TreeViewOutput};

use super::{AnyPart, PartId, Parts};
use anyhow::Result;
use hashbrown::HashMap;

#[derive(Debug, Serialize)]
pub struct DescribeSideBarPartOutput {
    pub views: HashMap<String, Vec<TreeViewOutput>>,
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
            .get_groups_by_location(&TreeViewGroupLocation::PrimaryBar)
        {
            for container in containers {
                if let Some(view_descriptors) = registry
                    .views
                    .get_view_descriptors_by_group_id(&container.id)
                {
                    views
                        .entry(container.id.to_string())
                        .or_insert_with(Vec::new)
                        .extend(view_descriptors.iter().map(|descriptor| descriptor.into()))
                }
            }
        }

        Ok(DescribeSideBarPartOutput { views })
    }
}
