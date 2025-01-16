use wit_bindgen;

wit_bindgen::generate!({
    world: "open-new-window-shortcut"
});

struct Plugin {}
impl Guest for Plugin {
    fn execute() -> () {
        open_new_window()
    }
}

export!(Plugin);
