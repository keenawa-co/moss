use crate::rule::{Operator, Rule};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::fmt::{Debug, Display};
use std::ops::Not;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};
use thiserror::Error;

// TODO: update documentation

#[derive(Debug, Error)]
pub enum RuleError {
    #[error("Operand '{operand:?}' have invalid type for operator '{operator}'")]
    InvalidType { operator: Operator, operand: Rule },
    #[error(
        "Operand '{left:?}' has incompatible type with operand '{right:?}' for equality checks"
    )]
    IncompatibleType { left: Rule, right: Rule },
    #[error("Cannot divide by zero")]
    ZeroDivision,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum ResultType {
    Number,
    String,
    Boolean,
    Array,
    Object,
    Variable,
    Undefined,
}

trait RuleType {
    fn get_type(&self) -> ResultType;
}

impl RuleType for Operator {
    fn get_type(&self) -> ResultType {
        match self {
            Operator::Equal => ResultType::Boolean,
            Operator::NotEqual => ResultType::Boolean,
            Operator::GreaterThan => ResultType::Boolean,
            Operator::LessThan => ResultType::Boolean,
            Operator::GreaterThanOrEqual => ResultType::Boolean,
            Operator::LessThanOrEqual => ResultType::Boolean,
            Operator::And => ResultType::Boolean,
            Operator::Or => ResultType::Boolean,
            Operator::Not => ResultType::Boolean,
            Operator::Add => ResultType::Number,
            Operator::Subtract => ResultType::Number,
            Operator::Multiply => ResultType::Number,
            Operator::Divide => ResultType::Number,
            Operator::Modulo => ResultType::Number,
            Operator::In => ResultType::Boolean,
            Operator::Cat => ResultType::Undefined,
            Operator::Map => ResultType::Variable,
            Operator::Reduce => ResultType::Variable,
            Operator::Filter => ResultType::Array,
            Operator::All => ResultType::Boolean,
            Operator::None => ResultType::Boolean,
            Operator::Some => ResultType::Boolean,
            Operator::Merge => ResultType::Array,
            Operator::If => ResultType::Undefined,
            Operator::Var => ResultType::Variable,
            Operator::Missing => ResultType::Undefined,
            Operator::MissingSome => ResultType::Undefined,
        }
    }
}

pub struct RuleWithValidation {
    raw_rule: Rule,
}

impl From<Rule> for RuleWithValidation {
    fn from(raw_rule: Rule) -> Self {
        RuleWithValidation { raw_rule }
    }
}

impl RuleType for RuleWithValidation {
    fn get_type(&self) -> ResultType {
        match &self.raw_rule {
            Rule::Constant(value) => match value {
                Value::Null => ResultType::Undefined,
                Value::Bool(_) => ResultType::Boolean,
                Value::Number(_) => ResultType::Number,
                Value::String(_) => ResultType::String,
                Value::Array(_) => ResultType::Array,
                Value::Object(_) => ResultType::Object,
            },
            Rule::Variable(_) => ResultType::Variable,
            Rule::Unary { operator, .. } => operator.get_type(),
            Rule::Binary { operator, .. } => operator.get_type(),
            Rule::Variadic { operator, .. } => operator.get_type(),
            Rule::Custom { .. } => ResultType::Variable,
        }
    }
}

impl RuleWithValidation {
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

    fn is_boolean_compatible(&self) -> bool {
        self.get_type() == ResultType::Boolean || self.get_type() == ResultType::Variable
    }

    fn is_number_compatible(&self) -> bool {
        self.get_type() == ResultType::Number || self.get_type() == ResultType::Variable
    }

    pub fn validate_boolean_operators(
        operator: Operator,
        operands: Vec<&RuleWithValidation>,
    ) -> Result<(), RuleError> {
        let invalid_operands = operands
            .iter()
            .filter(|x| !x.is_boolean_compatible())
            .collect::<Vec<_>>();
        if invalid_operands.is_empty() {
            Ok(())
        } else {
            Err(RuleError::InvalidType {
                operator,
                operand: invalid_operands[0].raw_rule.clone(),
            })
        }
    }

