use crate::rule::TypeError::{IncompatibleType, InvalidType};
use crate::rule::ValueError::ZeroDivision;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Not, Sub};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuleError {
    TypeError(TypeError),
    ValueError(ValueError),
}

impl Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleError::TypeError(e) => e.fmt(f),
            RuleError::ValueError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Operand '{operand:?}' have invalid type for operator '{operator}'")]
    InvalidType { operator: Operator, operand: Rule },
    #[error(
        "Operand '{left:?}' has incompatible type with operand '{right:?}' for equality checks"
    )]
    IncompatibleType { left: Rule, right: Rule },
}

#[derive(Debug, Error)]
pub enum ValueError {
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

/// Represents the standard JSON Logic operators.
///
/// The `Operator` enum encompasses all the standard operators defined by JSON Logic,
/// including comparison, logical, arithmetic, array, and miscellaneous operators.
/// Each variant is serialized with its corresponding JSON Logic string representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operator {
    // Comparison Operators
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<=")]
    LessThanOrEqual,

    // Logical Operators
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "!")]
    Not,

    // Arithmetic Operators
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "/")]
    Divide,
    #[serde(rename = "%")]
    Modulo,

    // Array Operators
    #[serde(rename = "in")]
    In,
    #[serde(rename = "cat")]
    Cat,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "reduce")]
    Reduce,
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "some")]
    Some,
    #[serde(rename = "merge")]
    Merge,

    // Miscellaneous Operators
    #[serde(rename = "if")]
    If,
    #[serde(rename = "var")]
    Var,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "missing_some")]
    MissingSome,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            // Comparison Operators
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThanOrEqual => "<=",

            // Logical Operators
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Not => "!",

            // Arithmetic Operators
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",

            // Array Operators
            Operator::In => "in",
            Operator::Cat => "cat",
            Operator::Map => "map",
            Operator::Reduce => "reduce",
            Operator::Filter => "filter",
            Operator::All => "all",
            Operator::None => "none",
            Operator::Some => "some",
            Operator::Merge => "merge",

            // Miscellaneous Operators
            Operator::If => "if",
            Operator::Var => "var",
            Operator::Missing => "missing",
            Operator::MissingSome => "missing_some",
        };
        write!(f, "{}", op_str)
    }
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

/// Represents a JSON Logic rule.
///
/// The `Rule` enum is a comprehensive representation of all possible JSON Logic constructs,
/// including constants, variables, unary operations, binary operations, variadic operations,
/// and custom operations. This structure allows for the flexible and expressive creation of
/// complex logical expressions.
///
/// ## Variants
///
/// - `Constant(Value)`: Represents a constant value such as a number, string, boolean, null, array, or object.
/// - `Variable(String)`: Represents a variable reference, e.g., `{"var": "x"}`.
/// - `Unary { operator, operand }`: Represents a unary operation, e.g., `{"!": {...}}`.
/// - `Binary { operator, left, right }`: Represents a binary operation, e.g., `{"==": [left, right]}`.
/// - `Variadic { operator, operands }`: Represents a variadic operation, e.g., `{"and": [op1, op2, ...]}`.
/// - `Custom { operator, operands }`: Represents a custom operation with an arbitrary structure.
///
/// ## Examples
///
/// ### Creating a Variable
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
///
/// let rule = Rule::var("age");
/// ```
///
/// ### Creating a Constant
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
///
/// let rule = Rule::value(30);
/// ```
///
/// ### Creating Multiple Constants
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
///
/// let rule = Rule::value("hello");
/// let number_rule = Rule::value(42);
/// let bool_rule = Rule::value(true);
/// ```
///
/// ### Creating a Binary Operation
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
///
/// let rule = Rule::var("age").gt(Rule::value(18));
/// ```
///
/// ### Combining Rules with Logical AND
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
///
/// let rule = Rule::var("age")
///     .gt(Rule::value(18))
///     .and(Rule::var("status").eq(Rule::value("active")));
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Rule {
    /// A constant value (number, string, boolean, null, array, or object).
    Constant(Value),

    /// A variable reference, e.g., {"var": "x"}.
    Variable(String),

    /// A unary operation, e.g., {"!": {...}}.
    Unary {
        /// The operator for the unary operation.
        operator: Operator,
        /// The operand of the unary operation.
        operand: Box<Rule>,
    },

    /// A binary operation, e.g., {"==": [left, right]}.
    Binary {
        /// The operator for the binary operation.
        operator: Operator,
        /// The left operand of the binary operation.
        left: Box<Rule>,
        /// The right operand of the binary operation.
        right: Box<Rule>,
    },

    /// A variadic operation, e.g., {"and": [op1, op2, ...]}.
    Variadic {
        /// The operator for the variadic operation.
        operator: Operator,
        /// The operands of the variadic operation.
        operands: Vec<Rule>,
    },

    /// A custom operation with arbitrary structure.
    Custom {
        /// The custom operator.
        operator: String,
        /// The operands for the custom operator.
        operands: Vec<Rule>,
    },
}

