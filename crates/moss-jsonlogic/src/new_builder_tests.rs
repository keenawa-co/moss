#[cfg(test)]

mod tests {
    use crate::new_builder::rule::var;
    use crate::new_builder::rule::{Expr, Rule, RuleSet};
    use serde_json::{json, to_string_pretty};
    #[test]
    fn test_expr_display() {
        println!("{}", Expr::from(1));
        println!("{}", Expr::from("1"));
        println!("{}", var("x"));
    }

    #[test]
    fn test_rule_equal() {
        assert_eq!(
            Rule::equal("1", "2").get_value(),
            json!({ "==": ["1", "2"] })
        );
        assert_eq!(Rule::equal(1, 2).get_value(), json!({ "==": [1, 2] }));
        assert_eq!(
            Rule::equal(true, false).get_value(),
            json!({ "==": [true, false] })
        );
        assert_eq!(Rule::equal("1", 1).get_value(), json!({ "==": ["1", 1] }));
        assert_eq!(
            Rule::equal(var("x"), "42").get_value(),
            json!({ "==": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::equal(var("x"), var("y")).get_value(),
            json!({ "==": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_strict_equal() {
        assert_eq!(
            Rule::strict_equal("1", "2").get_value(),
            json!({ "===": ["1", "2"] })
        );
        assert_eq!(
            Rule::strict_equal(1, 2).get_value(),
            json!({ "===": [1, 2] })
        );
        assert_eq!(
            Rule::strict_equal(true, false).get_value(),
            json!({ "===": [true, false] })
        );
        assert_eq!(
            Rule::strict_equal("1", 1).get_value(),
            json!({ "===": ["1", 1] })
        );
        assert_eq!(
            Rule::strict_equal(var("x"), "42").get_value(),
            json!({ "===": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::strict_equal(var("x"), var("y")).get_value(),
            json!({ "===": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_not_equal() {
        assert_eq!(
            Rule::not_equal("1", "2").get_value(),
            json!({ "!=": ["1", "2"] })
        );
        assert_eq!(Rule::not_equal(1, 2).get_value(), json!({ "!=": [1, 2] }));
        assert_eq!(
            Rule::not_equal(true, false).get_value(),
            json!({ "!=": [true, false] })
        );
        assert_eq!(
            Rule::not_equal("1", 1).get_value(),
            json!({ "!=": ["1", 1] })
        );
        assert_eq!(
            Rule::not_equal(var("x"), "42").get_value(),
            json!({ "!=": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::not_equal(var("x"), var("y")).get_value(),
            json!({ "!=": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_strict_not_equal() {
        assert_eq!(
            Rule::strict_not_equal("1", "2").get_value(),
            json!({ "!==": ["1", "2"] })
        );
        assert_eq!(
            Rule::strict_not_equal(1, 2).get_value(),
            json!({ "!==": [1, 2] })
        );
        assert_eq!(
            Rule::strict_not_equal(true, false).get_value(),
            json!({ "!==": [true, false] })
        );
        assert_eq!(
            Rule::strict_not_equal("1", 1).get_value(),
            json!({ "!==": ["1", 1] })
        );
        assert_eq!(
            Rule::strict_not_equal(var("x"), "42").get_value(),
            json!({ "!==": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::strict_not_equal(var("x"), var("y")).get_value(),
            json!({ "!==": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_greater_than() {
        assert_eq!(
            Rule::greater_than("1", "2").get_value(),
            json!({ ">": ["1", "2"] })
        );
        assert_eq!(Rule::greater_than(1, 2).get_value(), json!({ ">": [1, 2] }));
        assert_eq!(
            Rule::greater_than(true, false).get_value(),
            json!({ ">": [true, false] })
        );
        assert_eq!(
            Rule::greater_than("1", 1).get_value(),
            json!({ ">": ["1", 1] })
        );
        assert_eq!(
            Rule::greater_than(var("x"), "42").get_value(),
            json!({ ">": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::greater_than(var("x"), var("y")).get_value(),
            json!({ ">": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_greater_than_or_equal() {
        assert_eq!(
            Rule::greater_than_or_equal("1", "2").get_value(),
            json!({ ">=": ["1", "2"] })
        );
        assert_eq!(
            crate::new_builder::rule::Rule::greater_than_or_equal(1, 2).get_value(),
            json!({ ">=": [1, 2] })
        );
        assert_eq!(
            Rule::greater_than_or_equal(true, false).get_value(),
            json!({ ">=": [true, false] })
        );
        assert_eq!(
            Rule::greater_than_or_equal("1", 1).get_value(),
            json!({ ">=": ["1", 1] })
        );
        assert_eq!(
            Rule::greater_than_or_equal(var("x"), "42").get_value(),
            json!({ ">=": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::greater_than_or_equal(var("x"), var("y")).get_value(),
            json!({ ">=": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_less_than() {
        assert_eq!(
            Rule::less_than("1", "2").get_value(),
            json!({ "<": ["1", "2"] })
        );
        assert_eq!(Rule::less_than(1, 2).get_value(), json!({ "<": [1, 2] }));
        assert_eq!(
            Rule::less_than(true, false).get_value(),
            json!({ "<": [true, false] })
        );
        assert_eq!(
            Rule::less_than("1", 1).get_value(),
            json!({ "<": ["1", 1] })
        );
        assert_eq!(
            Rule::less_than(var("x"), "42").get_value(),
            json!({ "<": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::less_than(var("x"), var("y")).get_value(),
            json!({ "<": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_less_than_or_equal() {
        assert_eq!(
            Rule::less_than_or_equal("1", "2").get_value(),
            json!({ "<=": ["1", "2"] })
        );
        assert_eq!(
            Rule::less_than_or_equal(1, 2).get_value(),
            json!({ "<=": [1, 2] })
        );
        assert_eq!(
            Rule::less_than_or_equal(true, false).get_value(),
            json!({ "<=": [true, false] })
        );
        assert_eq!(
            Rule::less_than_or_equal("1", 1).get_value(),
            json!({ "<=": ["1", 1] })
        );
        assert_eq!(
            Rule::less_than_or_equal(var("x"), "42").get_value(),
            json!({ "<=": [{"var" : "x"}, "42"] })
        );
        assert_eq!(
            Rule::less_than_or_equal(var("x"), var("y")).get_value(),
            json!({ "<=": [{"var": "x"}, {"var": "y"}] })
        );
    }

    #[test]
    fn test_rule_not() {
        println!("{}", Rule::equal(var("x"), "42").not().get_value());
        assert_eq!(
            Rule::equal(var("x"), "42").not().get_value(),
            json!({
                "not": [{
                    "==": [
                        {"var": "x"},
                        "42"
                    ]
                }]
            })
        )
    }

    #[test]
    fn test_ruleset_and() {
        let rule1 = Rule::equal(var("x"), "42");
        let rule2 = Rule::greater_than(var("y"), "42");
        let rule3 = Rule::not_equal("1", "1").not();
        let ruleset = RuleSet::new()
            .and()
            .add_rule(rule1)
            .add_rule(rule2)
            .add_rule(rule3);

        let json = ruleset.get_value();
        assert!(json.get("and").unwrap().is_array());
        let rules = json["and"].as_array().unwrap();
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn test_ruleset_combined() {
        let sky = Rule::equal(var("sky"), "blue");
        let money = Rule::greater_than(var("money"), 100);
        let time = Rule::greater_than(var("time"), 8);
        let successful = RuleSet::new().and().add_rule(money).add_rule(time);
        let iamhappy = RuleSet::new().or().add_rule(sky).add_rule(successful);

        println!("{}", to_string_pretty(&iamhappy.get_value()).unwrap());
    }
}
