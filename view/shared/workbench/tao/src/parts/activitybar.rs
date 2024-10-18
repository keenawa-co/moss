use anyhow::Result;
use hashbrown::HashMap;
use moss_uikit::component::{layout::Order, primitive::Tooltip};

use super::{AnyPart, PartId};

pub type ActivityContainerId = &'static str;

pub type DescribeActivityContainerInput = (ActivityContainerId, Tooltip, Order);
pub type DescribeActivityContainerOutput<'a> = (&'a ActivityContainerId, &'a Tooltip, &'a Order);

pub struct ActivityBarPart {}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct ActivityBarItem {
    tooltip: Tooltip,
    order: Order,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct DescribeActivityBarOutput(HashMap<ActivityContainerId, ActivityBarItem>);

impl ActivityBarPart {
    pub fn new() -> Self {
        Self {}
    }
}

impl AnyPart for ActivityBarPart {
    const ID: PartId = crate::parts::ACTIVITY_BAR_PART;
    type DescribeOutput = DescribeActivityBarOutput;

    fn contribute(&self, layout: &mut crate::layout::Layout) {
        layout.add_tree_view_container(
            "leftActivityBar",
            "launchpad",
            (
                Tooltip {
                    header: "Launchpad",
                    text: Some(
                        "Explain behavior that is not clear from the setting or action name.",
                    ),
                    shortcut: Some("⌘⌥A"),
                    ..Default::default()
                },
                Order(1),
            ),
        );

        // self.entity_register
        //     .add_side_view("launchpad", ("Recently Viewed", Order))?;
    }

    fn describe(&self, layout: &crate::layout::Layout) -> Result<DescribeActivityBarOutput> {
        let mut result = DescribeActivityBarOutput(HashMap::new());

        if let Some(group) = layout.tree_view_container_groups.get("leftActivityBar") {
            for entity in group {
                let entity_ref = layout.registry.entity(*entity)?;
                let mut query = entity_ref.query::<DescribeActivityContainerOutput>();
                let (id, tooltip, order) = query.get().unwrap();

                result.0.insert(
                    id,
                    ActivityBarItem {
                        tooltip: tooltip.clone(),
                        order: order.clone(),
                    },
                );
            }
        }

        Ok(result)
    }
}
