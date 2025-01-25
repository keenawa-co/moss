use wit_bindgen;

wit_bindgen::generate!({
    world: "passing-data",
    path: "./wit/passing-data.wit",
    additional_derives: [PartialEq, Eq]});

use addon::demo::application_models::*;
use addon::demo::host_functions_preferences::{get_preferences, set_preferences};

struct Addon {}

impl Guest for Addon {
    fn execute() -> () {
        let new_pref = Preferences {
            theme: Some(ThemeDescriptor {
                id: "passing-data".to_string(),
                name: "passing-data".to_string(),
                source: "passing-data".to_string(),
            }),
            locale: Some(LocaleDescriptor {
                code: "passing-data".to_string(),
                name: "passing-data".to_string(),
                direction: None,
            }),
        };
        set_preferences(&new_pref);
    }
}

export!(Addon);
