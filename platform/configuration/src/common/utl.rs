pub(super) fn format_key<T: std::fmt::Display>(key: T) -> String {
    format!("$.{}", key)
}
