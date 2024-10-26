use moss_uikit_models::layout::{Alignment, Orientation};

use crate::{
    view::{TreeViewGroup, TreeViewGroupLocation},
    RegistryManager,
};

use super::{AnyPart, PartId, Parts};

#[derive(Debug, Serialize)]
pub struct DescribeActivityBarPartOutput {
    pub align: Alignment,
    pub orientation: Orientation,
    pub containers: Option<Vec<TreeViewGroup>>,
}

pub struct PrimaryActivityBarPart {
    align: Alignment,
    orientation: Orientation,
}

impl PrimaryActivityBarPart {
    pub fn new() -> Self {
        Self {
            align: Alignment::Start,
            orientation: Orientation::Horizontal,
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
                .get_groups_by_location(&TreeViewGroupLocation::PrimaryBar)
                .cloned(),
        };

        Ok(result)
    }
}
