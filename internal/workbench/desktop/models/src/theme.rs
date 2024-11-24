pub mod theming {
    use jsonschema::Validator;
    use serde_json;
    use serde_json::Value;
    use std::fs;

    const THEME_SCHEMA_PATH: &str =
        r"C:\git\moss\internal\workbench\desktop\models\schema\ThemeSchema.json";
    const TEST_THEME_PATH: &str = r"C:\git\moss\internal\workbench\desktop\models\schema\test.json";
    pub fn create_validator() -> Validator {
        let file = fs::File::open(THEME_SCHEMA_PATH).unwrap();
        let schema_json: Value = serde_json::from_reader(file).unwrap();

        jsonschema::options()
            .with_format("color", validate_color) // A custom format specified in JSON schema
            .should_validate_formats(true)
            .build(&schema_json)
            .unwrap()
    }

    fn validate_color(value: &str) -> bool {
        csscolorparser::parse(value).is_ok()
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
        #[test]
        fn test_parsing_correctly_formatted_rgba() {
            let value = "rgba(1, 2, 3, 1)";
            assert!(validate_color(value))
        }

        #[test]
        fn test_parsing_correctly_formatted_rgb() {
            let value = "rgb(1, 2, 3)";
            assert!(validate_color(value))
        }

        #[test]
        fn test_parsing_correctly_formatted_hex() {
            let value = "#010203";
            assert!(validate_color(value))
        }
        #[test]
        fn test_parsing_correctly_formatted_hsl() {
            let value = "hsl(0, 100%, 50%)";
            assert!(validate_color(value))
        }

        #[test]
        fn test_parsing_without_spaces() {
            let value = "rgba(1,2,3,1)";
            assert!(validate_color(value))
        }

        #[test]
        fn test_parsing_empty_color() {
            let value = "";
            assert!(!validate_color(value))
        }

        #[test]
        fn test_parsing_invalid_rgba() {
            let value = "rgba(x, 0, 0, 1)";
            assert!(!validate_color(value))
        }

        #[test]
        fn test_parsing_invalid_hex() {
            let value = "#gggggg";
            assert!(!validate_color(value))
        }
    }
}
