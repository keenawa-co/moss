use std::collections::HashMap;

use desktop_models::appearance::theming::ThemeType;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Represents a color position in a gradient.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ColorPosition {
    pub color: String,
    pub position: f64, // Position as a float between 0.0 and 1.0
}

/// Enum to represent either a solid color or a gradient.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ColorValue {
    Solid(String),
    Gradient(Vec<ColorPosition>),
}

/// Enum to represent the type of color value.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ColorValueType {
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "gradient")]
    Gradient,
}

/// Detailed information about a color token.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColorDetail {
    #[serde(rename = "type")]
    pub color_type: String,
    #[serde(default)]
    pub direction: Option<String>, // Direction for gradients, e.g., "to right"
    pub value: ColorValue,
}

/// Represents the entire theme model.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThemeModel {
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub theme_type: ThemeType,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub color: IndexMap<String, ColorDetail>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_theme_deserialization() {
        let json_data = include_str!("../tests/light-theme.test.json");
        let theme: ThemeModel =
            serde_json::from_str(json_data).expect("JSON deserialization failed");

        assert_eq!(theme.name, "Moss Light Test");
        assert_eq!(theme.slug, "moss-light-test");
        assert_eq!(theme.theme_type, ThemeType::Light);
        assert_eq!(theme.is_default, true);
    }
}
