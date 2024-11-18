use serde_json::{json, Value};
use lazy_static::lazy_static;
use moss_str::{read_only_str, ReadOnlyStr};

// Constants for AST parts
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
        format!("{:?}", self.value)
    }
}

trait BuildableRule {
    fn build_condition_tree(self) -> Result<Rule, &'static str>;
}

pub struct RuleBuilder {
    conditions: Vec<Value>,
}

impl RuleBuilder {
    pub fn new() -> Self {
        RuleBuilder { conditions: Vec::new() }
    }

    pub fn not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ NOT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ STRICT_NOT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ STRICT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn less_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ LESS_THAN_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }


    pub fn less_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ LESS_THAN_OR_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ GREATER_THAN_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ GREATER_THAN_OR_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn cast_to_boolean(mut self, key: &str) -> Self {
        let condition = json!({ DOUBLE_NOT_LITERAL.to_string(): { VAR_LITERAL.to_string(): key } });
        self.conditions.push(condition);
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        self.build_condition_tree()
    }
}

impl BuildableRule for RuleBuilder {
    fn build_condition_tree(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule { value: self.conditions.into_iter().next().unwrap() })
        } else {
            Ok(Rule { value: json!({ AND_LITERAL.to_string(): self.conditions }) })
        }
    }
}

pub struct LogicRuleBuilder {
    pub conditions: Vec<Value>,
}

impl LogicRuleBuilder {
    pub fn new() -> Self {
        LogicRuleBuilder { conditions: Vec::new() }
    }

    pub fn child(mut self, rule: impl BuildableRule) -> Self {
        match rule.build_condition_tree() {
            Ok(r) => self.conditions.push(r.value),
            Err(_) => (),
        }
        self
    }

    pub fn and(mut self) -> Self {
        let wrapped = json!({ AND_LITERAL.to_string(): self.conditions });
        self.conditions = vec![wrapped];
        self
    }

    pub fn or(mut self) -> Self {
        let wrapped = json!({ OR_LITERAL.to_string(): self.conditions });
        self.conditions = vec![wrapped];
        self
    }

    pub fn not(mut self) -> Self {
        if let Some(cond) = self.conditions.pop() {
            let wrapped = json!({ NOT_LITERAL.to_string(): cond });
            self.conditions.push(wrapped);
        }
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        self.build_condition_tree()
    }
}

impl BuildableRule for LogicRuleBuilder {
    fn build_condition_tree(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule { value: self.conditions.into_iter().next().unwrap() })
        } else {
            Ok(Rule { value: json!({ AND_LITERAL.to_string(): self.conditions }) })
        }
    }
}