// use serde::Serialize;
use serde_json::{json, Value};

pub struct Rule {
    pub value: Value,
}

pub struct RuleBuilder {
    conditions: Vec<Value>,
}

impl RuleBuilder {
    pub fn new() -> Self {
        RuleBuilder {
            conditions: Vec::new(),
        }
    }

    pub fn equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Equality condition (`==`)
        let condition = json!({ "==": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Strict equality condition (`===`)
        let condition = json!({ "===": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn and(mut self, other: RuleBuilder) -> Self {
        self.conditions.extend(other.conditions);
        self
    }

    pub fn or(mut self, conditions: Vec<RuleBuilder>) -> Self {
        let or_conditions: Vec<Value> = conditions.into_iter().map(|c| c.build().unwrap().value).collect();
        let condition = json!({ "or": or_conditions });
        self.conditions.push(condition);
        self
    }
    pub fn not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Inequality (`!=`)
        let condition = json!({ "!=": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Strict inequality (`!==`)
        let condition = json!({ "!==": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn not(mut self, condition: RuleBuilder) -> Self {
        // Logical NOT (`!`)
        let condition = json!({ "!": condition.build().unwrap().value });
        self.conditions.push(condition);
        self
    }

    pub fn double_not(mut self, key: &str) -> Self {
        // Double NOT (`!!`)
        let condition = json!({ "!!": { "var": key } });
        self.conditions.push(condition);
        self
    }


    pub fn less_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Less than (`<`)
        let condition = json!({ "<": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn less_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Less than or equal (`<=`)
        let condition = json!({ "<=": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Greater than (`>`)
        let condition = json!({ ">": [{ "var": key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Greater than or equal (`>=`)
        let condition = json!({ ">=": [{ "var": key }, value.into()] });
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