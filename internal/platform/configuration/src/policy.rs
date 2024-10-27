use serde_json::Number;

pub enum PolicyDefinitionType {
    String,
    Number,
}

pub trait PolicyService {
    fn get_value(&self, name: impl ToString) -> Option<&serde_json::Value>;
}

pub struct EmptyPolicyService;

impl PolicyService for EmptyPolicyService {
    fn get_value(&self, _: impl ToString) -> Option<&serde_json::Value> {
        None
    }
}
