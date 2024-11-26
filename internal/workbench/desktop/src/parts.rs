// pub mod primary_activitybar;
// pub mod primary_sidebar;

use anyhow::Result;

use crate::RegistryManager;

pub type PartId = &'static str;

pub enum Parts {
    PrimaryActivityBar,
    SecondaryActivityBar,

    PrimarySideBar,
}

impl Parts {
    const PRIMARY_ACTIVITY_BAR: PartId = "workbench.part.primaryActivityBar";
    const SECONDARY_ACTIVITY_BAR: PartId = "workbench.part.secondaryActivityBar";

    const PRIMARY_SIDE_BAR: PartId = "workbench.part.primarySideBar";

    pub fn as_str(&self) -> &'static str {
        match &self {
            Parts::PrimaryActivityBar => Self::PRIMARY_ACTIVITY_BAR,
            Parts::SecondaryActivityBar => Self::SECONDARY_ACTIVITY_BAR,
            Parts::PrimarySideBar => Self::PRIMARY_SIDE_BAR,
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
