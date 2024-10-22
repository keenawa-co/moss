use crate::{
    contribution::ViewContainerLocation,
    views::{TreeViewContainer, TreeViewDescriptor},
};

use super::{AnyPart, PartId, Parts};
use anyhow::Result;
use hashbrown::HashMap;

#[derive(Debug, Serialize)]
pub struct DescribeSideBarPartOutput {
    pub views: HashMap<String, Vec<TreeViewDescriptor>>,
}

pub struct SideBarPart {
    view_container_group_key: ViewContainerLocation,
}

impl SideBarPart {
    pub fn new(view_container_group_key: ViewContainerLocation) -> Self {
        Self {
            view_container_group_key,
        }
    }

    fn get_containers(&self, registry: &crate::RegistryManager) -> Option<Vec<TreeViewContainer>> {
        registry
            .views
            .get_containers_by_group_id(&self.view_container_group_key.as_group_key())
    }
}

impl AnyPart for SideBarPart {
    const ID: PartId = Parts::PRIMARY_SIDE_BAR;
    type DescribeOutput = DescribeSideBarPartOutput;

    fn describe(&self, registry: &crate::RegistryManager) -> Result<Self::DescribeOutput> {
        let mut views = HashMap::new();

        if let Some(containers) = self.get_containers(registry) {
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
