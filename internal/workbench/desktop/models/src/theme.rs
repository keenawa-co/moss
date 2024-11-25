pub mod theming {
    use jsonschema::Validator;
    use serde_json;
    use serde_json::Value;
    use std::fs;

    // TODO: Use relative path
    const THEME_SCHEMA_PATH: &str =
        r"C:\git\moss\internal\workbench\desktop\models\schema\ThemeSchema.json";
    const TEST_THEME_PATH: &str = r"C:\git\moss\internal\workbench\desktop\models\schema\test.json";
    pub fn create_validator() -> Validator {
        let file = fs::File::open(THEME_SCHEMA_PATH).unwrap();
        let schema_json: Value = serde_json::from_reader(file).unwrap();
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
            let file = fs::File::open(TEST_THEME_PATH).unwrap();
            let theme: Value = serde_json::from_reader(file).unwrap();
            assert!(validate_theme(&theme));
        }

        // TODO: write more tests for invalid themes
    }
}
