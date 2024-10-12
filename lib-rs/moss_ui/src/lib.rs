pub mod toolbar_part;

use specta::Type;
use specta_typescript::export;

#[derive(Type)]
pub struct Something {
    a: String,
}
