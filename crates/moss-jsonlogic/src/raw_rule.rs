use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Not, Sub};

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
/// use moss_jsonlogic::raw_rule::RawRule;
///
/// let rule = RawRule::var("age");
/// ```
///
/// ### Creating a Constant
///
/// ```rust
/// use moss_jsonlogic::raw_rule::RawRule;
///
/// let rule = RawRule::value(30);
/// ```
///
/// ### Creating Multiple Constants
///
/// ```rust
/// use moss_jsonlogic::raw_rule::RawRule;
///
/// let rule = RawRule::value("hello");
/// let number_rule = RawRule::value(42);
/// let bool_rule = RawRule::value(true);
/// ```
///
/// ### Creating a Binary Operation
///
/// ```rust
/// use moss_jsonlogic::raw_rule::RawRule;
///
/// let rule = RawRule::var("age").gt(RawRule::value(18));
/// ```
///
/// ### Combining Rules with Logical AND
///
/// ```rust
/// use moss_jsonlogic::raw_rule::RawRule;
///
/// let rule = RawRule::var("age")
///     .gt(RawRule::value(18))
///     .and(RawRule::var("status").eq(RawRule::value("active")));
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RawRule {
    /// A constant value (number, string, boolean, null, array, or object).
    Constant(Value),

    /// A variable reference, e.g., {"var": "x"}.
    Variable(String),

    /// A unary operation, e.g., {"!": {...}}.
    Unary {
        /// The operator for the unary operation.
        operator: Operator,
        /// The operand of the unary operation.
        operand: Box<RawRule>,
    },

    /// A binary operation, e.g., {"==": [left, right]}.
    Binary {
        /// The operator for the binary operation.
        operator: Operator,
        /// The left operand of the binary operation.
        left: Box<RawRule>,
        /// The right operand of the binary operation.
        right: Box<RawRule>,
    },

    /// A variadic operation, e.g., {"and": [op1, op2, ...]}.
    Variadic {
        /// The operator for the variadic operation.
        operator: Operator,
        /// The operands of the variadic operation.
        operands: Vec<RawRule>,
    },

    /// A custom operation with arbitrary structure.
    Custom {
        /// The custom operator.
        operator: String,
        /// The operands for the custom operator.
        operands: Vec<RawRule>,
    },
}
impl RawRule {
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
    /// This method allows you to create a `RawRule::Constant` from any type that can be
    /// converted into a `serde_json::Value`. It supports multiple data types, including
    /// numbers, strings, booleans, nulls, arrays, and objects.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::value("hello");
    /// let number_rule = RawRule::value(42);
    /// let bool_rule = RawRule::value(true);
    /// ```
    pub fn value<V: Into<Value>>(value: V) -> Self {
        RawRule::Constant(value.into())
    }

    /// Creates a constant value.
    ///
    /// Alias for `RawRule::value`. This method is provided for convenience and clarity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::constant("world");
    /// ```
    pub fn constant<V: Into<Value>>(value: V) -> Self {
        RawRule::Constant(value.into())
    }

    /// Creates a variable reference.
    ///
    /// This method creates a `RawRule::Variable` which references a variable by its name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("user_age");
    /// ```
    pub fn var<S: Into<String>>(name: S) -> Self {
        RawRule::Variable(name.into())
    }

