use anyhow::Result;
use hashbrown::HashMap;
use moss_uikit::component::{layout::Order, primitive::Tooltip};

use crate::EntityRegister;

pub type ActivityContainerId = &'static str;

pub type DescribeActivityContainerInput = (ActivityContainerId, Tooltip, Order);
pub type DescribeActivityContainerOutput<'a> = (&'a ActivityContainerId, &'a Tooltip, &'a Order);

pub struct ActivityBarPart {
    group_id: &'static str,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct ActivityBarItem {
    tooltip: Tooltip,
    order: Order,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct DescribeActivityBarOutput(HashMap<ActivityContainerId, ActivityBarItem>);

impl ActivityBarPart {
    pub fn new(side_view_container_group_id: &'static str) -> Self {
        Self {
            group_id: side_view_container_group_id,
        }
    }
    pub fn describe(&self, from: &EntityRegister) -> Result<DescribeActivityBarOutput> {
        let mut result = DescribeActivityBarOutput(HashMap::new());

        if let Some(group) = from.side_view_container_groups.get(self.group_id) {
            for entity in group {
                let entity_ref = from.frame.entity(*entity)?;
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
