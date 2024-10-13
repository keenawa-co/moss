use serde::Serialize;
use specta::Type;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Action(pub &'static str);
