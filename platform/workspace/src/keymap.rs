use std::borrow::Cow;

#[cfg(target_os = "macos")]
pub const DEFAULT_KEYMAP_PATH: &str = "keymap/default-macos.json";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_KEYMAP_PATH: &str = "keymap/default-linux.json";

pub fn default_keymap() -> Cow<'static, str> {
    // TODO: use DEFAULT_KEYMAP_PATH and read keymap file
    unimplemented!()
}
