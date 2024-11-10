mod builder;

use builder::RuleBuilder;

fn main() {
    let age_rule = RuleBuilder::new().less_than("age", 40);
    let status_rule = RuleBuilder::new().equal("status", "active");

    // Combine them with `and`
    let combined_rule = RuleBuilder::new()
        .and(age_rule)
        .and(status_rule)
        .build();
    
    match combined_rule {
        Ok(rule) => println!("Combined rule: {}", rule.value),
        Err(e) => println!("Failed to create rule: {}", e),
    }
}