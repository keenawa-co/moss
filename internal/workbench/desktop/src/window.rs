use once_cell::sync::OnceCell;
use std::{path::PathBuf, rc::Rc};
use sysinfo::System;
use tauri::AppHandle;
use typography::Font;

/// A color in the `sRGB` color space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    /// Red, 0.0 - 1.0
    pub r: f32,
    /// Green, 0.0 - 1.0
    pub g: f32,
    /// Blue, 0.0 - 1.0
    pub b: f32,
    /// Alpha, 0.0 - 1.0
    pub a: f32,
}

impl Color {
    #[rustfmt::skip]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        debug_assert!(
            (0.0..=1.0).contains(&r),
            "Red component must be on [0, 1]"
        );
        debug_assert!(
            (0.0..=1.0).contains(&g),
            "Green component must be on [0, 1]"
        );
        debug_assert!(
            (0.0..=1.0).contains(&b),
            "Blue component must be on [0, 1]"
        );
        debug_assert!(
            (0.0..=1.0).contains(&a),
            "Alpha component must be on [0, 1]"
        );

        Color { r, g, b, a }
    }
}

pub mod typography {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
    pub struct Font {
        family: Family,
        weight: Weight,
        style: Style,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
    pub enum Family {
        Name(&'static str),

        #[serde(rename = "sans")]
        #[default]
        Sans,
        #[serde(rename = "serif")]
        Serif,
        #[serde(rename = "mono")]
        Mono,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
    pub enum Weight {
        #[serde(rename = "thin")]
        Thin,
        #[serde(rename = "earthlight")]
        ExtraLight,
        #[serde(rename = "light")]
        Light,
        #[serde(rename = "normal")]
        #[default]
        Normal,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "semibold")]
        Semibold,
        #[serde(rename = "bold")]
        Bold,
        #[serde(rename = "extrabold")]
        ExtraBold,
        #[serde(rename = "black")]
        Black,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
    pub enum Style {
        #[serde(rename = "not-italic")]
        #[default]
        NotItalic,
        #[serde(rename = "italic")]
        Italic,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Pixels(pub f32);

impl Pixels {
    /// Zero pixels.
    pub const ZERO: Self = Self(0.0);
}

impl From<f32> for Pixels {
    fn from(amount: f32) -> Self {
        Self(amount)
    }
}

impl From<u16> for Pixels {
    fn from(amount: u16) -> Self {
        Self(f32::from(amount))
    }
}

impl From<Pixels> for f32 {
    fn from(pixels: Pixels) -> Self {
        pixels.0
    }
}

impl std::ops::Add for Pixels {
    type Output = Pixels;

    fn add(self, rhs: Self) -> Self {
        Pixels(self.0 + rhs.0)
    }
}

impl std::ops::Add<f32> for Pixels {
    type Output = Pixels;

    fn add(self, rhs: f32) -> Self {
        Pixels(self.0 + rhs)
    }
}

impl std::ops::Mul for Pixels {
    type Output = Pixels;

    fn mul(self, rhs: Self) -> Self {
        Pixels(self.0 * rhs.0)
    }
}

impl std::ops::Mul<f32> for Pixels {
    type Output = Pixels;

    fn mul(self, rhs: f32) -> Self {
        Pixels(self.0 * rhs)
    }
}

impl std::ops::Div for Pixels {
    type Output = Pixels;

    fn div(self, rhs: Self) -> Self {
        Pixels(self.0 / rhs.0)
    }
}

impl std::ops::Div<f32> for Pixels {
    type Output = Pixels;

    fn div(self, rhs: f32) -> Self {
        Pixels(self.0 / rhs)
    }
}

pub struct Appearance {
    pub theme_slug: String,
    pub primary_color: Color,
    pub bar_color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WorkbenchMode {
    #[default]
    Empty,
    Workspace,
}

pub enum WindowMode {
    Maximized,
    Normal,
    Fullscreen,
}

pub struct WindowState {
    pub appearance: Appearance,
    pub mode: WindowMode,
    pub zoom_level: f32,
}

pub struct Window {
    pub raw: OnceCell<Rc<AppHandle>>,
    pub state: WindowState,
    pub mode: WindowMode,
    // pub platform:
}

#[derive(Debug, Clone, Serialize)]
pub struct NativePlatformInfo {
    pub os: String,
    pub version: String,
    pub hostname: String,
}

impl NativePlatformInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: System::name().unwrap_or_else(|| "unknown".to_string()),
            version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct NativeWindowConfiguration {
    pub home_dir: PathBuf,
    pub full_screen: bool,
    pub platform_info: NativePlatformInfo,
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {}
}
