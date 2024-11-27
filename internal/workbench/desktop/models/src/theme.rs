pub mod theming {
    use jsonschema::Validator;
    use serde_json;
    use serde_json::Value;
    use std::fs;

    const THEME_SCHEMA_STR: &str = include_str!("../schema/ThemeSchema.json");
    const TEST_THEME_STR: &str = include_str!("../schema/test.json");

    pub fn create_validator() -> Validator {
        let schema_json: Value = serde_json::from_str(THEME_SCHEMA_STR).unwrap();
        jsonschema::validator_for(&schema_json).unwrap()
    }

    pub fn validate_theme(theme: &Value) -> bool {
        // This function will also validate color values
        let validator = create_validator();
        let result = validator.validate(&theme);
        println!("Validation result {:#?}", result);
        result.is_ok()
    }

    mod tests {
        use super::*;

        #[test]
        fn test() {
            let theme: Value = serde_json::from_str(TEST_THEME_STR).unwrap();
            assert!(validate_theme(&theme));
        }

        // TODO: write more tests for invalid themes
    }
}