    /// Creates a unary operation.
    ///
    /// This method constructs a `RawRule::Unary` representing a unary operation such as logical NOT.
    ///
    /// # Parameters
    ///
    /// - `operator`: The unary operator to apply.
    /// - `operand`: The operand on which the operator is applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::{RawRule, Operator};
    ///
    /// let rule = RawRule::var("is_active").not();
    /// ```
    pub(crate) fn unary(operator: Operator, operand: Self) -> Self {
        RawRule::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    /// Creates a binary operation.
    ///
    /// This method constructs a `RawRule::Binary` representing a binary operation such as equality.
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
    /// use moss_jsonlogic::raw_rule::{RawRule, Operator};
    ///
    /// let rule = RawRule::var("age").gt(RawRule::value(18));
    /// ```
    pub(crate) fn binary(operator: Operator, left: Self, right: Self) -> Self {
        RawRule::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Creates a variadic operation.
    ///
    /// This method constructs a `RawRule::Variadic` representing operations that can take multiple operands,
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
    /// use moss_jsonlogic::raw_rule::{RawRule, Operator};
    ///
    /// let rule = RawRule::var("is_admin").and(RawRule::var("is_owner")).and(RawRule::var("is_active"));
    /// ```
    pub(crate) fn variadic(operator: Operator, operands: Vec<Self>) -> Self {
        RawRule::Variadic { operator, operands }
    }

    /// Creates a custom operation.
    ///
    /// This method constructs a `RawRule::Custom` allowing for the use of custom operators beyond the standard set.
    ///
    /// # Parameters
    ///
    /// - `operator`: The custom operator name.
    /// - `operands`: A vector of operands for the custom operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::custom("customOp", vec![RawRule::var("input"), RawRule::value(42)]);
    /// ```
    pub fn custom<S: Into<String>>(operator: S, operands: Vec<Self>) -> Self {
        RawRule::Custom {
            operator: operator.into(),
            operands,
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
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("is_active").not();
    /// ```
    pub fn not(self) -> Self {
        RawRule::unary(Operator::Not, self)
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
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("is_admin").and(RawRule::var("is_active"));
    /// ```
    pub fn and(self, other: Self) -> Self {
        match self {
            RawRule::Variadic {
                operator: Operator::And,
                mut operands,
            } => {
                operands.push(other);
                RawRule::Variadic {
                    operator: Operator::And,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::And, vec![self, other]),
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
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("is_guest").or(RawRule::var("is_banned"));
    /// ```
    pub fn or(self, other: Self) -> Self {
        match self {
            RawRule::Variadic {
                operator: Operator::Or,
                mut operands,
            } => {
                operands.push(other);
                RawRule::Variadic {
                    operator: Operator::Or,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Or, vec![self, other]),
        }
    }

    /// Equality comparison.
    ///
    /// Compares the current rule with another rule for equality using the `==` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("role").eq(RawRule::value("admin"));
    /// ```
    pub fn eq(self, other: Self) -> Self {
        RawRule::binary(Operator::Equal, self, other)
    }

    /// Inequality comparison.
    ///
    /// Compares the current rule with another rule for inequality using the `!=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("status").ne(RawRule::value("inactive"));
    /// ```
    pub fn ne(self, other: Self) -> Self {
        RawRule::binary(Operator::NotEqual, self, other)
    }

    /// Greater-than comparison.
    ///
    /// Compares if the current rule is greater than another rule using the `>` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("score").gt(RawRule::value(75));
    /// ```
    pub fn gt(self, other: Self) -> Self {
        RawRule::binary(Operator::GreaterThan, self, other)
    }

    /// Less-than comparison.
    ///
    /// Compares if the current rule is less than another rule using the `<` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("age").lt(RawRule::value(18));
    /// ```
    pub fn lt(self, other: Self) -> Self {
        RawRule::binary(Operator::LessThan, self, other)
    }

    /// Greater-than-or-equal-to comparison.
    ///
    /// Compares if the current rule is greater than or equal to another rule using the `>=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("experience").gte(RawRule::value(5));
    /// ```
    pub fn gte(self, other: Self) -> Self {
        RawRule::binary(Operator::GreaterThanOrEqual, self, other)
    }

    /// Less-than-or-equal-to comparison.
    ///
    /// Compares if the current rule is less than or equal to another rule using the `<=` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("height").lte(RawRule::value(180));
    /// ```
    pub fn lte(self, other: Self) -> Self {
        RawRule::binary(Operator::LessThanOrEqual, self, other)
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
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("quantity") + RawRule::value(10);
    /// ```
    pub fn add(self, other: Self) -> Self {
        match self {
            RawRule::Variadic {
                operator: Operator::Add,
                mut operands,
            } => {
                operands.push(other);
                RawRule::Variadic {
                    operator: Operator::Add,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Add, vec![self, other]),
        }
    }

    /// Subtraction operation.
    ///
    /// Subtracts another rule from the current rule using the `-` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("total") - RawRule::value(20);
    /// ```
    pub fn subtract(self, other: Self) -> Self {
        RawRule::binary(Operator::Subtract, self, other)
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
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("price") * RawRule::value(2);
    /// ```
    pub fn multiply(self, other: Self) -> Self {
        match self {
            RawRule::Variadic {
                operator: Operator::Multiply,
                mut operands,
            } => {
                operands.push(other);
                RawRule::Variadic {
                    operator: Operator::Multiply,
                    operands,
                }
            }
            _ => RawRule::variadic(Operator::Multiply, vec![self, other]),
        }
    }

    /// Division operation.
    ///
    /// Divides the current rule by another rule using the `/` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("total") / RawRule::value(4);
    /// ```
    pub fn divide(self, other: Self) -> Self {
        RawRule::binary(Operator::Divide, self, other)
    }

    /// Modulo operation.
    ///
    /// Calculates the remainder of the division of the current rule by another rule using the `%` operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("number").modulo(RawRule::value(3));
    /// ```
    pub fn modulo(self, other: Self) -> Self {
        RawRule::binary(Operator::Modulo, self, other)
    }
}

impl Serialize for RawRule {
    // Serialization enables the conversion of complex rule structures into a
    // JSON-compatible format for evaluation or transmission.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RawRule::Constant(value) => value.serialize(serializer),
            RawRule::Variable(name) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("var", name)?;
                map.end()
            }
            RawRule::Unary { operator, operand } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), operand)?;
                map.end()
            }
            RawRule::Binary {
                operator,
                left,
                right,
            } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), &[left.as_ref(), right.as_ref()])?;
                map.end()
            }
            RawRule::Variadic { operator, operands } => {
                let operands_refs: Vec<&RawRule> = operands.iter().collect();
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&operator.to_string(), &operands_refs)?;
                map.end()
            }
            RawRule::Custom { operator, operands } => {
                let operands_refs: Vec<&RawRule> = operands.iter().collect();
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
/// directly into `RawRule::Constant` variants, enhancing the ergonomics of rule construction.
///
/// # Examples
///
/// ```rust
/// use moss_jsonlogic::raw_rule::RawRule;
/// use serde_json::json;
///
/// let string_rule: RawRule = "active".into();
/// let number_rule: RawRule = 42.into();
/// let bool_rule: RawRule = true.into();
/// let json_rule: RawRule = json!({"key": "value"}).into();
/// ```
impl From<&str> for RawRule {
    fn from(s: &str) -> Self {
        RawRule::constant(s)
    }
}

impl From<String> for RawRule {
    fn from(s: String) -> Self {
        RawRule::constant(s)
    }
}

impl From<i64> for RawRule {
    fn from(n: i64) -> Self {
        RawRule::constant(n)
    }
}

impl From<f64> for RawRule {
    fn from(n: f64) -> Self {
        RawRule::constant(n)
    }
}

impl From<bool> for RawRule {
    fn from(b: bool) -> Self {
        RawRule::constant(b)
    }
}

impl From<Value> for RawRule {
    fn from(value: Value) -> Self {
        RawRule::constant(value)
    }
}

// ----------------------------------------------------------------------------
// Operator Overloading
//
// This section implements Rust's standard operator traits for the `Rule` struct,
// enabling the use of familiar operators (e.g., `&`, `|`, `+`, `-`, `*`, `/`, `!`)
// to construct complex JSON Logic rules in an intuitive and readable manner.
// ----------------------------------------------------------------------------

impl BitAnd for RawRule {
    type Output = RawRule;

    /// Enables the use of the `&` operator to perform logical AND operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("is_admin") & RawRule::var("is_active");
    /// ```
    fn bitand(self, rhs: RawRule) -> RawRule {
        self.and(rhs)
    }
}

impl BitOr for RawRule {
    type Output = RawRule;

    /// Enables the use of the `|` operator to perform logical OR operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("is_guest") | RawRule::var("is_banned");
    /// ```
    fn bitor(self, rhs: RawRule) -> RawRule {
        self.or(rhs)
    }
}

impl Add for RawRule {
    type Output = RawRule;

    /// Enables the use of the `+` operator to perform addition operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("quantity") + RawRule::value(10);
    /// ```
    fn add(self, rhs: RawRule) -> RawRule {
        self.add(rhs)
    }
}

impl Sub for RawRule {
    type Output = RawRule;

    /// Enables the use of the `-` operator to perform subtraction operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("total") - RawRule::value(20);
    /// ```
    fn sub(self, rhs: RawRule) -> RawRule {
        self.subtract(rhs)
    }
}

impl Mul for RawRule {
    type Output = RawRule;

    /// Enables the use of the `*` operator to perform multiplication operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("price") * RawRule::value(2);
    /// ```
    fn mul(self, rhs: RawRule) -> RawRule {
        self.multiply(rhs)
    }
}

impl Div for RawRule {
    type Output = RawRule;

    /// Enables the use of the `/` operator to perform division operations between rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = RawRule::var("total") / RawRule::value(4);
    /// ```
    fn div(self, rhs: RawRule) -> RawRule {
        self.divide(rhs)
    }
}

impl Not for RawRule {
    type Output = RawRule;

    /// Enables the use of the `!` operator to perform logical NOT operations on rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moss_jsonlogic::raw_rule::RawRule;
    ///
    /// let rule = !RawRule::var("is_active");
    /// ```
    fn not(self) -> RawRule {
        self.not()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moss_jsonlogic_macro::rule;
    use serde_json::json;

    /// Tests arithmetic operations and their serialization.
    ///
    /// Constructs a rule that checks if the sum of `x` and `y` is greater than `10`.
    #[test]
    fn test_arithmetic_operations() {
        let var_x = RawRule::var("x");
        let var_y = RawRule::var("y");

        let const_ten = RawRule::value(10);

        // Build rule: (x + y) > 10
        let rule = (var_x + var_y).gt(const_ten);

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
        let var_status = RawRule::var("status");

        let const_active = RawRule::from("active");

        // Build rule: !(status == "active")
        let rule = !(var_status.eq(const_active));

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
        let var_x = RawRule::var("x");
        let var_y = RawRule::var("y");
        let var_z = RawRule::var("z");

        let const_five = RawRule::from(5);
        let const_ten = RawRule::from(10);
        let const_twenty = RawRule::from(20);

        // Build rules
        let rule1 = var_x.gt(const_five); // x > 5
        let rule2 = var_y.lt(const_ten); // y < 10
        let rule3 = var_z.eq(const_twenty); // z == 20

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
        let var_a = RawRule::var("a");
        let var_b = RawRule::var("b");
        let var_c = RawRule::var("c");

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
        let var_score = RawRule::var("score");
        let var_bonus = RawRule::var("bonus");

        let const_threshold = RawRule::from(100);

        // Build rule: (score + bonus) >= 100
        let rule = (var_score + var_bonus).gte(const_threshold);

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
        let var_status = RawRule::var("status");
        let var_attempts = RawRule::var("attempts");

        let const_locked = RawRule::value("locked");
        let const_three = RawRule::value(3);

        // Build rule: !(status == "locked" || attempts > 3)
        let rule = !(var_status.eq(const_locked) | var_attempts.gt(const_three));

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
        let var_a = RawRule::var("a");
        let var_b = RawRule::var("b");
        let var_c = RawRule::var("c");
        let var_d = RawRule::var("d");
        let var_e = RawRule::var("e");

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
        let var_input = RawRule::var("input");

        // Build rule using a custom operator "customOp"
        let custom_rule = RawRule::custom("customOp", vec![var_input, RawRule::from(42)]);

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
        let var_x = RawRule::var("x");
        let var_y = RawRule::var("y");
        let var_z = RawRule::var("z");
        let var_w = RawRule::var("w");

        let const_ten = RawRule::value(10);
        let const_five = RawRule::value(5);
        let const_three = RawRule::value(3);

        let rule_sum = var_x + var_y; // x + y
        let rule_gt = rule_sum.gt(const_ten); // (x + y) > 10
        let rule_le = var_z.lte(const_five); // z <= 5
        let rule_ne = var_w.ne(const_three); // w != 3
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
        let var_status = RawRule::var("status");
        let var_attempts = RawRule::var("attempts");

        let const_locked = RawRule::from("locked");
        let const_max_attempts = RawRule::from(3);

        // Build rule: !(status == "locked" || attempts > 3)
        let rule = !(var_status.eq(const_locked) | var_attempts.gt(const_max_attempts));

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
        let var_a = RawRule::var("a");
        let var_b = RawRule::var("b");
        let var_c = RawRule::var("c");

        let rule1 = var_a.eq(RawRule::from(5));
        let rule2 = var_b.gt(RawRule::from(10));
        let rule3 = var_c.lt(RawRule::from(20));

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
        let rule = RawRule::var("view")
            .eq(RawRule::value("recents.view.id"))
            .and(RawRule::var("viewItem").eq(RawRule::value("recents.item")));

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
        let rule_and = RawRule::var("is_admin") & RawRule::var("is_active");
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
        let rule_or = RawRule::var("is_guest") | RawRule::var("is_banned");
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
        let rule_not = !RawRule::var("is_active");
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
        let rule_add = RawRule::var("quantity") + RawRule::value(10);
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
        let rule_sub = RawRule::var("total") - RawRule::value(20);
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
        let rule_mul = RawRule::var("price") * RawRule::value(2);
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
        let rule_div = RawRule::var("total") / RawRule::value(4);
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
        let rule = RawRule::var("age")
            .gte(RawRule::value(18))
            .and(RawRule::var("status").eq(RawRule::value("active")));

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
        let rule = RawRule::var("status")
            .ne(RawRule::value("inactive"))
            .and(RawRule::var("age").gte(RawRule::value(18)));

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

    #[test]
    fn test_rule_macro_with_val() {
        let active_value = "active";
        let rule = rule!((age > 18 && status == val!(active_value)) || is_admin);

        let json_logic = serde_json::to_value(rule).unwrap();

        let expected_json = json!({
            "or": [
                {
                    "and": [
                        { ">": [ { "var": "age" }, 18 ] },
                        { "==": [ { "var": "status" }, "active" ] }
                    ]
                },
                { "var": "is_admin" }
            ]
        });

        assert_eq!(json_logic, expected_json);
    }
}
