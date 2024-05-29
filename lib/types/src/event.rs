pub struct Event {
    pub route: String,
    pub correlation_token: String, // TODO: create own type with BoundedString
    pub idempotence_token: String, // TODO: create own type with BoundedString
    pub data: serde_json::Value,
}
