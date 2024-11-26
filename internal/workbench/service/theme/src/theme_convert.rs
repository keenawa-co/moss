use anyhow::{Context as _, Result};
use indexmap::IndexMap;

use crate::theme_model::{ColorDetail, ColorValue, ThemeModel};

pub struct ThemeConvertService;

impl ThemeConvertService {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_theme_to_css(&self, content: String) -> Result<String> {
        let theme: ThemeModel =
            serde_json::from_str(&content).context("JSON deserialization failed")?;

        let mut css_sections = Vec::new();

        if !theme.color.is_empty() {
            let color_vars = self.convert_colors_to_css_variables("color", &theme.color);
            let mut color_css = String::with_capacity(color_vars.len() * 40); // Estimate capacity
            for (var, val) in &color_vars {
                color_css.push_str(&format!("  {}: {};\n", var, val));
            }
            css_sections.push(color_css);
        }

        let mut result = String::with_capacity(css_sections.len() * 100); // Estimate capacity
        result.push_str(":root {\n");
        for (i, section) in css_sections.iter().enumerate() {
            result.push_str(section);
            if i < css_sections.len() - 1 {
                result.push_str("\n");
            }
        }
        result.push_str("}\n");

        Ok(result)
    }

    fn convert_colors_to_css_variables(
        &self,
        category: &str,
        colors: &IndexMap<String, ColorDetail>,
    ) -> IndexMap<String, String> {
        self.convert_category_to_css_variables(category, colors, |color_detail| match &color_detail
            .value
        {
            ColorValue::Solid(val) => val.clone(),
            ColorValue::Gradient(vals) => {
                let direction = color_detail.direction.as_deref().unwrap_or("to right");

                let gradient = vals
                    .iter()
                    .map(|cp| {
                        let percentage = (cp.position * 100.0).round() as usize;
                        format!("{} {}%", cp.color, percentage)
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("linear-gradient({}, {})", direction, gradient)
            }
        })
    }

    fn convert_category_to_css_variables<F, T>(
        &self,
        category: &str,
        tokens: &IndexMap<String, T>,
        converter: F,
    ) -> IndexMap<String, String>
    where
        F: Fn(&T) -> String,
    {
        let mut css_vars = IndexMap::with_capacity(tokens.len());

        for (key, token) in tokens {
            let css_var_name = format!("--{}-{}", category, key.replace('.', "-"));
            let css_var_value = converter(token);
            css_vars.insert(css_var_name, css_var_value);
        }

        css_vars
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use desktop_models::appearance::theming::ThemeType;

    use crate::theme_model::ColorPosition;

    use super::*;

    #[test]
    fn test_missing_color_type() -> Result<()> {
        let json = r#"
    {
        "name": "Incomplete Theme",
        "slug": "incomplete-theme",
        "type": "light",
        "isDefault": true,
        "color": {
            "primary": {
                "value": "rgba(0,0,0,1)"
            }
        }
    }
    "#;
        let service = ThemeConvertService::new();
        let result = service.convert_theme_to_css(json.to_string());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_convert_theme_to_css_single_and_gradient() -> Result<()> {
        let mut color_map = IndexMap::new();

        color_map.insert(
            "primary".to_string(),
            ColorDetail {
                color_type: "solid".to_string(),
                direction: None,
                value: ColorValue::Solid("rgba(0, 0, 0, 1)".to_string()),
            },
        );

        color_map.insert(
            "toolBar.background".to_string(),
            ColorDetail {
                color_type: "gradient".to_string(),
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

        let theme = ThemeModel {
            name: "Test Theme".to_string(),
            slug: "test-theme".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            color: color_map,
        };

        let service = ThemeConvertService::new();
        let css = service.convert_theme_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {
  --color-primary: rgba(0, 0, 0, 1);
  --color-toolBar-background: linear-gradient(to right, red 0%, orange 18%, yellow 33%, rgba(244, 244, 245, 1) 50%, blue 68%, indigo 83%, violet 100%);
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
                color_type: "gradient".to_string(),
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

        let theme = ThemeModel {
            name: "Gradient Test".to_string(),
            slug: "gradient-test".to_string(),
            theme_type: ThemeType::Dark,
            is_default: false,
            color: color_map,
        };

        let service = ThemeConvertService::new();
        let css = service.convert_theme_to_css(serde_json::to_string(&theme)?)?;

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
                color_type: "solid".to_string(),
                direction: None,
                value: ColorValue::Solid("rgba(200, 200, 200, 1)".to_string()),
            },
        );

        let theme = ThemeModel {
            name: "Solid Test".to_string(),
            slug: "solid-test".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            color: color_map,
        };

        let service = ThemeConvertService::new();
        let css = service.convert_theme_to_css(serde_json::to_string(&theme)?)?;

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

        let theme = ThemeModel {
            name: "Empty Color Theme".to_string(),
            slug: "empty-color-theme".to_string(),
            theme_type: ThemeType::Light,
            is_default: true,
            color: color_map,
        };

        let service = ThemeConvertService::new();
        let css = service.convert_theme_to_css(serde_json::to_string(&theme)?)?;

        let expected_css = "\
:root {
}
";

        assert_eq!(css, expected_css);

        Ok(())
    }
}