impl RuleType for Rule {
    fn get_type(&self) -> ResultType {
        match self {
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

impl Rule {
    fn is_type_compatible_with(&self, other: &Rule) -> bool {
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
}

impl Rule {
    // ----------------------------------------------------------------------------
    // Constructor Methods
    //
    // This section provides methods to create various types of rules, including
    // constants, variables, unary operations, binary operations, variadic operations,
    // and custom operations. These constructors form the foundation for building
    // complex JSON Logic expressions.
    // ----------------------------------------------------------------------------

    /// Creates a constant value.
    ///
    /// This method allows you to create a `Rule::Constant` from any type that can be
    /// converted into a `serde_json::Value`. It supports multiple data types, including
    /// numbers, strings, booleans, nulls, arrays, and objects.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::value("hello");
    /// let number_rule = Rule::value(42);
    /// let bool_rule = Rule::value(true);
    /// ```
    pub fn value<V: Into<Value>>(value: V) -> Self {
        Rule::Constant(value.into())
    }

    /// Creates a constant value.
    ///
    /// Alias for `Rule::value`. This method is provided for convenience and clarity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::constant("world");
    /// ```
    pub fn constant<V: Into<Value>>(value: V) -> Self {
        Rule::Constant(value.into())
    }

    /// Creates a variable reference.
    ///
    /// This method creates a `Rule::Variable` which references a variable by its name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("user_age");
    /// ```
    pub fn var<S: Into<String>>(name: S) -> Self {
        Rule::Variable(name.into())
    }

    /// Creates a unary operation.
    ///
    /// This method constructs a `Rule::Unary` representing a unary operation such as logical NOT.
    ///
    /// # Parameters
    ///
    /// - `operator`: The unary operator to apply.
    /// - `operand`: The operand on which the operator is applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::{Rule, Operator};
    ///
    /// let rule = Rule::var("is_active").not();
    /// ```
    fn unary(operator: Operator, operand: Self) -> Self {
        Rule::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    /// Creates a binary operation.
    ///
    /// This method constructs a `Rule::Binary` representing a binary operation such as equality.
    ///
    /// # Parameters
    ///
    /// - `operator`: The binary operator to apply.
    /// - `left`: The left operand.
    /// - `right`: The right operand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::{Rule, Operator};
    ///
    /// let rule = Rule::var("age").gt(Rule::value(18));
    /// ```
    fn binary(operator: Operator, left: Self, right: Self) -> Self {
        Rule::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Creates a variadic operation.
    ///
    /// This method constructs a `Rule::Variadic` representing operations that can take multiple operands,
    /// such as logical AND or OR.
    ///
    /// # Parameters
    ///
    /// - `operator`: The variadic operator to apply.
    /// - `operands`: A vector of operands for the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::{Rule, Operator};
    ///
    /// let rule = Rule::var("is_admin").and(Rule::var("is_owner")).and(Rule::var("is_active"));
    /// ```
    fn variadic(operator: Operator, operands: Vec<Self>) -> Self {
        Rule::Variadic { operator, operands }
    }

    /// Creates a custom operation.
    ///
    /// This method constructs a `Rule::Custom` allowing for the use of custom operators beyond the standard set.
    ///
    /// # Parameters
    ///
    /// - `operator`: The custom operator name.
    /// - `operands`: A vector of operands for the custom operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::custom("customOp", vec![Rule::var("input"), Rule::value(42)]);
    /// ```
    pub fn custom<S: Into<String>>(operator: S, operands: Vec<Self>) -> Self {
        Rule::Custom {
            operator: operator.into(),
            operands,
        }
    }

    // ----------------------------------------------------------------------------
    // Validation Methods
    //
    // This section provides methods corresponding to specific operators, enabling
    // the construction of logical, comparison, and arithmetic operations. These
    // methods facilitate fluent and intuitive rule building through method chaining.
    // ----------------------------------------------------------------------------
    pub fn validate_boolean_operators(
        operator: Operator,
        operands: Vec<&Rule>,
    ) -> Result<(), RuleError> {
        let invalid_operands = operands
            .iter()
            .filter(|x| !x.is_boolean_compatible())
            .collect::<Vec<_>>();
        if invalid_operands.is_empty() {
            Ok(())
        } else {
            Err(RuleError::TypeError(InvalidType {
                operator,
                operand: invalid_operands[0].to_owned().clone(),
            }))
        }
    }

    pub fn validate_equality_operators(left: &Rule, right: &Rule) -> Result<(), RuleError> {
        if !left.is_type_compatible_with(right) {
            return Err(RuleError::TypeError(IncompatibleType {
                left: left.clone(),
                right: right.clone(),
            }));
        }
        Ok(())
    }

    pub fn validate_numeric_operators(
        operator: Operator,
        operands: Vec<&Rule>,
    ) -> Result<(), RuleError> {
        let invalid_operands = operands
            .iter()
            .filter(|x| !x.is_number_compatible())
            .collect::<Vec<_>>();
        if invalid_operands.is_empty() {
            Ok(())
        } else {
            Err(RuleError::TypeError(InvalidType {
                operator,
                operand: invalid_operands[0].to_owned().clone(),
            }))
        }
    }

    pub fn validate_division(operator: Operator, right: &Rule) -> Result<(), RuleError> {
        // For now we only check zero literal
        // In the future, we can check expressions that evaluate to zero
        if let Rule::Constant(divisor) = right.clone() {
            if !divisor.is_number() {
                return Err(RuleError::TypeError(InvalidType {
                    operator,
                    operand: right.clone(),
                }));
            }
            let divisor = divisor.as_number().unwrap();
            if divisor.is_f64() && divisor.as_f64().unwrap().abs() <= f64::EPSILON {
                Err(RuleError::ValueError(ZeroDivision))
            } else if divisor.is_i64() && divisor.as_i64().unwrap() == 0 {
                Err(RuleError::ValueError(ZeroDivision))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    // ----------------------------------------------------------------------------
    // Operator-Specific Methods
    //
    // This section provides methods corresponding to specific operators, enabling
    // the construction of logical, comparison, and arithmetic operations. These
    // methods facilitate fluent and intuitive rule building through method chaining.
    // ----------------------------------------------------------------------------

    /// Logical NOT operation.
    ///
    /// Applies the logical NOT operator to the current rule.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("is_active").not();
    /// ```
    pub fn not(self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::Not, vec![&self])?;
        Ok(Rule::unary(Operator::Not, self))
    }

    /// Logical AND operation.
    ///
    /// Combines the current rule with another rule using the logical AND operator.
    /// If the current rule is already an AND variadic operation, the other rule is appended
    /// to its operands. Otherwise, a new AND variadic operation is created.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("is_admin").and(Rule::var("is_active"));
    /// ```
    pub fn and(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::And, vec![&self, &other])?;
        match self {
            Rule::Variadic {
                operator: Operator::And,
                mut operands,
            } => {
                operands.push(other);
                Ok(Rule::Variadic {
                    operator: Operator::And,
                    operands,
                })
            }
            _ => Ok(Rule::variadic(Operator::And, vec![self, other])),
        }
    }

    /// Logical OR operation.
    ///
    /// Combines the current rule with another rule using the logical OR operator.
    /// If the current rule is already an OR variadic operation, the other rule is appended
    /// to its operands. Otherwise, a new OR variadic operation is created.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("is_guest").or(Rule::var("is_banned"));
    /// ```
    pub fn or(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_boolean_operators(Operator::Or, vec![&self, &other])?;
        match self {
            Rule::Variadic {
                operator: Operator::Or,
                mut operands,
            } => {
                operands.push(other);
                Ok(Rule::Variadic {
                    operator: Operator::Or,
                    operands,
                })
            }
            _ => Ok(Rule::variadic(Operator::Or, vec![self, other])),
        }
    }

    /// Equality comparison.
    ///
    /// Compares the current rule with another rule for equality using the `==` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("role").eq(Rule::value("admin"));
    /// ```
    pub fn eq(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_equality_operators(&self, &other)?;
        Ok(Rule::binary(Operator::Equal, self, other))
    }

    /// Inequality comparison.
    ///
    /// Compares the current rule with another rule for inequality using the `!=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("status").ne(Rule::value("inactive"));
    /// ```
    pub fn ne(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_equality_operators(&self, &other)?;
        Ok(Rule::binary(Operator::NotEqual, self, other))
    }

    /// Greater-than comparison.
    ///
    /// Compares if the current rule is greater than another rule using the `>` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("score").gt(Rule::value(75));
    /// ```
    pub fn gt(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::GreaterThan, vec![&self, &other])?;
        Ok(Rule::binary(Operator::GreaterThan, self, other))
    }

    /// Less-than comparison.
    ///
    /// Compares if the current rule is less than another rule using the `<` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("age").lt(Rule::value(18));
    /// ```
    pub fn lt(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::LessThan, vec![&self, &other])?;
        Ok(Rule::binary(Operator::LessThan, self, other))
    }

    /// Greater-than-or-equal-to comparison.
    ///
    /// Compares if the current rule is greater than or equal to another rule using the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("experience").gte(Rule::value(5));
    /// ```
    pub fn gte(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::GreaterThanOrEqual, vec![&self, &other])?;
        Ok(Rule::binary(Operator::GreaterThanOrEqual, self, other))
    }

    /// Less-than-or-equal-to comparison.
    ///
    /// Compares if the current rule is less than or equal to another rule using the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("height").lte(Rule::value(180));
    /// ```
    pub fn lte(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::LessThanOrEqual, vec![&self, &other])?;
        Ok(Rule::binary(Operator::LessThanOrEqual, self, other))
    }

    /// Addition operation.
    ///
    /// Adds the current rule with another rule using the `+` operator.
    /// If the current rule is already an ADD variadic operation, the other rule is appended
    /// to its operands. Otherwise, a new ADD variadic operation is created.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("quantity") + Rule::value(10);
    /// ```
    pub fn add(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Add, vec![&self, &other])?;
        match self {
            Rule::Variadic {
                operator: Operator::Add,
                mut operands,
            } => {
                operands.push(other);
                Ok(Rule::Variadic {
                    operator: Operator::Add,
                    operands,
                })
            }
            _ => Ok(Rule::variadic(Operator::Add, vec![self, other])),
        }
    }

    /// Subtraction operation.
    ///
    /// Subtracts another rule from the current rule using the `-` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("total") - Rule::value(20);
    /// ```
    pub fn subtract(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Subtract, vec![&self, &other])?;
        Ok(Rule::binary(Operator::Subtract, self, other))
    }

    /// Multiplication operation.
    ///
    /// Multiplies the current rule with another rule using the `*` operator.
    /// If the current rule is already a MULTIPLY variadic operation, the other rule is appended
    /// to its operands. Otherwise, a new MULTIPLY variadic operation is created.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("price") * Rule::value(2);
    /// ```
    pub fn multiply(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Multiply, vec![&self, &other])?;
        match self {
            Rule::Variadic {
                operator: Operator::Multiply,
                mut operands,
            } => {
                operands.push(other);
                Ok(Rule::Variadic {
                    operator: Operator::Multiply,
                    operands,
                })
            }
            _ => Ok(Rule::variadic(Operator::Multiply, vec![self, other])),
        }
    }

