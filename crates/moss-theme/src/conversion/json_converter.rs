use anyhow::{Context as _, Result};

use crate::models::theme::Theme;

use super::{util::convert_colors_to_css_variables, ThemeConverter, Validator};

const COLOR_VARIABLE_PREFIX: &str = "moss-color";

pub struct JsonThemeConverter<V: Validator> {
    validator: V,
}

impl<V: Validator> JsonThemeConverter<V> {
    pub fn new(validator: V) -> Self {
        Self { validator }
    }
}

impl<V: Validator> ThemeConverter for JsonThemeConverter<V> {
    fn convert_to_css(&self, content: String) -> Result<String> {
        let theme_value = serde_json::from_str(&content)
            .context("Failed to deserialize JSON content into Value.")?;

        self.validator.validate(&theme_value)?;

        let theme: Theme = serde_json::from_value(theme_value)
            .context("Failed to deserialize JSON content into Theme structure.")?;

        if theme.colors.is_empty() {
            return Ok(String::from(":root {}\n"));
        }

        let color_vars = convert_colors_to_css_variables(COLOR_VARIABLE_PREFIX, &theme.colors);

        let mut css_content = String::with_capacity(color_vars.len() * 50 + 10); // Оценка емкости
        css_content.push_str(":root {\n");
        for (var, val) in &color_vars {
            css_content.push_str(&format!("  {}: {};\n", var, val));
        }
        css_content.push_str("}\n");

        Ok(css_content)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use indexmap::IndexMap;

    use crate::models::theme::{ColorDetail, ColorPosition, ColorType, ColorValue, ThemeType};

    use super::*;

    struct MockThemeValidator {
        theme_is_valid: bool,
    }

    impl Validator for MockThemeValidator {
        fn validate(&self, _theme_value: &serde_json::Value) -> Result<()> {
            if self.theme_is_valid {
                return Ok(());
            } else {
                return Err(anyhow!("Schema validation failed"));
            }
        }
    }

    #[test]
    fn test_missing_color_type() -> Result<()> {
        let json = r#"
    {
        "name": "Incomplete Theme",
        "slug": "incomplete-theme",
        "type": "light",
        "colors": {
            "primary": {
                "value": "rgba(0,0,0,1)"
            }
        }
    }
    "#;

        let validator = MockThemeValidator {
            theme_is_valid: true,
        };
        let converter = JsonThemeConverter::new(validator);
        let result = converter.convert_to_css(json.to_string());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_convert_theme_to_css_single_and_gradient() -> Result<()> {
        let mut color_map = IndexMap::new();

        color_map.insert(
            "primary".to_string(),
            ColorDetail {
                color_type: ColorType::Solid,
                direction: None,
                value: ColorValue::Solid("rgba(0, 0, 0, 1)".to_string()),
            },
        );

        color_map.insert(
            "toolBar.background".to_string(),
            ColorDetail {
                color_type: ColorType::Gradient,
                direction: Some("to right".to_string()),
                value: ColorValue::Gradient(vec![
                    ColorPosition {
                        color: "red".to_string(),
                        position: 0.0,
                    },
                    ColorPosition {
                        color: "orange".to_string(),
                        position: 0.18,
                    },
                    ColorPosition {
                        color: "yellow".to_string(),
                        position: 0.33,
                    },
                    ColorPosition {
                        color: "rgba(244, 244, 245, 1)".to_string(),
                        position: 0.5,
                    },
                    ColorPosition {
                        color: "blue".to_string(),
                        position: 0.68,
                    },
                    ColorPosition {
                        color: "indigo".to_string(),
                        position: 0.83,
                    },
                    ColorPosition {
                        color: "violet".to_string(),
                        position: 1.0,
                    },
                ]),
            },
        );

        let theme = Theme {
            name: "Test Theme".to_string(),
            slug: "test-theme".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            colors: color_map,
        };

        let validator = MockThemeValidator {
            theme_is_valid: true,
        };
        let converter = JsonThemeConverter::new(validator);
        let css = converter.convert_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {
  --moss-color-primary: rgba(0, 0, 0, 1);
  --moss-color-toolBar-background: linear-gradient(to right, red 0%, orange 18%, yellow 33%, rgba(244, 244, 245, 1) 50%, blue 68%, indigo 83%, violet 100%);
}
";

        assert_eq!(css, expected_css);

        Ok(())
    }

    #[test]
    fn test_convert_theme_to_css_gradient_without_direction() -> Result<()> {
        let mut color_map = IndexMap::new();

        color_map.insert(
            "background.gradient".to_string(),
            ColorDetail {
                color_type: ColorType::Gradient,
                direction: None,
                value: ColorValue::Gradient(vec![
                    ColorPosition {
                        color: "green".to_string(),
                        position: 0.0,
                    },
                    ColorPosition {
                        color: "blue".to_string(),
                        position: 1.0,
                    },
                ]),
            },
        );

        let theme = Theme {
            name: "Gradient Test".to_string(),
            slug: "gradient-test".to_string(),
            theme_type: ThemeType::Dark,
            is_default: false,
            colors: color_map,
        };

        let validator = MockThemeValidator {
            theme_is_valid: true,
        };
        let converter = JsonThemeConverter::new(validator);
        let css = converter.convert_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {
  --color-background-gradient: linear-gradient(to right, green 0%, blue 100%);
}
";

        assert_eq!(css, expected_css);

        Ok(())
    }

    #[test]
    fn test_convert_theme_to_css_solid_only() -> Result<()> {
        let mut color_map = IndexMap::new();

        color_map.insert(
            "sidebar.text".to_string(),
            ColorDetail {
                color_type: ColorType::Solid,
                direction: None,
                value: ColorValue::Solid("rgba(200, 200, 200, 1)".to_string()),
            },
        );

        let theme = Theme {
            name: "Solid Test".to_string(),
            slug: "solid-test".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            colors: color_map,
        };

        let validator = MockThemeValidator {
            theme_is_valid: true,
        };
        let converter = JsonThemeConverter::new(validator);
        let css = converter.convert_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {
  --color-sidebar-text: rgba(200, 200, 200, 1);
}
";

        assert_eq!(css, expected_css);

        Ok(())
    }

    #[test]
    fn test_convert_theme_to_css_empty_color_map() -> Result<()> {
        let color_map = IndexMap::new();

        let theme = Theme {
            name: "Empty Color Theme".to_string(),
            slug: "empty-color-theme".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            colors: color_map,
        };

        let validator = MockThemeValidator {
            theme_is_valid: true,
        };
        let converter = JsonThemeConverter::new(validator);
        let css = converter.convert_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {}
";

        assert_eq!(css, expected_css);

        Ok(())
    }
}
