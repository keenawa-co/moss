use crate::raw_rule::{Operator, RawRule};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Debug;
use std::ops::Not;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuleError {
    #[error("Operand '{operand:?}' has invalid type for operator '{operator}'")]
    InvalidType {
        operator: Operator,
        operand: RawRule,
    },
    #[error(
        "Operand '{left:?}' has incompatible type with operand '{right:?}' for equality checks"
    )]
    IncompatibleType { left: RawRule, right: RawRule },
    #[error("Cannot divide by zero")]
    ZeroDivision,
}

/// Represents the result type of a rule.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
enum ResultType {
    Number,
    String,
    Boolean,
    Array,
    Object,
    Variable,
    Undefined,
}

/// Trait to get the result type of a rule or operator.
trait RuleType {
    fn get_type(&self) -> ResultType;
}

impl RuleType for Operator {
    fn get_type(&self) -> ResultType {
        match self {
            // Comparison Operators
            Operator::Equal
            | Operator::NotEqual
            | Operator::GreaterThan
            | Operator::LessThan
            | Operator::GreaterThanOrEqual
            | Operator::LessThanOrEqual
            | Operator::In
            | Operator::All
            | Operator::None
            | Operator::Some => ResultType::Boolean,

            // Logical Operators
            Operator::And | Operator::Or | Operator::Not => ResultType::Boolean,

            // Arithmetic Operators
            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Modulo => ResultType::Number,

            // Array Operators
            Operator::Cat | Operator::Merge => ResultType::Array,
            Operator::Map | Operator::Reduce | Operator::Filter => ResultType::Array,

            // Miscellaneous Operators
            Operator::If | Operator::Missing | Operator::MissingSome => ResultType::Undefined,
            Operator::Var => ResultType::Variable,
        }
    }
}

impl RuleType for RawRule {
    fn get_type(&self) -> ResultType {
        match self {
            RawRule::Constant(value) => match value {
                Value::Null => ResultType::Undefined,
                Value::Bool(_) => ResultType::Boolean,
                Value::Number(_) => ResultType::Number,
                Value::String(_) => ResultType::String,
                Value::Array(_) => ResultType::Array,
                Value::Object(_) => ResultType::Object,
            },
            RawRule::Variable(_) => ResultType::Variable,
            RawRule::Unary { operator, .. } => operator.get_type(),
            RawRule::Binary { operator, .. } => operator.get_type(),
            RawRule::Variadic { operator, .. } => operator.get_type(),
            RawRule::Custom { .. } => ResultType::Variable, // Assume custom operators return variable type
        }
    }
}

/// Represents a JSON Logic rule with validation.
///
/// `CheckedRule` wraps a `RawRule` and ensures that the rule is valid
/// according to JSON Logic specifications. It performs type checking and other
/// validations to prevent the creation of invalid rules.
#[derive(Debug, Clone)]
pub struct CheckedRule {
    raw_rule: RawRule,
}

impl CheckedRule {
    pub fn new(raw_rule: RawRule) -> Result<Self, RuleError> {
        let rule_with_validation = CheckedRule { raw_rule };
        rule_with_validation.validate()?;
        Ok(rule_with_validation)
    }

    fn validate(&self) -> Result<(), RuleError> {
        match &self.raw_rule {
            RawRule::Constant(_) | RawRule::Variable(_) => Ok(()),
            RawRule::Unary { operator, operand } => {
                let operand = CheckedRule {
                    raw_rule: *operand.clone(),
                };
                operand.validate()?;
                self.validate_unary_operator(operator, &operand)
            }
            RawRule::Binary {
                operator,
                left,
                right,
            } => {
                let left = CheckedRule {
                    raw_rule: *left.clone(),
                };
                let right = CheckedRule {
                    raw_rule: *right.clone(),
                };
                left.validate()?;
                right.validate()?;
                self.validate_binary_operator(operator, &left, &right)
            }
            RawRule::Variadic { operator, operands } => {
                for operand in operands {
                    let operand = CheckedRule {
                        raw_rule: operand.clone(),
                    };
                    operand.validate()?;
                }
                self.validate_variadic_operator(operator, operands)
            }
            RawRule::Custom { .. } => {
                // Custom operators are not validated
                Ok(())
            }
        }
    }

