use std::collections::HashMap;

use desktop_models::appearance::theming::ThemeType;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct GradientColorEntry {
    color: String,
    position: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ColorValue {
    Solid(String),
    Gradient(Vec<GradientColorEntry>),
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ColorValueType {
    Solid,
    Gradient,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ColorTokenValue {
    r#type: ColorValueType,
    value: ColorValue,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ThemeModel {
    name: String,
    slug: String,
    r#type: ThemeType,
    is_default: bool,
    color: HashMap<String, ColorTokenValue>,
}
