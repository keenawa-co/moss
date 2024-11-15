// use serde::Serialize;
use serde_json::{json, Value};
use lazy_static::lazy_static;
use moss_str::{read_only_str, ReadOnlyStr};

// TODO:
// define AST parts ("===", "==", VAR_LITERAL, etc.) as constants 
#[rustfmt::skip]
lazy_static! {
    static ref EQUAL_LITERAL: ReadOnlyStr = read_only_str!("==");
    static ref STRICT_EQUAL_LITERAL: ReadOnlyStr = read_only_str!("===");
    static ref VAR_LITERAL: ReadOnlyStr = read_only_str!("var");
    static ref OR_LITERAL: ReadOnlyStr = read_only_str!("or");
    static ref AND_LITERAL: ReadOnlyStr = read_only_str!("and");
    static ref NOT_LITERAL: ReadOnlyStr = read_only_str!("!");
    static ref DOUBLE_NOT_LITERAL: ReadOnlyStr = read_only_str!("!!");
    static ref NOT_EQUAL_LITERAL: ReadOnlyStr = read_only_str!("!=");
    static ref STRICT_NOT_EQUAL_LITERAL: ReadOnlyStr = read_only_str!("!==");
    static ref LESS_THAN_LITERAL: ReadOnlyStr = read_only_str!("<");
    static ref LESS_THAN_OR_EQUAL_LITERAL: ReadOnlyStr = read_only_str!("<=");
    static ref GREATER_THAN_LITERAL: ReadOnlyStr = read_only_str!(">");
    static ref GREATER_THAN_OR_EQUAL_LITERAL: ReadOnlyStr = read_only_str!(">=");
}


pub struct Rule {
    pub value: Value,
}

impl ToString for Rule {
    fn to_string(&self) -> String {
        // should return READABLE string (example: jexl format)
        todo!()
    }
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
        let condition = json!({ EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Strict equality condition (`===`)
        let condition = json!({ STRICT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn and(mut self, other: RuleBuilder) -> Self {
        self.conditions.extend(other.conditions);
        self
    }

    pub fn or(mut self, conditions: Vec<RuleBuilder>) -> Self {
        let or_conditions: Vec<Value> = conditions.into_iter().map(|c| c.build().unwrap().value).collect();
        let condition = json!({ OR_LITERAL.to_string(): or_conditions });
        self.conditions.push(condition);
        self
    }
    pub fn not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Inequality (`!=`)
        let condition = json!({ NOT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Strict inequality (`!==`)
        let condition = json!({ STRICT_NOT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn not(mut self, condition: RuleBuilder) -> Self {
        // Logical NOT (`!`)
        let condition = json!({ NOT_LITERAL.to_string(): condition.build().unwrap().value });
        self.conditions.push(condition);
        self
    }

    pub fn double_not(mut self, key: &str) -> Self {
        // Double NOT (`!!`)
        let condition = json!({ DOUBLE_NOT_LITERAL.to_string(): { VAR_LITERAL.to_string(): key } });
        self.conditions.push(condition);
        self
    }


    pub fn less_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Less than (`<`)
        let condition = json!({ LESS_THAN_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn less_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Less than or equal (`<=`)
        let condition = json!({ LESS_THAN_OR_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Greater than (`>`)
        let condition = json!({ GREATER_THAN_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        // Greater than or equal (`>=`)
        let condition = json!({ GREATER_THAN_OR_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
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
            Ok(Rule { value: json!({ AND_LITERAL.to_string(): self.conditions }) })
        }
    }
}