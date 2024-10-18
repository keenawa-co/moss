use anyhow::Result;
use hashbrown::HashMap;
use moss_hecs::{BuiltEntity, DynamicBundle, EntityBuilder};

use crate::layout::Layout;

pub mod activitybar;
pub mod headbar;

pub type PartId = &'static str;

pub const ACTIVITY_BAR_PART: PartId = "activityBar";

pub trait AnyPart {
    const ID: PartId;
    type DescribeOutput;

    fn id(&self) -> PartId {
        Self::ID
    }
    fn contribute(&self, layout: &mut Layout);
    fn describe(&self, layout: &Layout) -> Result<Self::DescribeOutput>;
}
