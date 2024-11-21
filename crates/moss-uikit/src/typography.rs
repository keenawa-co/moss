/// Represents a font configuration, including family, weight, and style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "typography.ts")]
pub struct Font {
    /// The font family (e.g., Sans, Serif, Mono, or a custom name).
    family: Family,
    /// The weight of the font (e.g., Thin, Normal, Bold).
    weight: Weight,
    /// The style of the font (e.g., Italic, NotItalic).
    style: Style,
}

/// Represents the family of a font (e.g., Sans, Serif, Mono, or a named family).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
#[ts(export, export_to = "typography.ts")]
pub enum Family {
    /// A named font family.
    Name(&'static str),

    /// Default sans-serif font.
    #[serde(rename = "sans")]
    #[default]
    Sans,

    /// Serif font.
    #[serde(rename = "serif")]
    Serif,

    /// Monospace font.
    #[serde(rename = "mono")]
    Mono,
}

/// Represents the weight of a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
#[ts(export, export_to = "typography.ts")]
pub enum Weight {
    /// Thin font weight.
    #[serde(rename = "thin")]
    Thin,

    /// Extra light font weight.
    #[serde(rename = "extralight")]
    ExtraLight,

    /// Light font weight.
    #[serde(rename = "light")]
    Light,

    /// Normal font weight (default).
    #[serde(rename = "normal")]
    #[default]
    Normal,

    /// Medium font weight.
    #[serde(rename = "medium")]
    Medium,

    /// Semi-bold font weight.
    #[serde(rename = "semibold")]
    Semibold,

    /// Bold font weight.
    #[serde(rename = "bold")]
    Bold,

    /// Extra bold font weight.
    #[serde(rename = "extrabold")]
    ExtraBold,

    /// Black font weight.
    #[serde(rename = "black")]
    Black,
}

/// Represents the style of a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
#[ts(export, export_to = "typography.ts")]
pub enum Style {
    /// Non-italic font style (default).
    #[serde(rename = "not-italic")]
    #[default]
    NotItalic,

    /// Italic font style.
    #[serde(rename = "italic")]
    Italic,
}
