pub mod json_converter;

mod util;

use anyhow::Result;

pub trait ThemeConverter {
    fn convert_to_css(&self, content: String) -> Result<String>;
}
