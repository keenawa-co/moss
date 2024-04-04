use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BehaverPreferenceFile {
    pub visual: VisualBehaverPreference,
    pub notification: NotificationBehaverPreference,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VisualBehaverPreference {
    pub theme: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NotificationBehaverPreference {
    pub sound: bool,
}
