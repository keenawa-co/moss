#[cfg(feature = "graphql")]
use async_graphql::SimpleObject;

#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Preference {
    pub visual: VisualPreference,
    pub notification: NotificationPreference,
}

#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VisualPreference {
    pub theme: String,
}

#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct NotificationPreference {
    pub sound: bool,
}
