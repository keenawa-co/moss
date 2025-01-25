use wit_bindgen;

wit_bindgen::generate!({});

use addon::demo::host_functions::{create_folder, get_hash};

struct Addon {}
impl Guest for Addon {
    fn execute() -> () {
        create_folder(&get_hash())
    }
}

export!(Addon);
