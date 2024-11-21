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
        let condition = json!({ EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn strict_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ STRICT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn not_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ NOT_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn greater_than(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ GREATER_THAN_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
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

    pub fn greater_than_or_equal(mut self, key: &str, value: impl Into<Value>) -> Self {
        let condition = json!({ GREATER_THAN_OR_EQUAL_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }, value.into()] });
        self.conditions.push(condition);
        self
    }

    pub fn cast_to_boolean(mut self, key: &str) -> Self {
        let condition = json!({ DOUBLE_NOT_LITERAL.to_string(): [{ VAR_LITERAL.to_string(): key }] });
        self.conditions.push(condition);
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule {
                value: self.conditions.into_iter().next().unwrap(),
            })
        } else {
            Ok(Rule {
                value: json!(self.conditions),
            })
        }
    }
}

// LogicRuleBuilder with state handling for chaining
pub struct LogicRuleBuilder<State> {
    conditions: Vec<Value>,
    _state: std::marker::PhantomData<State>,
}

// State markers
pub struct Initial;
pub struct AfterChild;
pub struct AfterLogicOp;

impl LogicRuleBuilder<Initial> {
    pub fn new() -> Self {
        LogicRuleBuilder {
            conditions: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }

    pub fn child(self, rule: impl BuildableRule) -> LogicRuleBuilder<AfterChild> {
        let mut conditions = self.conditions;
        if let Ok(r) = rule.build_condition_tree() {
            conditions.push(r.value);
        }
        LogicRuleBuilder {
            conditions,
            _state: std::marker::PhantomData,
        }
    }
}

impl LogicRuleBuilder<AfterChild> {
    pub fn and(self) -> LogicRuleBuilder<AfterLogicOp> {
        let wrapped = json!({ AND_LITERAL.to_string(): self.conditions });
        
        LogicRuleBuilder {
            conditions: vec![wrapped],
            _state: std::marker::PhantomData,
        }
    }

    pub fn or(self) -> LogicRuleBuilder<AfterLogicOp> {
        let wrapped = json!({ OR_LITERAL.to_string(): self.conditions });
        
        LogicRuleBuilder {
            conditions: vec![wrapped],
            _state: std::marker::PhantomData,
        }
    }

    pub fn not(mut self) -> Self {
        if let Some(cond) = self.conditions.pop() {
            self.conditions.push(json!({
                NOT_LITERAL.to_string(): cond // Wrap the last condition under "!"
            }));
        }
        self
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        self.build_condition_tree()
    }
}

impl LogicRuleBuilder<AfterLogicOp> {
    pub fn child(self, rule: impl BuildableRule) -> LogicRuleBuilder<AfterChild> {
        let mut conditions = self.conditions;
        if let Ok(r) = rule.build_condition_tree() {
            conditions.push(r.value);
        }
        LogicRuleBuilder {
            conditions,
            _state: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> Result<Rule, &'static str> {
        self.build_condition_tree()
    }
}

// BuildableRule Trait
trait BuildableRule {
    fn build_condition_tree(self) -> Result<Rule, &'static str>;
}

impl BuildableRule for LogicRuleBuilder<AfterChild> {
    fn build_condition_tree(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule {
                value: self.conditions.into_iter().next().unwrap(),
            })
        } else {
            Ok(Rule {
                value: json!(self.conditions),
            })
        }
    }
}

impl BuildableRule for LogicRuleBuilder<AfterLogicOp> {
    fn build_condition_tree(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule {
                value: self.conditions.into_iter().next().unwrap(),
            })
        } else {
            Ok(Rule {
                value: json!(self.conditions),
            })
        }
    }
}

// Implement BuildableRule for RuleBuilder
impl BuildableRule for RuleBuilder {
    fn build_condition_tree(self) -> Result<Rule, &'static str> {
        if self.conditions.is_empty() {
            Err("No conditions provided")
        } else if self.conditions.len() == 1 {
            Ok(Rule {
                value: self.conditions.into_iter().next().unwrap(),
            })
        } else {
            Ok(Rule {
                value: json!(self.conditions),
            })
        }
    }
}