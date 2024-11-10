mod builder;

use builder::RuleBuilder;

fn main() {
    let rule = RuleBuilder::new()
        .equal("age", 18)
        .equal("score", 90)
        .build()
        .unwrap();
    
    println!("Generated Rule: {}", rule.value);
}