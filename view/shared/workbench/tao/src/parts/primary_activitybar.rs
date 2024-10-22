use moss_uikit::component::layout::{Alignment, Orientation};

use crate::{contribution::ViewContainerLocation, views::TreeViewContainer, RegistryManager};

use super::{AnyPart, PartId, Parts};

#[derive(Debug, Serialize)]
pub struct DescribeActivityBarPartOutput {
    pub align: Alignment,
    pub orientation: Orientation,
    pub containers: Option<Vec<TreeViewContainer>>,
}

pub struct PrimaryActivityBarPart {
    align: Alignment,
    orientation: Orientation,
    view_container_group_key: ViewContainerLocation,
}

impl PrimaryActivityBarPart {
    pub fn new(view_container_group_key: ViewContainerLocation) -> Self {
        Self {
            align: Alignment::Start,
            orientation: Orientation::Horizontal,
            view_container_group_key,
        }
    }
}

impl AnyPart for PrimaryActivityBarPart {
    const ID: PartId = Parts::PRIMARY_ACTIVITY_BAR;
    type DescribeOutput = DescribeActivityBarPartOutput;

    fn describe(&self, registry: &RegistryManager) -> anyhow::Result<Self::DescribeOutput> {
        let result = DescribeActivityBarPartOutput {
            align: self.align.clone(),
            orientation: self.orientation.clone(),
            containers: registry
                .views
                .get_containers_by_group_id(&self.view_container_group_key.as_group_key()),
        };

        Ok(result)
    }
}
