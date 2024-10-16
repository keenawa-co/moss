use anyhow::Result;
use moss_hecs::MissingComponent;
use moss_uikit::component::{layout::Order, primitive::Tooltip};

use crate::Workbench;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeActivityOutput {
    pub tooltip: Tooltip,
    pub order: usize,
}

impl Workbench {
    // OPTIMIZE: consider to use a SmallVec type, as we don't expect tons of such elements
    pub fn command_get_all_activities(&self) -> Result<Vec<DescribeActivityOutput>> {
        // let mut result = Vec::new();
        // for entity in &self.known_activities {
        //     let entity_ref = self.frame.entity(*entity)?;

        //     let tooltip = entity_ref
        //         .get::<&Tooltip>()
        //         .ok_or_else(|| MissingComponent::new::<Tooltip>())?;

        //     let order = entity_ref
        //         .get::<&Order>()
        //         .ok_or_else(|| MissingComponent::new::<Order>())?;

        //     result.push(DescribeActivityOutput {
        //         tooltip: (*tooltip).clone(),
        //         order: order.value.clone(),
        //     });
        // }

        // Ok(result)

        todo!()
    }

    // pub fn describe_menu_bar_part(&self) ->
}
