use indexmap::IndexMap;

use crate::model::*;

pub(crate) fn convert_colors_to_css_variables(
    category: &str,
    colors: &IndexMap<String, ColorDetail>,
) -> IndexMap<String, String> {
    convert_category_to_css_variables(category, colors, |color_detail| match &color_detail.value {
        ColorValue::Solid(val) => val.clone(),
        ColorValue::Gradient(vals) => {
            let direction = color_detail
                .direction
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("to right");

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

pub(crate) fn convert_category_to_css_variables<F, T>(
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
