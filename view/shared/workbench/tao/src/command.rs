use anyhow::Result;
use crater::MissingComponent;
use ts_rs::TS;

use crate::{
    component::{Order, Tooltip},
    Workbench,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "dummy.ts")]
pub struct DescribeActivityOutput {
    pub tooltip: String,
    pub order: usize,
}

impl Workbench {
    // OPTIMIZE: consider to use a SmallVec type, as we don't expect tons of such elements
    pub fn command_get_all_activities(&self) -> Result<Vec<DescribeActivityOutput>> {
        let mut result = Vec::new();
        for entity in &self.known_activities {
            let entity_ref = self.frame.entity(*entity)?;

            let r = entity_ref
                .get::<&Tooltip>()
                .ok_or_else(|| MissingComponent::new::<Tooltip>())?;

            let r2 = entity_ref
                .get::<&Order>()
                .ok_or_else(|| MissingComponent::new::<Order>())?;

            result.push(DescribeActivityOutput {
                tooltip: r.0.to_string(),
                order: r2.0.clone(),
            });
        }

        Ok(result)
    }
}
