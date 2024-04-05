use serde::{Deserialize, Serialize};

#[cfg(feature = "gql")]
use async_graphql::SimpleObject;

#[cfg_attr(feature = "gql", derive(SimpleObject))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BehaverPreferenceConfig {
    pub visual: VisualBehaverPreference,
    pub notification: NotificationBehaverPreference,
}

#[cfg_attr(feature = "gql", derive(SimpleObject))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VisualBehaverPreference {
    pub theme: String,
}

#[cfg_attr(feature = "gql", derive(SimpleObject))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NotificationBehaverPreference {
    pub sound: bool,
}
