use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[ts(export, export_to = "models.ts")]
pub struct Action(pub &'static str);