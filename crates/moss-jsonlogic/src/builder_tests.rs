#[cfg(test)]
mod tests {
    use crate::builder::{RuleBuilder, LogicRuleBuilder};
    use serde_json::json;

    #[test]
    fn test_equal_operation() {
        let rule = RuleBuilder::new()
            .equal("age", 30)
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({ "==": [{ "var": "age" }, 30] })
        );
    }

    #[test]
    fn test_strict_equal_operation() {
        let rule = RuleBuilder::new()
            .strict_equal("status", "active")
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({ "===": [{ "var": "status" }, "active"] })
        );
    }

    #[test]
    fn test_not_equal_operation() {
        let rule = RuleBuilder::new()
            .not_equal("score", 50)
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({ "!=": [{ "var": "score" }, 50] })
        );
    }

    #[test]
    fn test_greater_than_operation() {
        let rule = RuleBuilder::new()
            .greater_than("height", 150)
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({ ">": [{ "var": "height" }, 150] })
        );
    }

    #[test]
    fn test_less_than_or_equal_operation() {
        let rule = RuleBuilder::new()
            .less_than_or_equal("weight", 80)
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({ "<=": [{ "var": "weight" }, 80] })
        );
    }

    #[test]
    fn test_or_operation() {
        let rule = LogicRuleBuilder::new()
            .child(RuleBuilder::new().greater_than("age", 18))
            .or()
            .child(RuleBuilder::new().equal("status", "active"))
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "or": [
                    { ">": [{ "var": "age" }, 18] },
                    { "==": [{ "var": "status" }, "active"] }
                ]
            })
        );
    }

    #[test]
    fn test_combined_and_operation() {
        let rule = LogicRuleBuilder::new()
            .child(RuleBuilder::new().greater_than("age", 18))
            .and()
            .child(RuleBuilder::new().equal("status", "active"))
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "and": [
                    { ">": [{ "var": "age" }, 18] },
                    { "==": [{ "var": "status" }, "active"] }
                ]
            })
        );
    }

    #[test]
    fn test_not_operation() {
        let rule = LogicRuleBuilder::new()
            .not()
            .child(RuleBuilder::new().equal("status", "inactive"))
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "!": { "==": [{ "var": "status" }, "inactive"] }
            })
        );
    }

    #[test]
    fn test_nested_and_or_operations() {
        let rule = LogicRuleBuilder::new()
            .child(
                LogicRuleBuilder::new()
                    .child(RuleBuilder::new().greater_than("age", 18))
                    .or()
                    .child(RuleBuilder::new().equal("status", "active"))
            )
            .and()
            .child(RuleBuilder::new().less_than("score", 100))
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "and": [
                    {
                        "or": [
                            { ">": [{ "var": "age" }, 18] },
                            { "==": [{ "var": "status" }, "active"] }
                        ]
                    },
                    { "<": [{ "var": "score" }, 100] }
                ]
            })
        );
    }

    #[test]
    fn test_cast_to_boolean_operation() {
        let rule = RuleBuilder::new()
            .cast_to_boolean("verified")
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "!!": { "var": "verified" }
            })
        );
    }

    #[test]
    fn test_complex_condition_with_all_operations() {
        let rule = LogicRuleBuilder::new()
            .child(
                LogicRuleBuilder::new()
                    .child(RuleBuilder::new().greater_than("age", 18))
                    .and()
                    .child(RuleBuilder::new().less_than_or_equal("age", 65))
            )
            .or()
            .child(
                LogicRuleBuilder::new()
                    .not()
                    .child(RuleBuilder::new().equal("status", "retired"))
            )
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "or": [
                    {
                        "and": [
                            { ">": [{ "var": "age" }, 18] },
                            { "<=": [{ "var": "age" }, 65] }
                        ]
                    },
                    {
                        "!": { "==": [{ "var": "status" }, "retired"] }
                    }
                ]
            })
        );
    }
}
