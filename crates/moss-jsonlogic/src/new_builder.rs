pub(crate) mod rule {

    use serde::Serialize;
    use serde_json::{json, Value};
    use std::fmt::{Display, Pointer};

    trait Operator: Display {}

    #[derive(Clone, Debug, Serialize)]
    enum ComparisonOps {
        Equal,
        StrictEqual,
        NotEqual,
        StrictNotEqual,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
    }

    impl Display for ComparisonOps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ComparisonOps::Equal => write!(f, "=="),
                ComparisonOps::StrictEqual => write!(f, "==="),
                ComparisonOps::NotEqual => write!(f, "!="),
                ComparisonOps::StrictNotEqual => write!(f, "!=="),
                ComparisonOps::LessThan => write!(f, "<"),
                ComparisonOps::LessThanOrEqual => write!(f, "<="),
                ComparisonOps::GreaterThan => write!(f, ">"),
                ComparisonOps::GreaterThanOrEqual => write!(f, ">="),
            }
        }
    }
    impl Operator for ComparisonOps {}

    #[derive(Clone, Debug, Serialize)]
    enum BooleanOps {
        And,
        Or,
        Not,
    }

    impl Display for BooleanOps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BooleanOps::And => write!(f, "and"),
                BooleanOps::Or => write!(f, "or"),
                BooleanOps::Not => write!(f, "not"),
            }
        }
    }

    impl Operator for BooleanOps {}

    #[derive(Clone, Debug, Serialize)]
    #[serde(transparent)]
    pub struct Expr {
        pub value: Value,
    }

    impl Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<T> From<T> for Expr
    where
        T: Into<Value>,
    {
        fn from(value: T) -> Self {
            Expr {
                value: value.into(),
            }
        }
    }

    pub fn var(name: &str) -> Expr {
        Expr {
            value: json!({"var": name}),
        }
    }

    #[derive(Clone, Debug, Serialize)]
    #[serde(transparent)]
    pub struct Rule {
        pub expr: Expr,
    }

    impl Display for Rule {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", format!("{:?}", self.expr))
        }
    }

    impl Rule {
        fn new(operands: Vec<Expr>, operator: impl Operator) -> Self {
            let op = operator.to_string();
            Rule {
                expr: json!({op: operands}).into(),
            }
        }
        pub fn equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(vec![left.into(), right.into()], ComparisonOps::Equal)
        }

        pub fn strict_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(vec![left.into(), right.into()], ComparisonOps::StrictEqual)
        }

        pub fn not_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(vec![left.into(), right.into()], ComparisonOps::NotEqual)
        }

        pub fn strict_not_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(
                vec![left.into(), right.into()],
                ComparisonOps::StrictNotEqual,
            )
        }

        pub fn greater_than(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(vec![left.into(), right.into()], ComparisonOps::GreaterThan)
        }

        pub fn greater_than_or_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(
                vec![left.into(), right.into()],
                ComparisonOps::GreaterThanOrEqual,
            )
        }

        pub fn less_than(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(vec![left.into(), right.into()], ComparisonOps::LessThan)
        }

        pub fn less_than_or_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            Self::new(
                vec![left.into(), right.into()],
                ComparisonOps::LessThanOrEqual,
            )
        }

        pub fn not(self) -> Self {
            Self::new(vec![self.expr], BooleanOps::Not)
        }

        pub fn get_value(&self) -> Value {
            self.expr.value.clone()
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct RuleSet {
        rules: Vec<Rule>,
        relation: BooleanOps,
    }

    impl Into<Rule> for RuleSet {
        fn into(self) -> Rule {
            Rule::new(
                self.rules
                    .into_iter()
                    .map(|rule| rule.expr)
                    .collect::<Vec<Expr>>(),
                self.relation,
            )
        }
    }

    impl RuleSet {
        pub fn new() -> Self {
            RuleSet {
                rules: vec![],
                relation: BooleanOps::And,
            }
        }

        pub fn and(self) -> Self {
            RuleSet {
                relation: BooleanOps::And,
                ..self
            }
        }

        pub fn or(self) -> Self {
            RuleSet {
                relation: BooleanOps::Or,
                ..self
            }
        }

        pub fn add_rule(self, rule: impl Into<Rule>) -> Self {
            let mut rules = self.rules;
            rules.push(rule.into());
            RuleSet { rules, ..self }
        }

        pub fn get_value(&self) -> Value {
            json!({&self.relation.to_string(): &self.rules})
        }
    }
}
