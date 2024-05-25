use chrono::Utc;
use types::id::NanoId;

#[derive(Serialize, Deserialize)]
pub struct AbstractNotification<T> {
    pub id: NanoId,
    #[serde(flatten)]
    pub body: T,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Notification {
    #[serde(rename = "system")]
    System(AbstractNotification<SystemNotification>),
    #[serde(rename = "client")]
    Client(AbstractNotification<ClientNotification>),
}

#[derive(Serialize, Deserialize)]
pub struct SystemNotification {
    pub message: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Severity {
    Hint,
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct ClientNotification {
    pub message: String,
    // pub severity: Severity,
}

impl Notification {
    pub fn create_client(message: String) -> Self {
        let body = ClientNotification { message };
        let payload = AbstractNotification {
            id: NanoId::new(),
            body,
            timestamp: Utc::now().timestamp(),
        };

        Self::Client(payload)
    }
}
