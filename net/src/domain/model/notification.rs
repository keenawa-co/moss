use async_graphql::{Enum, MergedObject, Object, OutputType, SimpleObject, Union};
use chrono::Utc;
use types::id::NanoId;

#[derive(Serialize, Deserialize)]
pub struct NotificationPayload<T> {
    pub id: NanoId,
    #[serde(flatten)]
    pub body: T,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Notification {
    #[serde(rename = "system")]
    System(NotificationPayload<SystemNotification>),
    #[serde(rename = "client")]
    Client(NotificationPayload<ClientNotification>),
}

#[derive(Serialize, Deserialize)]
pub struct SystemNotification {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientNotification {
    pub message: String,
}

impl Notification {
    pub fn create_client(message: String) -> Self {
        let body = ClientNotification { message };
        let payload = NotificationPayload {
            id: NanoId::new(),
            body,
            timestamp: Utc::now().timestamp(),
        };

        Self::Client(payload)
    }
}
