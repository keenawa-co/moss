use wit_bindgen;

wit_bindgen::generate!({
    world: "wasmhost-demo"
});

struct Addon {}
impl Guest for Addon {
    fn execute() -> () {
        create_folder(&get_hash())
    }
}

export!(Addon);
