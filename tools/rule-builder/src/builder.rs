// use serde::Serialize;
use serde_json::{json, Value};

pub struct Rule {
    pub value: Value,
}

pub struct RuleBuilder {
    conditions: Vec<Value>, // Holds conditions to be combined in `and`
}

impl RuleBuilder {
    pub fn new() -> Self {
        RuleBuilder {
            conditions: Vec::new(),
        }
    }

    pub fn equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ "==": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn and(mut self, other: RuleBuilder) -> Self {
        self.conditions.extend(other.conditions);
        self
    }

    pub fn less_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Add a less-than condition
        let condition = json!({ "<": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule { value: self.conditions.into_iter().next().unwrap() })
        } else {
            // Multiple conditions - wrap in "and"
            Ok(Rule { value: json!({ "and": self.conditions }) })
        }
    }
}