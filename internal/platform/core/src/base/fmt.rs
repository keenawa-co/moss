#[macro_export]
macro_rules! quote {
    ($value:expr) => {
        format!("'{}'", $value)
    };
}
