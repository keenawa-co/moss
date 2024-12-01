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
/// `RuleWithValidation` wraps a `RawRule` and ensures that the rule is valid
/// according to JSON Logic specifications. It performs type checking and other
/// validations to prevent the creation of invalid rules.
#[derive(Debug, Clone)]
pub struct RuleWithValidation {
    raw_rule: RawRule,
}

impl RuleWithValidation {
    /// Creates a new `RuleWithValidation` from a `RawRule`.
    ///
    /// This function validates the rule upon creation.
    pub fn new(raw_rule: RawRule) -> Result<Self, RuleError> {
        let rule_with_validation = RuleWithValidation { raw_rule };
        rule_with_validation.validate()?;
        Ok(rule_with_validation)
    }

    /// Validates the rule and its operands recursively.
    ///
    /// Ensures that all parts of the rule are valid, performing type checks and
    /// specific operator validations.
    fn validate(&self) -> Result<(), RuleError> {
        match &self.raw_rule {
            RawRule::Constant(_) | RawRule::Variable(_) => Ok(()),
            RawRule::Unary { operator, operand } => {
                let operand = RuleWithValidation {
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
                let left = RuleWithValidation {
                    raw_rule: *left.clone(),
                };
                let right = RuleWithValidation {
                    raw_rule: *right.clone(),
                };
                left.validate()?;
                right.validate()?;
                self.validate_binary_operator(operator, &left, &right)
            }
            RawRule::Variadic { operator, operands } => {
                for operand in operands {
                    let operand = RuleWithValidation {
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

    /// Validates a unary operator.
    ///
    /// Performs type checking specific to unary operators.
    fn validate_unary_operator(
        &self,
        operator: &Operator,
        operand: &RuleWithValidation,
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

    /// Validates a binary operator.
    ///
    /// Performs type checking specific to binary operators.
    fn validate_binary_operator(
        &self,
        operator: &Operator,
        left: &RuleWithValidation,
        right: &RuleWithValidation,
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

    /// Validates a variadic operator.
    ///
    /// Performs type checking specific to variadic operators.
    fn validate_variadic_operator(
        &self,
        operator: &Operator,
        operands: &[RawRule],
    ) -> Result<(), RuleError> {
        match operator {
            Operator::And | Operator::Or => {
                for operand in operands {
                    let operand = RuleWithValidation {
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
                    let operand = RuleWithValidation {
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

    /// Validates division by zero.
    ///
    /// Checks if the divisor is zero in division or modulo operations.
    fn validate_division_by_zero(&self, right: &RuleWithValidation) -> Result<(), RuleError> {
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

    /// Checks if the rule type is compatible with another rule's type.
    fn is_type_compatible_with(&self, other: &RuleWithValidation) -> bool {
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

    /// Checks if the rule is compatible with boolean operations.
    fn is_boolean_compatible(&self) -> bool {
        let ty = self.get_type();
        ty == ResultType::Boolean || ty == ResultType::Variable
    }

    /// Checks if the rule is compatible with numeric operations.
    fn is_number_compatible(&self) -> bool {
        let ty = self.get_type();
        ty == ResultType::Number || ty == ResultType::Variable
    }

    // ----------------------------------------------------------------------------
    // Constructor Methods
    //
    // These methods provide convenient ways to create new rules.
    // ----------------------------------------------------------------------------

    /// Creates a constant value rule.
    pub fn value<V: Into<Value>>(value: V) -> Self {
        RuleWithValidation {
            raw_rule: RawRule::value(value),
        }
    }

    /// Creates a constant value rule (alias for `value`).
    pub fn constant<V: Into<Value>>(value: V) -> Self {
        RuleWithValidation {
            raw_rule: RawRule::constant(value),
        }
    }

    /// Creates a variable reference rule.
    pub fn var<S: Into<String>>(name: S) -> Self {
        RuleWithValidation {
            raw_rule: RawRule::var(name),
        }
    }

    /// Creates a custom operation rule.
    pub fn custom<S: Into<String>>(operator: S, operands: Vec<Self>) -> Self {
        RuleWithValidation {
            raw_rule: RawRule::custom(operator, operands.into_iter().map(|r| r.raw_rule).collect()),
        }
    }

    // ----------------------------------------------------------------------------
    // Operator-Specific Methods
    //
    // These methods allow building complex rules using logical and arithmetic operators.
    // ----------------------------------------------------------------------------

    /// Applies the logical NOT operator.
    pub fn not(self) -> Result<Self, RuleError> {
        let rule = RawRule::unary(Operator::Not, self.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Combines two rules with logical AND.
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
        RuleWithValidation::new(rule)
    }

    /// Combines two rules with logical OR.
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
        RuleWithValidation::new(rule)
    }

    /// Checks if two rules are equal.
    pub fn eq(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Equal, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Checks if two rules are not equal.
    pub fn ne(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::NotEqual, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Checks if the first rule is greater than the second.
    pub fn gt(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::GreaterThan, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Checks if the first rule is less than the second.
    pub fn lt(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::LessThan, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Checks if the first rule is greater than or equal to the second.
    pub fn gte(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::GreaterThanOrEqual, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Checks if the first rule is less than or equal to the second.
    pub fn lte(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::LessThanOrEqual, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Adds two rules.
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
        RuleWithValidation::new(rule)
    }

    /// Subtracts the second rule from the first.
    pub fn subtract(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Subtract, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Multiplies two rules.
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
        RuleWithValidation::new(rule)
    }

    /// Divides the first rule by the second.
    pub fn divide(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Divide, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }

    /// Computes the remainder of the division of the first rule by the second.
    pub fn modulo(self, other: Self) -> Result<Self, RuleError> {
        let rule = RawRule::binary(Operator::Modulo, self.raw_rule, other.raw_rule);
        RuleWithValidation::new(rule)
    }
}

impl RuleType for RuleWithValidation {
    fn get_type(&self) -> ResultType {
        self.raw_rule.get_type()
    }
}

// ----------------------------------------------------------------------------
// Implementations of From traits
//
// These implementations allow for easy conversion from basic types to rules.
// ----------------------------------------------------------------------------

impl From<&str> for RuleWithValidation {
    fn from(s: &str) -> Self {
        RuleWithValidation::value(s)
    }
}

impl From<String> for RuleWithValidation {
    fn from(s: String) -> Self {
        RuleWithValidation::value(s)
    }
}

impl From<i64> for RuleWithValidation {
    fn from(n: i64) -> Self {
        RuleWithValidation::value(n)
    }
}

impl From<f64> for RuleWithValidation {
    fn from(n: f64) -> Self {
        RuleWithValidation::value(n)
    }
}

impl From<bool> for RuleWithValidation {
    fn from(b: bool) -> Self {
        RuleWithValidation::value(b)
    }
}

impl From<Value> for RuleWithValidation {
    fn from(value: Value) -> Self {
        RuleWithValidation::value(value)
    }
}

// ----------------------------------------------------------------------------
// Implement Serialize
//
// This allows RuleWithValidation to be serialized using Serde.
// ----------------------------------------------------------------------------

impl Serialize for RuleWithValidation {
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
// Enables the use of operators like +, -, *, /, &, |, ! on RuleWithValidation.
// ----------------------------------------------------------------------------

impl BitAnd for RuleWithValidation {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.and(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl BitOr for RuleWithValidation {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs).unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Add for RuleWithValidation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Sub for RuleWithValidation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.subtract(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Mul for RuleWithValidation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.multiply(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Div for RuleWithValidation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.divide(rhs)
            .unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

impl Not for RuleWithValidation {
    type Output = Self;

    fn not(self) -> Self {
        self.not().unwrap_or_else(|e| panic!("Rule error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moss_jsonlogic_macro::rule_with_validation;
    use serde_json::json;

    #[test]
    fn test_arithmetic_operations() {
        let var_x = RuleWithValidation::var("x");
        let var_y = RuleWithValidation::var("y");

        let const_ten = RuleWithValidation::value(10);

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
        let var_status = RuleWithValidation::var("status");

        let const_active = RuleWithValidation::from("active");

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
        let var_x = RuleWithValidation::var("x");
        let var_y = RuleWithValidation::var("y");
        let var_z = RuleWithValidation::var("z");

        let const_five = RuleWithValidation::from(5);
        let const_ten = RuleWithValidation::from(10);
        let const_twenty = RuleWithValidation::from(20);

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
        let var_a = RuleWithValidation::var("a");
        let var_b = RuleWithValidation::var("b");
        let var_c = RuleWithValidation::var("c");

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
        let var_score = RuleWithValidation::var("score");
        let var_bonus = RuleWithValidation::var("bonus");

        let const_threshold = RuleWithValidation::from(100);

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
        let var_status = RuleWithValidation::var("status");
        let var_attempts = RuleWithValidation::var("attempts");

        let const_locked = RuleWithValidation::value("locked");
        let const_three = RuleWithValidation::value(3);

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
        let var_a = RuleWithValidation::var("a");
        let var_b = RuleWithValidation::var("b");
        let var_c = RuleWithValidation::var("c");
        let var_d = RuleWithValidation::var("d");
        let var_e = RuleWithValidation::var("e");

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
        let var_input = RuleWithValidation::var("input");

        // Build rule using a custom operator "customOp"
        let custom_rule =
            RuleWithValidation::custom("customOp", vec![var_input, RuleWithValidation::from(42)]);

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
        let var_x = RuleWithValidation::var("x");
        let var_y = RuleWithValidation::var("y");
        let var_z = RuleWithValidation::var("z");
        let var_w = RuleWithValidation::var("w");

        let const_ten = RuleWithValidation::value(10);
        let const_five = RuleWithValidation::value(5);
        let const_three = RuleWithValidation::value(3);

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
        let var_status = RuleWithValidation::var("status");
        let var_attempts = RuleWithValidation::var("attempts");

        let const_locked = RuleWithValidation::from("locked");
        let const_max_attempts = RuleWithValidation::from(3);

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
        let var_a = RuleWithValidation::var("a");
        let var_b = RuleWithValidation::var("b");
        let var_c = RuleWithValidation::var("c");

        let rule1 = var_a.eq(RuleWithValidation::from(5)).unwrap();
        let rule2 = var_b.gt(RuleWithValidation::from(10)).unwrap();
        let rule3 = var_c.lt(RuleWithValidation::from(20)).unwrap();

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
        let rule = RuleWithValidation::var("view")
            .eq(RuleWithValidation::value("recents.view.id"))
            .unwrap()
            .and(
                RuleWithValidation::var("viewItem")
                    .eq(RuleWithValidation::value("recents.item"))
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
        let rule_and = RuleWithValidation::var("is_admin") & RuleWithValidation::var("is_active");
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
        let rule_or = RuleWithValidation::var("is_guest") | RuleWithValidation::var("is_banned");
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
        let rule_not = !RuleWithValidation::var("is_active");
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
        let rule_add = RuleWithValidation::var("quantity") + RuleWithValidation::value(10);
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
        let rule_sub = RuleWithValidation::var("total") - RuleWithValidation::value(20);
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
        let rule_mul = RuleWithValidation::var("price") * RuleWithValidation::value(2);
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
        let rule_div = RuleWithValidation::var("total") / RuleWithValidation::value(4);
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
        let rule = RuleWithValidation::var("age")
            .gte(RuleWithValidation::value(18))
            .unwrap()
            .and(
                RuleWithValidation::var("status")
                    .eq(RuleWithValidation::value("active"))
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
        let rule = RuleWithValidation::var("status")
            .ne(RuleWithValidation::value("inactive"))
            .unwrap()
            .and(
                RuleWithValidation::var("age")
                    .gte(RuleWithValidation::value(18))
                    .unwrap(),
            )
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

    #[test]
    fn test_rule_macro_simple_unary() {
        let rule = rule_with_validation!(!flag);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON"),
            json!({"!": {"var": "flag"}})
        );
    }

    #[test]
    fn test_rule_macro_simple_binary() {
        let rule = rule_with_validation!(age > 18);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON."),
            json!({ ">": [{ "var": "age" }, 18] })
        );
    }

    #[test]
    fn test_rule_macro_simple_variadic() {
        let rule = rule_with_validation!(a + b + c);
        println!("{}", serde_json::to_string_pretty(&rule).unwrap());
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON"),
            json!({"+" : [{"var" : "a"}, {"var" : "b"}, {"var" : "c"}]})
        )
    }

    #[test]
    fn test_rule_macro_logical_and() {
        let rule = rule_with_validation!(age > 18 && status == "active");
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON."),
            json!({
                "and": [
                    { ">": [{ "var": "age" }, 18] },
                    { "==": [{ "var": "status" }, "active"] }
                ]
            })
        );
    }

    #[test]
    fn test_rule_macro_complex() {
        let rule = rule_with_validation!((age > 18 && status == "active") || is_admin);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON."),
            json!({
                "or": [
                    {
                        "and": [
                            { ">": [{ "var": "age" }, 18] },
                            { "==": [{ "var": "status" }, "active"] }
                        ]
                    },
                    { "var": "is_admin" }
                ]
            })
        );
    }

    #[test]
    fn test_rule_macro_modulo() {
        let rule = rule_with_validation!(number % 2 == 0);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON."),
            json!({
                "==": [
                    {
                        "%": [{ "var": "number" }, 2]
                    },
                    0
                ]
            })
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_not() {
        let _ = rule_with_validation!(!"1");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_and() {
        let _ = rule_with_validation!(1 && true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_or() {
        let _ = rule_with_validation!("1" || true);
    }

    #[test]
    #[should_panic]
    fn test_incompatible_type_eq() {
        let _ = rule_with_validation!(1 == "1");
    }

    #[test]
    #[should_panic]
    fn test_incompatible_type_ne() {
        let _ = rule_with_validation!(true != "false");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_gt() {
        let _ = rule_with_validation!("42" > 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_lt() {
        let _ = rule_with_validation!(false < true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_gte() {
        let _ = rule_with_validation!("42" >= 42);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_lte() {
        let _ = rule_with_validation!(42 <= "42");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_add() {
        let _ = rule_with_validation!(true + "true");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_subtract() {
        let _ = rule_with_validation!("1" - 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_multiply() {
        let _ = rule_with_validation!("1" * 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_divide() {
        let _ = rule_with_validation!("foo" / "bar");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_modulo() {
        let _ = rule_with_validation!("3" % "2");
    }
    #[test]
    #[should_panic]
    fn test_zero_division_divide() {
        let _ = rule_with_validation!(42 / 0);
    }

    #[test]
    #[should_panic]
    fn test_zero_division_modulo() {
        let _ = rule_with_validation!(42 % 0.0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound1() {
        let _ = rule_with_validation!(1 + 2 / "3");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound2() {
        let _ = rule_with_validation!(x - true * false);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound3() {
        let _ = rule_with_validation!(x && !"true");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound5() {
        let _ = rule_with_validation!(3.14 < 159 - "26");
    }
}