    pub fn validate_equality_operators(
        left: &RuleWithValidation,
        right: &RuleWithValidation,
    ) -> Result<(), RuleError> {
        if !left.is_type_compatible_with(right) {
            return Err(RuleError::IncompatibleType {
                left: left.raw_rule.clone(),
                right: right.raw_rule.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_numeric_operators(
        operator: Operator,
        operands: Vec<&RuleWithValidation>,
    ) -> Result<(), RuleError> {
        let invalid_operands = operands
            .iter()
            .filter(|x| !x.is_number_compatible())
            .collect::<Vec<_>>();
        if invalid_operands.is_empty() {
            Ok(())
        } else {
            Err(RuleError::InvalidType {
                operator,
                operand: invalid_operands[0].raw_rule.clone(),
            })
        }
    }

    pub fn validate_division(
        operator: Operator,
        right: &RuleWithValidation,
    ) -> Result<(), RuleError> {
        // For now we only check zero literal
        // In the future, we can check expressions that evaluate to zero
        if let Rule::Constant(divisor) = right.raw_rule.clone() {
            if !divisor.is_number() {
                return Err(RuleError::InvalidType {
                    operator,
                    operand: right.raw_rule.clone(),
                });
            }
            let divisor = divisor.as_number().unwrap();
            if divisor.is_f64() && divisor.as_f64().unwrap().abs() <= f64::EPSILON {
                Err(RuleError::ZeroDivision)
            } else if divisor.is_i64() && divisor.as_i64().unwrap() == 0 {
                Err(RuleError::ZeroDivision)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub fn value<V: Into<Value>>(value: V) -> Self {
        Rule::Constant(value.into()).into()
    }

    pub fn constant<V: Into<Value>>(value: V) -> Self {
        Rule::Constant(value.into()).into()
    }

    pub fn var<S: Into<String>>(name: S) -> Self {
        Rule::Variable(name.into()).into()
    }

    pub fn custom<S: Into<String>>(operator: S, operands: Vec<Self>) -> Self {
        Rule::Custom {
            operator: operator.into(),
            operands: operands.into_iter().map(|x| x.raw_rule).collect(),
        }
        .into()
    }

    pub fn not(self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::Not, vec![&self])?;
        Ok(self.raw_rule.not().into())
    }

    pub fn and(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::And, vec![&self, &other])?;
        Ok(self.raw_rule.and(other.raw_rule).into())
    }

    pub fn or(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::Or, vec![&self, &other])?;
        Ok(self.raw_rule.or(other.raw_rule).into())
    }

    pub fn eq(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_equality_operators(&self, &other)?;
        Ok(self.raw_rule.eq(other.raw_rule).into())
    }

    pub fn ne(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_equality_operators(&self, &other)?;
        Ok(self.raw_rule.ne(other.raw_rule).into())
    }

    pub fn gt(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::GreaterThan, vec![&self, &other])?;
        Ok(self.raw_rule.gt(other.raw_rule).into())
    }

    pub fn lt(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::LessThan, vec![&self, &other])?;
        Ok(self.raw_rule.lt(other.raw_rule).into())
    }

    pub fn gte(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::GreaterThanOrEqual, vec![&self, &other])?;
        Ok(self.raw_rule.gte(other.raw_rule).into())
    }

    pub fn lte(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::LessThanOrEqual, vec![&self, &other])?;
        Ok(self.raw_rule.lte(other.raw_rule).into())
    }

    pub fn add(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Add, vec![&self, &other])?;
        Ok(self.raw_rule.add(other.raw_rule).into())
    }

    pub fn subtract(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Subtract, vec![&self, &other])?;
        Ok(self.raw_rule.subtract(other.raw_rule).into())
    }

    pub fn multiply(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Multiply, vec![&self, &other])?;
        Ok(self.raw_rule.multiply(other.raw_rule).into())
    }
    pub fn divide(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Divide, vec![&self, &other])?;
        Self::validate_division(Operator::Divide, &other)?;
        Ok(self.raw_rule.divide(other.raw_rule).into())
    }

    pub fn modulo(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Modulo, vec![&self, &other])?;
        Self::validate_division(Operator::Modulo, &other)?;
        Ok(self.raw_rule.modulo(other.raw_rule).into())
    }
}

impl Serialize for RuleWithValidation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.raw_rule.serialize(serializer)
    }
}

impl From<&str> for RuleWithValidation {
    fn from(s: &str) -> Self {
        Rule::constant(s).into()
    }
}

impl From<String> for RuleWithValidation {
    fn from(s: String) -> Self {
        Rule::constant(s).into()
    }
}

impl From<i64> for RuleWithValidation {
    fn from(n: i64) -> Self {
        Rule::constant(n).into()
    }
}

impl From<f64> for RuleWithValidation {
    fn from(n: f64) -> Self {
        Rule::constant(n).into()
    }
}

impl From<bool> for RuleWithValidation {
    fn from(b: bool) -> Self {
        Rule::constant(b).into()
    }
}

impl From<Value> for RuleWithValidation {
    fn from(value: Value) -> Self {
        Rule::constant(value).into()
    }
}

impl BitAnd for RuleWithValidation {
    type Output = RuleWithValidation;

    fn bitand(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.and(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl BitOr for RuleWithValidation {
    type Output = RuleWithValidation;

    fn bitor(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.or(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl Add for RuleWithValidation {
    type Output = RuleWithValidation;

    fn add(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.add(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl Sub for RuleWithValidation {
    type Output = RuleWithValidation;

    fn sub(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.subtract(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl Mul for RuleWithValidation {
    type Output = RuleWithValidation;
    fn mul(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.multiply(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl Div for RuleWithValidation {
    type Output = RuleWithValidation;
    fn div(self, rhs: RuleWithValidation) -> RuleWithValidation {
        self.divide(rhs).map_err(|e| e.to_string()).unwrap()
    }
}

impl Not for RuleWithValidation {
    type Output = RuleWithValidation;

    fn not(self) -> RuleWithValidation {
        self.not().map_err(|e| e.to_string()).unwrap()
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