    /// Division operation.
    ///
    /// Divides the current rule by another rule using the `/` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("total") / Rule::value(4);
    /// ```
    pub fn divide(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Divide, vec![&self, &other])?;
        Self::validate_division(Operator::Divide, &other)?;
        Ok(Rule::binary(Operator::Divide, self, other))
    }

    /// Modulo operation.
    ///
    /// Calculates the remainder of the division of the current rule by another rule using the `%` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("number").modulo(Rule::value(3));
    /// ```
    pub fn modulo(self, other: Self) -> Result<Self, RuleError> {
        Self::validate_numeric_operators(Operator::Modulo, vec![&self, &other])?;
        Self::validate_division(Operator::Modulo, &other)?;
        Ok(Rule::binary(Operator::Modulo, self, other))
    }
}

impl Serialize for Rule {
    // Serialization enables the conversion of complex rule structures into a
    // JSON-compatible format for evaluation or transmission.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Rule::Constant(value) => value.serialize(serializer),
            Rule::Variable(name) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("var", name)?;
                map.end()
            }
            Rule::Unary { operator, operand } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), operand)?;
                map.end()
            }
            Rule::Binary {
                operator,
                left,
                right,
            } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), &[left.as_ref(), right.as_ref()])?;
                map.end()
            }
            Rule::Variadic { operator, operands } => {
                let operands_refs: Vec<&Rule> = operands.iter().collect();
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), &operands_refs)?;
                map.end()
            }
            Rule::Custom { operator, operands } => {
                let operands_refs: Vec<&Rule> = operands.iter().collect();
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(operator, &operands_refs)?;
                map.end()
            }
        }
    }
}

