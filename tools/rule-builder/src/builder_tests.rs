#[cfg(test)]
mod tests {
    use crate::builder::RuleBuilder;
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
        let age_rule = RuleBuilder::new().greater_than("age", 18);
        let status_rule = RuleBuilder::new().equal("status", "active");

        let rule = RuleBuilder::new()
            .or(vec![age_rule, status_rule])
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
        let rule = RuleBuilder::new()
            .greater_than("age", 18)
            .and(RuleBuilder::new().equal("status", "active"))
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
        let rule = RuleBuilder::new()
            .not(RuleBuilder::new().equal("status", "inactive"))
            .build()
            .expect("Failed to build rule");

        assert_eq!(
            rule.value,
            json!({
                "!": { "==": [{ "var": "status" }, "inactive"] }
            })
        );
    }
}
