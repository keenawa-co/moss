use serde::Serialize;
use serde_json::json;
use serde_json::Value;

pub struct Rule {
    pub value: serde_json::Value
}

pub struct RuleBuilder {
    conditions: Vec<Rule>,
}
impl RuleBuilder {
    pub fn new() -> Self {
        RuleBuilder {
            conditions: Vec::new(),
        }
    }

    pub fn equal<T: Serialize>(mut self, key: &str, value: T) -> Self {
        let json_value = json!({"==": [{"var": key}, value]});

        let rule = Rule {value: json_value};

        self.conditions.push(rule);
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        let combined = json!({
            "and": self.conditions.into_iter().map(|r| r.value).collect::<Vec<_>>()
        });
        Ok(Rule {value: combined})
    }
}