    fn validate_unary_operator(
        &self,
        operator: &Operator,
        operand: &CheckedRule,
    ) -> Result<(), RuleError> {
        match operator {
            Operator::Not => {
                if operand.is_boolean_compatible() {
                    Ok(())
                } else {
                    Err(RuleError::InvalidType {
                        operator: operator.clone(),
                        operand: operand.raw_rule.clone(),
                    })
                }
            }
            _ => Ok(()), // Other unary operators can be added here
        }
    }

    fn validate_binary_operator(
        &self,
        operator: &Operator,
        left: &CheckedRule,
        right: &CheckedRule,
    ) -> Result<(), RuleError> {
        match operator {
            Operator::Equal | Operator::NotEqual => {
                if left.is_type_compatible_with(right) {
                    Ok(())
                } else {
                    Err(RuleError::IncompatibleType {
                        left: left.raw_rule.clone(),
                        right: right.raw_rule.clone(),
                    })
                }
            }
            Operator::GreaterThan
            | Operator::LessThan
            | Operator::GreaterThanOrEqual
            | Operator::LessThanOrEqual => {
                if left.is_number_compatible() && right.is_number_compatible() {
                    Ok(())
                } else {
                    Err(RuleError::InvalidType {
                        operator: operator.clone(),
                        operand: if !left.is_number_compatible() {
                            left.raw_rule.clone()
                        } else {
                            right.raw_rule.clone()
                        },
                    })
                }
            }
            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Modulo => {
                if left.is_number_compatible() && right.is_number_compatible() {
                    if *operator == Operator::Divide || *operator == Operator::Modulo {
                        self.validate_division_by_zero(right)
                    } else {
                        Ok(())
                    }
                } else {
                    Err(RuleError::InvalidType {
                        operator: operator.clone(),
                        operand: if !left.is_number_compatible() {
                            left.raw_rule.clone()
                        } else {
                            right.raw_rule.clone()
                        },
                    })
                }
            }
            _ => Ok(()), // Other binary operators can be added here
        }
    }

    fn validate_variadic_operator(
        &self,
        operator: &Operator,
        operands: &[RawRule],
    ) -> Result<(), RuleError> {
        match operator {
            Operator::And | Operator::Or => {
                for operand in operands {
                    let operand = CheckedRule {
                        raw_rule: operand.clone(),
                    };
                    if !operand.is_boolean_compatible() {
                        return Err(RuleError::InvalidType {
                            operator: operator.clone(),
                            operand: operand.raw_rule.clone(),
                        });
                    }
                }
                Ok(())
            }
            Operator::Add | Operator::Multiply => {
                for operand in operands {
                    let operand = CheckedRule {
                        raw_rule: operand.clone(),
                    };
                    if !operand.is_number_compatible() {
                        return Err(RuleError::InvalidType {
                            operator: operator.clone(),
                            operand: operand.raw_rule.clone(),
                        });
                    }
                }
                Ok(())
            }
            _ => Ok(()), // Other variadic operators can be added here
        }
    }

    fn validate_division_by_zero(&self, right: &CheckedRule) -> Result<(), RuleError> {
        if let RawRule::Constant(value) = &right.raw_rule {
            if value.is_number() {
                let number = value.as_f64().unwrap_or(0.0);
                if number.abs() <= f64::EPSILON {
                    return Err(RuleError::ZeroDivision);
                }
            }
        }
        Ok(())
    }

    fn is_type_compatible_with(&self, other: &CheckedRule) -> bool {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type == other_type {
            true
        } else if self_type == ResultType::Variable || other_type == ResultType::Variable {
            true
        } else {
            false
        }
    }

    fn is_boolean_compatible(&self) -> bool {
        let ty = self.get_type();
        ty == ResultType::Boolean || ty == ResultType::Variable
    }

