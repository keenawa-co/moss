pub mod activitybar;
pub mod sidebar;

use anyhow::Result;

use crate::RegistryManager;

pub type PartId = &'static str;

pub enum Parts {
    ActivityBar,
    AuxiliaryBar,
}

impl Parts {
    const ACTIVITY_BAR: PartId = "workbench.part.activityBar";
    const AUXILIARY_BAR: PartId = "workbench.part.auxiliaryBar";

    pub fn as_part_id(&self) -> PartId {
        match &self {
            Parts::ActivityBar => Self::ACTIVITY_BAR,
            Parts::AuxiliaryBar => Self::AUXILIARY_BAR,
        }
    }
}

pub trait AnyPart {
    const ID: PartId;
    type DescribeOutput;

    fn id(&self) -> PartId {
        Self::ID
    }

    fn describe(&self, registry: &RegistryManager) -> Result<Self::DescribeOutput>;
}
