pub mod theming {
    use indexmap::IndexMap;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    /// Represents the type of a theme, either light or dark.
    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub enum ThemeType {
        /// Light theme type.
        #[serde(rename = "light")]
        Light,
        /// Dark theme type.
        #[serde(rename = "dark")]
        Dark,
    }

    /// Descriptor for a theme, containing metadata such as ID, name, and source.
    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export, export_to = "appearance.ts")]
    pub struct ThemeDescriptor {
        /// Unique identifier of the theme.
        pub id: String,
        /// Display name of the theme.
        pub name: String,
        /// Source of the theme (e.g., file path or URL).
        pub source: String,
    }

    /// Represents a color position in a gradient.
    #[derive(Debug, Deserialize, Serialize, PartialEq, Clone, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub struct ColorPosition {
        /// Hex or RGB(A) color value.
        pub color: String,
        /// Position of the color in the gradient, as a value between 0.0 and 1.0.
        pub position: f64,
    }

    /// Represents a color value, which can either be a solid color or a gradient.
    #[derive(Deserialize, Serialize, Debug, Clone, TS)]
    #[ts(export, export_to = "appearance.ts")]
    #[serde(untagged)]
    pub enum ColorValue {
        /// A single solid color value.
        Solid(String),
        /// A gradient represented by a list of color positions.
        Gradient(Vec<ColorPosition>),
    }

    /// Represents the type of a color, either solid or gradient.
    #[derive(Deserialize, Serialize, Debug, Clone, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub enum ColorType {
        /// A solid color type.
        #[serde(rename = "solid")]
        Solid,
        /// A gradient color type.
        #[serde(rename = "gradient")]
        Gradient,
    }

    /// Detailed information about a color, including its type, optional gradient direction, and value.
    #[derive(Deserialize, Serialize, Debug, Clone, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub struct ColorDetail {
        /// Type of the color (solid or gradient).
        #[serde(rename = "type")]
        pub color_type: ColorType,
        /// Direction for gradients (e.g., "to right"). Optional for solid colors.
        #[ts(optional)]
        pub direction: Option<String>,
        /// The color value, either solid or gradient.
        pub value: ColorValue,
    }

    /// Represents a theme with properties such as name, type, default status, and color tokens.
    #[derive(Deserialize, Serialize, Debug, Clone, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export, export_to = "appearance.ts")]
    pub struct Theme {
        /// Display name of the theme.
        pub name: String,
        /// Slug identifier for the theme, used in file paths or URLs.
        pub slug: String,
        /// Type of the theme (light or dark).
        #[serde(rename = "type")]
        pub theme_type: ThemeType,
        /// Indicates if this is the default theme.
        pub is_default: bool,
        /// A collection of color tokens used by the theme.
        pub colors: IndexMap<String, ColorDetail>,
    }
}