    fn is_number_compatible(&self) -> bool {
        let ty = self.get_type();
        ty == ResultType::Number || ty == ResultType::Variable
    }

    // ----------------------------------------------------------------------------
    // Constructor Methods
    //
    // These methods provide convenient ways to create new rules.
    // ----------------------------------------------------------------------------

    pub fn value<V: Into<Value>>(value: V) -> Self {
        CheckedRule {
            raw_rule: RawRule::value(value),
        }
    }

    pub fn constant<V: Into<Value>>(value: V) -> Self {
        CheckedRule {
            raw_rule: RawRule::constant(value),
        }
    }

    pub fn var<S: Into<String>>(name: S) -> Self {
        CheckedRule {
            raw_rule: RawRule::var(name),
        }
    }

    pub fn custom<S: Into<String>>(operator: S, operands: Vec<Self>) -> Self {
        CheckedRule {
            raw_rule: RawRule::custom(operator, operands.into_iter().map(|r| r.raw_rule).collect()),
        }
    }

    // ----------------------------------------------------------------------------
    // Operator-Specific Methods
    //
    // These methods allow building complex rules using logical and arithmetic operators.
    // ----------------------------------------------------------------------------

    pub fn not(self) -> Result<Self, RuleError> {
        let rule = RawRule::unary(Operator::Not, self.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn and(self, other: Self) -> Result<Self, RuleError> {
        let rule = match self.raw_rule {
            RawRule::Variadic {
                operator: Operator::And,
                mut operands,
            } => {
                operands.push(other.raw_rule);
                RawRule::Variadic {
                    operator: Operator::And,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::And, vec![self.raw_rule, other.raw_rule]),
        };
        CheckedRule::new(rule)
    }

    pub fn or(self, other: Self) -> Result<Self, RuleError> {
        let rule = match self.raw_rule {
            RawRule::Variadic {
                operator: Operator::Or,
                mut operands,
            } => {
                operands.push(other.raw_rule);
                RawRule::Variadic {
                    operator: Operator::Or,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Or, vec![self.raw_rule, other.raw_rule]),
        };
        CheckedRule::new(rule)
    }

    pub fn eq(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Equal, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn ne(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::NotEqual, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn gt(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::GreaterThan, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn lt(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::LessThan, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn gte(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::GreaterThanOrEqual, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn lte(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::LessThanOrEqual, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn add(self, other: Self) -> Result<Self, RuleError> {
        let rule = match self.raw_rule {
            RawRule::Variadic {
                operator: Operator::Add,
                mut operands,
            } => {
                operands.push(other.raw_rule);
                RawRule::Variadic {
                    operator: Operator::Add,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Add, vec![self.raw_rule, other.raw_rule]),
        };
        CheckedRule::new(rule)
    }

    pub fn subtract(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Subtract, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn multiply(self, other: Self) -> Result<Self, RuleError> {
        let rule = match self.raw_rule {
            RawRule::Variadic {
                operator: Operator::Multiply,
                mut operands,
            } => {
                operands.push(other.raw_rule);
                RawRule::Variadic {
                    operator: Operator::Multiply,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Multiply, vec![self.raw_rule, other.raw_rule]),
        };
        CheckedRule::new(rule)
    }

    pub fn divide(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Divide, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }

    pub fn modulo(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Modulo, self.raw_rule, other.raw_rule);
        CheckedRule::new(rule)
    }
}

impl RuleType for CheckedRule {
    fn get_type(&self) -> ResultType {
        self.raw_rule.get_type()
    }
}

// ----------------------------------------------------------------------------
// Implementations of From traits
//
// These implementations allow for easy conversion from basic types to rules.
// ----------------------------------------------------------------------------

impl From<&str> for CheckedRule {
    fn from(s: &str) -> Self {
        CheckedRule::value(s)
    }
}

impl From<String> for CheckedRule {
    fn from(s: String) -> Self {
        CheckedRule::value(s)
    }
}

impl From<i64> for CheckedRule {
    fn from(n: i64) -> Self {
        CheckedRule::value(n)
    }
}

impl From<f64> for CheckedRule {
    fn from(n: f64) -> Self {
        CheckedRule::value(n)
    }
}

impl From<bool> for CheckedRule {
    fn from(b: bool) -> Self {
        CheckedRule::value(b)
    }
}

impl From<Value> for CheckedRule {
    fn from(value: Value) -> Self {
        CheckedRule::value(value)
    }
}

// ----------------------------------------------------------------------------
// Implement Serialize
//
// This allows CheckedRule to be serialized using Serde.
// ----------------------------------------------------------------------------

impl Serialize for CheckedRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.raw_rule.serialize(serializer)
    }
}

// ----------------------------------------------------------------------------
// Operator Overloading
//
// Enables the use of operators like +, -, *, /, &, |, ! on CheckedRule.
// ----------------------------------------------------------------------------

impl BitAnd for CheckedRule {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.and(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl BitOr for CheckedRule {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs).unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Add for CheckedRule {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Sub for CheckedRule {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.subtract(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Mul for CheckedRule {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.multiply(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Div for CheckedRule {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.divide(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Not for CheckedRule {
    type Output = Self;

    fn not(self) -> Self {
        self.not().unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_arithmetic_operations() {
        let var_x = CheckedRule::var("x");
        let var_y = CheckedRule::var("y");

        let const_ten = CheckedRule::value(10);

        // Build rule: (x + y) > 10
        let rule = (var_x + var_y).gt(const_ten).unwrap();

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            ">": [
                { "+": [ { "var": "x" }, { "var": "y" } ] },
                10
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_logical_not() {
        let var_status = CheckedRule::var("status");

        let const_active = CheckedRule::from("active");

        // Build rule: !(status == "active")
        let rule = !(var_status.eq(const_active).unwrap());

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            "!": {
                "==": [
                    { "var": "status" },
                    "active"
                ]
            }
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_complex_nested_rules() {
        let var_x = CheckedRule::var("x");
        let var_y = CheckedRule::var("y");
        let var_z = CheckedRule::var("z");

        let const_five = CheckedRule::from(5);
        let const_ten = CheckedRule::from(10);
        let const_twenty = CheckedRule::from(20);

        // Build rules
        let rule1 = var_x.gt(const_five).unwrap(); // x > 5
        let rule2 = var_y.lt(const_ten).unwrap(); // y < 10
        let rule3 = var_z.eq(const_twenty).unwrap(); // z == 20

        // Combine rules: (x > 5 AND y < 10) OR z == 20
        let combined_rule = (rule1 & rule2) | rule3;

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(combined_rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            "or": [
                {
                    "and": [
                        { ">": [ { "var": "x" }, 5 ] },
                        { "<": [ { "var": "y" }, 10 ] }
                    ]
                },
                { "==": [ { "var": "z" }, 20 ] }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_chained_arithmetic_operations() {
        let var_a = CheckedRule::var("a");
        let var_b = CheckedRule::var("b");
        let var_c = CheckedRule::var("c");

        // Build rule: (a * b) + c
        let arithmetic_rule = (var_a * var_b) + var_c;

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(arithmetic_rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            "+": [
                { "*": [ { "var": "a" }, { "var": "b" } ] },
                { "var": "c" }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_combining_logical_and_arithmetic_operations() {
        let var_score = CheckedRule::var("score");
        let var_bonus = CheckedRule::var("bonus");

        let const_threshold = CheckedRule::from(100);

        // Build rule: (score + bonus) >= 100
        let rule = (var_score + var_bonus).gte(const_threshold).unwrap();

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            ">=": [
                { "+": [ { "var": "score" }, { "var": "bonus" } ] },
                100
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_complex_nested_rules_with_not() {
        let var_status = CheckedRule::var("status");
        let var_attempts = CheckedRule::var("attempts");

        let const_locked = CheckedRule::value("locked");
        let const_three = CheckedRule::value(3);

        // Build rule: !(status == "locked" || attempts > 3)
        let rule = !(var_status.eq(const_locked).unwrap() | var_attempts.gt(const_three).unwrap());

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Define the expected JSON Logic
        let expected_json = json!({
            "!": {
                "or": [
                    { "==": [ { "var": "status" }, "locked" ] },
                    { ">": [ { "var": "attempts" }, 3 ] }
                ]
            }
        });

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_multiple_arithmetic_operations() {
        let var_a = CheckedRule::var("a");
        let var_b = CheckedRule::var("b");
        let var_c = CheckedRule::var("c");
        let var_d = CheckedRule::var("d");
        let var_e = CheckedRule::var("e");

        // Build rule: (a * b) + (c / d) - e
        let rule = (var_a * var_b) + (var_c / var_d) - var_e;

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Define the expected JSON Logic
        let expected_json = json!({
            "-": [
                { "+": [
                    { "*": [ { "var": "a" }, { "var": "b" } ] },
                    { "/": [ { "var": "c" }, { "var": "d" } ] }
                ] },
                { "var": "e" }
            ]
        });

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_custom_operator() {
        let var_input = CheckedRule::var("input");

        // Build rule using a custom operator "customOp"
        let custom_rule = CheckedRule::custom("customOp", vec![var_input, CheckedRule::from(42)]);

        let json_logic =
            serde_json::to_value(custom_rule).expect("Failed to serialize the rule into JSON.");
        let expected_json = json!({
            "customOp": [
                { "var": "input" },
                42
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_combined_logical_and_arithmetic_operations() {
        let var_x = CheckedRule::var("x");
        let var_y = CheckedRule::var("y");
        let var_z = CheckedRule::var("z");
        let var_w = CheckedRule::var("w");

        let const_ten = CheckedRule::value(10);
        let const_five = CheckedRule::value(5);
        let const_three = CheckedRule::value(3);

        let rule_sum = var_x + var_y; // x + y
        let rule_gt = rule_sum.gt(const_ten).unwrap(); // (x + y) > 10
        let rule_le = var_z.lte(const_five).unwrap(); // z <= 5
        let rule_ne = var_w.ne(const_three).unwrap(); // w != 3
        let rule_or = rule_le | rule_ne; // (z <= 5 OR w != 3)
        let combined_rule = rule_gt & rule_or; // (x + y) > 10 AND (z <= 5 OR w != 3)

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(combined_rule).expect("Failed to serialize the rule into JSON.");

        // Define the expected JSON Logic
        let expected_json = json!({
            "and": [
                {
                    ">": [
                        { "+": [ { "var": "x" }, { "var": "y" } ] },
                        10
                    ]
                },
                {
                    "or": [
                        { "<=": [ { "var": "z" }, 5 ] },
                        { "!=": [ { "var": "w" }, 3 ] }
                    ]
                }
            ]
        });

        // Assert that the serialized JSON matches the expected JSON
        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_complex_rule_with_not_and_operator_overloading() {
        let var_status = CheckedRule::var("status");
        let var_attempts = CheckedRule::var("attempts");

        let const_locked = CheckedRule::from("locked");
        let const_max_attempts = CheckedRule::from(3);

        // Build rule: !(status == "locked" || attempts > 3)
        let rule =
            !(var_status.eq(const_locked).unwrap() | var_attempts.gt(const_max_attempts).unwrap());

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            "!": {
                "or": [
                    { "==": [ { "var": "status" }, "locked" ] },
                    { ">": [ { "var": "attempts" }, 3 ] }
                ]
            }
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_logical_operations() {
        let var_a = CheckedRule::var("a");
        let var_b = CheckedRule::var("b");
        let var_c = CheckedRule::var("c");

        let rule1 = var_a.eq(CheckedRule::from(5)).unwrap();
        let rule2 = var_b.gt(CheckedRule::from(10)).unwrap();
        let rule3 = var_c.lt(CheckedRule::from(20)).unwrap();

        // Combine rules using logical operators
        let combined_rule = rule1 & (rule2 | rule3);

        // Serialize to JSON Logic
        let json_logic =
            serde_json::to_value(combined_rule).expect("Failed to serialize the rule into JSON.");

        // Expected JSON Logic
        let expected_json = json!({
            "and": [
                { "==": [ { "var": "a" }, 5 ] },
                {
                    "or": [
                        { ">": [ { "var": "b" }, 10 ] },
                        { "<": [ { "var": "c" }, 20 ] }
                    ]
                }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_rule_with_desired_api() {
        let rule = CheckedRule::var("view")
            .eq(CheckedRule::value("recents.view.id"))
            .unwrap()
            .and(
                CheckedRule::var("viewItem")
                    .eq(CheckedRule::value("recents.item"))
                    .unwrap(),
            )
            .unwrap();

        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        let expected_json = json!({
            "and": [
                {
                    "==": [
                        { "var": "view" },
                        "recents.view.id"
                    ]
                },
                {
                    "==": [
                        { "var": "viewItem" },
                        "recents.item"
                    ]
                }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_operator_overloading() {
        // Logical AND
        let rule_and = CheckedRule::var("is_admin") & CheckedRule::var("is_active");
        let expected_and = json!({
            "and": [
                { "var": "is_admin" },
                { "var": "is_active" }
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_and).expect("Failed to serialize the rule into JSON."),
            expected_and
        );

        // Logical OR
        let rule_or = CheckedRule::var("is_guest") | CheckedRule::var("is_banned");
        let expected_or = json!({
            "or": [
                { "var": "is_guest" },
                { "var": "is_banned" }
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_or).expect("Failed to serialize the rule into JSON."),
            expected_or
        );

        // Logical NOT
        let rule_not = !CheckedRule::var("is_active");
        let expected_not = json!({
            "!": {
                "var": "is_active"
            }
        });
        assert_eq!(
            serde_json::to_value(rule_not).expect("Failed to serialize the rule into JSON."),
            expected_not
        );

        // Addition
        let rule_add = CheckedRule::var("quantity") + CheckedRule::value(10);
        let expected_add = json!({
            "+": [
                { "var": "quantity" },
                10
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_add).expect("Failed to serialize the rule into JSON."),
            expected_add
        );

        // Subtraction
        let rule_sub = CheckedRule::var("total") - CheckedRule::value(20);
        let expected_sub = json!({
            "-": [
                { "var": "total" },
                20
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_sub).expect("Failed to serialize the rule into JSON."),
            expected_sub
        );

        // Multiplication
        let rule_mul = CheckedRule::var("price") * CheckedRule::value(2);
        let expected_mul = json!({
            "*": [
                { "var": "price" },
                2
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_mul).expect("Failed to serialize the rule into JSON."),
            expected_mul
        );

        // Division
        let rule_div = CheckedRule::var("total") / CheckedRule::value(4);
        let expected_div = json!({
            "/": [
                { "var": "total" },
                4
            ]
        });
        assert_eq!(
            serde_json::to_value(rule_div).expect("Failed to serialize the rule into JSON."),
            expected_div
        );
    }

    #[test]
    fn test_method_chaining() {
        let rule = CheckedRule::var("age")
            .gte(CheckedRule::value(18))
            .unwrap()
            .and(
                CheckedRule::var("status")
                    .eq(CheckedRule::value("active"))
                    .unwrap(),
            )
            .unwrap();

        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        let expected_json = json!({
            "and": [
                {
                    ">=": [
                        { "var": "age" },
                        18
                    ]
                },
                {
                    "==": [
                        { "var": "status" },
                        "active"
                    ]
                }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }

    #[test]
    fn test_rule_with_variables_and_values() {
        let rule = CheckedRule::var("status")
            .ne(CheckedRule::value("inactive"))
            .unwrap()
            .and(CheckedRule::var("age").gte(CheckedRule::value(18)).unwrap())
            .unwrap();

        let json_logic =
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON.");

        let expected_json = json!({
            "and": [
                {
                    "!=": [
                        { "var": "status" },
                        "inactive"
                    ]
                },
                {
                    ">=": [
                        { "var": "age" },
                        18
                    ]
                }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }
}