/// Implements the `From` trait to allow easy creation of `Rule` constants from various types.
///
/// This implementation facilitates the conversion from primitive types and `serde_json::Value`
/// directly into `Rule::Constant` variants, enhancing the ergonomics of rule construction.
///
/// # Examples
///
/// ```rust
/// use moss_jsonlogic::rule::Rule;
/// use serde_json::json;
///
/// let string_rule: Rule = "active".into();
/// let number_rule: Rule = 42.into();
/// let bool_rule: Rule = true.into();
/// let json_rule: Rule = json!({"key": "value"}).into();
/// ```
impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        Rule::constant(s)
    }
}

impl From<String> for Rule {
    fn from(s: String) -> Self {
        Rule::constant(s)
    }
}

impl From<i64> for Rule {
    fn from(n: i64) -> Self {
        Rule::constant(n)
    }
}

impl From<f64> for Rule {
    fn from(n: f64) -> Self {
        Rule::constant(n)
    }
}

impl From<bool> for Rule {
    fn from(b: bool) -> Self {
        Rule::constant(b)
    }
}

impl From<Value> for Rule {
    fn from(value: Value) -> Self {
        Rule::constant(value)
    }
}

// ----------------------------------------------------------------------------
// Operator Overloading
//
// This section implements Rust's standard operator traits for the `Rule` struct,
// enabling the use of familiar operators (e.g., `&`, `|`, `+`, `-`, `*`, `/`, `!`)
// to construct complex JSON Logic rules in an intuitive and readable manner.
// ----------------------------------------------------------------------------

impl BitAnd for Rule {
    type Output = Rule;

    /// Enables the use of the `&` operator to perform logical AND operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("is_admin") & Rule::var("is_active");
    /// ```
    fn bitand(self, rhs: Rule) -> Rule {
        self.and(rhs).unwrap()
    }
}

impl BitOr for Rule {
    type Output = Rule;

    /// Enables the use of the `|` operator to perform logical OR operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("is_guest") | Rule::var("is_banned");
    /// ```
    fn bitor(self, rhs: Rule) -> Rule {
        self.or(rhs).unwrap()
    }
}

impl Add for Rule {
    type Output = Rule;

    /// Enables the use of the `+` operator to perform addition operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("quantity") + Rule::value(10);
    /// ```
    fn add(self, rhs: Rule) -> Rule {
        self.add(rhs).unwrap()
    }
}

impl Sub for Rule {
    type Output = Rule;

    /// Enables the use of the `-` operator to perform subtraction operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("total") - Rule::value(20);
    /// ```
    fn sub(self, rhs: Rule) -> Rule {
        self.subtract(rhs).unwrap()
    }
}

impl Mul for Rule {
    type Output = Rule;

    /// Enables the use of the `*` operator to perform multiplication operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("price") * Rule::value(2);
    /// ```
    fn mul(self, rhs: Rule) -> Rule {
        self.multiply(rhs).unwrap()
    }
}

impl Div for Rule {
    type Output = Rule;

    /// Enables the use of the `/` operator to perform division operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = Rule::var("total") / Rule::value(4);
    /// ```
    fn div(self, rhs: Rule) -> Rule {
        self.divide(rhs).unwrap()
    }
}

impl Not for Rule {
    type Output = Rule;

    /// Enables the use of the `!` operator to perform logical NOT operations on rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::rule::Rule;
    ///
    /// let rule = !Rule::var("is_active");
    /// ```
    fn not(self) -> Rule {
        self.not().unwrap()
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use moss_jsonlogic_macro::rule;
    use serde_json::json;

    /// Tests arithmetic operations and their serialization.
    ///
    /// Constructs a rule that checks if the sum of `x` and `y` is greater than `10`.
    #[test]
    fn test_arithmetic_operations() {
        let var_x = Rule::var("x");
        let var_y = Rule::var("y");

        let const_ten = Rule::value(10);

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
        let var_status = Rule::var("status");

        let const_active = Rule::from("active");

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
        let var_x = Rule::var("x");
        let var_y = Rule::var("y");
        let var_z = Rule::var("z");

        let const_five = Rule::from(5);
        let const_ten = Rule::from(10);
        let const_twenty = Rule::from(20);

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
        let var_a = Rule::var("a");
        let var_b = Rule::var("b");
        let var_c = Rule::var("c");

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

    /// Tests the combination of logical and arithmetic operations with operator overloading.
    ///
    /// Constructs a rule that checks if `(x * y) + z <= 100`.
    #[test]
    fn test_combining_logical_and_arithmetic_operations() {
        let var_score = Rule::var("score");
        let var_bonus = Rule::var("bonus");

        let const_threshold = Rule::from(100);

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

    /// Tests complex nested rules involving logical NOT and operator overloading.
    ///
    /// Constructs a rule that represents `!(status == "locked" || attempts > 3)`.
    #[test]
    fn test_complex_nested_rules_with_not() {
        let var_status = Rule::var("status");
        let var_attempts = Rule::var("attempts");

        let const_locked = Rule::value("locked");
        let const_three = Rule::value(3);

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

    /// Tests serialization of rules with multiple arithmetic operations.
    ///
    /// Constructs a rule that represents `(a * b) + (c / d) - e`.
    #[test]
    fn test_multiple_arithmetic_operations() {
        let var_a = Rule::var("a");
        let var_b = Rule::var("b");
        let var_c = Rule::var("c");
        let var_d = Rule::var("d");
        let var_e = Rule::var("e");

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

    /// Tests the use of custom operators in rule creation.
    ///
    /// Constructs a rule using a custom operator `"customOp"` with operands `input` and `42`.
    #[test]
    fn test_custom_operator() {
        let var_input = Rule::var("input");

        // Build rule using a custom operator "customOp"
        let custom_rule = Rule::custom("customOp", vec![var_input, Rule::from(42)]);

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

    /// Tests the creation and serialization of a rule with multiple logical and arithmetic operations.
    ///
    /// Constructs a rule that checks if `(x + y) > 10 AND (z <= 5 OR w != 3)`.
    #[test]
    fn test_combined_logical_and_arithmetic_operations() {
        let var_x = Rule::var("x");
        let var_y = Rule::var("y");
        let var_z = Rule::var("z");
        let var_w = Rule::var("w");

        let const_ten = Rule::value(10);
        let const_five = Rule::value(5);
        let const_three = Rule::value(3);

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
        let var_status = Rule::var("status");
        let var_attempts = Rule::var("attempts");

        let const_locked = Rule::from("locked");
        let const_max_attempts = Rule::from(3);

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

    /// Tests logical operations combined with operator overloading.
    ///
    /// Constructs a rule that checks if `a == 5` AND (`b > 10` OR `c < 20`).
    #[test]
    fn test_logical_operations() {
        let var_a = Rule::var("a");
        let var_b = Rule::var("b");
        let var_c = Rule::var("c");

        let rule1 = var_a.eq(Rule::from(5)).unwrap();
        let rule2 = var_b.gt(Rule::from(10)).unwrap();
        let rule3 = var_c.lt(Rule::from(20)).unwrap();

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

    /// Tests the creation and serialization of a complex rule using the desired API.
    ///
    /// The rule constructed is:
    /// ```
    /// view == 'recents.view.id' && viewItem == 'recents.item'
    /// ```
    ///
    /// # Assertions
    ///
    /// - Verifies that the serialized JSON Logic matches the expected structure.
    #[test]
    fn test_rule_with_desired_api() {
        let rule = Rule::var("view")
            .eq(Rule::value("recents.view.id"))
            .unwrap()
            .and(
                Rule::var("viewItem")
                    .eq(Rule::value("recents.item"))
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

    /// Tests the use of operator overloading (`&`, `|`, `+`, `-`, `*`, `/`, `!`) in rule creation.
    ///
    /// This test covers logical AND, OR, NOT operations, as well as arithmetic operations.
    #[test]
    fn test_operator_overloading() {
        // Logical AND
        let rule_and = Rule::var("is_admin") & Rule::var("is_active");
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
        let rule_or = Rule::var("is_guest") | Rule::var("is_banned");
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
        let rule_not = !Rule::var("is_active");
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
        let rule_add = Rule::var("quantity") + Rule::value(10);
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
        let rule_sub = Rule::var("total") - Rule::value(20);
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
        let rule_mul = Rule::var("price") * Rule::value(2);
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
        let rule_div = Rule::var("total") / Rule::value(4);
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

    /// Tests method chaining capabilities in rule creation.
    ///
    /// Constructs a rule that checks if `age >= 18` and `status == 'active'`.
    #[test]
    fn test_method_chaining() {
        let rule = Rule::var("age")
            .gte(Rule::value(18))
            .unwrap()
            .and(Rule::var("status").eq(Rule::value("active")).unwrap())
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
        let rule = Rule::var("status")
            .ne(Rule::value("inactive"))
            .unwrap()
            .and(Rule::var("age").gte(Rule::value(18)).unwrap())
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
        let rule = rule!(!flag);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON"),
            json!({"!": {"var": "flag"}})
        );
    }

    #[test]
    fn test_rule_macro_simple_binary() {
        let rule = rule!(age > 18);
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON."),
            json!({ ">": [{ "var": "age" }, 18] })
        );
    }
    // TODO: Flatten variadic operators
    // +, -, && and ||
    // (Without flattening)
    // 1 + 2 + 3 => {
    //     "+": [
    //         {"+": [1, 2]},
    //         3
    //     ]
    // }
    // (With flattening)
    // 1 + 2 + 3 => {
    //     "+": [1, 2, 3]

    #[test]
    fn test_rule_macro_simple_variadic() {
        let rule = rule!(a + b + c);
        println!("{}", serde_json::to_string_pretty(&rule).unwrap());
        assert_eq!(
            serde_json::to_value(rule).expect("Failed to serialize the rule into JSON"),
            json!({"+" : [{"var" : "a"}, {"var" : "b"}, {"var" : "c"}]})
        )
    }

    #[test]
    fn test_rule_macro_logical_and() {
        let rule = rule!(age > 18 && status == "active");
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
        let rule = rule!((age > 18 && status == "active") || is_admin);
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
        let rule = rule!(number % 2 == 0);
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

    /// --------------------
    /// Validation test
    /// --------------------

    #[test]
    #[should_panic]
    fn test_invalid_type_not() {
        let rule = rule!(!"1");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_and() {
        let rule = rule!(1 && true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_or() {
        let rule = rule!("1" || true);
    }

    #[test]
    #[should_panic]
    fn test_incompatible_type_eq() {
        let rule = rule!(1 == "1");
    }

    #[test]
    #[should_panic]
    fn test_incompatible_type_ne() {
        let rule = rule!(true != "false");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_gt() {
        let rule = rule!("42" > 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_lt() {
        let rule = rule!(false < true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_gte() {
        let rule = rule!("42" >= 42);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_lte() {
        let rule = rule!(42 <= "42");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_add() {
        let rule = rule!(true + "true");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_subtract() {
        let rule = rule!("1" - 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_multiply() {
        let rule = rule!("1" * 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_divide() {
        let rule = rule!("foo" / "bar");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_modulo() {
        let rule = rule!("3" % "2");
    }
    #[test]
    #[should_panic]
    fn test_zero_division_divide() {
        let rule = rule!(42 / 0);
    }

    #[test]
    #[should_panic]
    fn test_zero_division_modulo() {
        let rule = rule!(42 % 0.0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound1() {
        let rule = rule!(1 + 2 / "3");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound2() {
        let rule = rule!(x - true * false);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound3() {
        let rule = rule!(x && !"true");
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound4() {
        let rule = rule!(42 > 0 < 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_type_compound5() {
        let rule = rule!(3.14 < 159 - "26");
    }
}
