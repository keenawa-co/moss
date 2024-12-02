/// A macro for wrapping a given expression in single quotes.
///
/// This macro takes an expression and returns it as a `String` wrapped in single quotes (`'`).
/// It uses `format!` to convert the expression to a string and add single quotes around it.
///
/// # Parameters
///
/// - `$value`: The expression to be wrapped in single quotes. This can be any expression that
///   implements the `Display` trait (such as strings, integers, etc.).
///
/// # Example
///
/// ```rust
/// use my_crate::quote;
///
/// let name = "Alice";
/// let quoted_name = quote!(name);
/// assert_eq!(quoted_name, "'Alice'");
///
/// let number = 42;
/// let quoted_number = quote!(number);
/// assert_eq!(quoted_number, "'42'");
/// ```
///
/// # Notes
///
/// - The macro returns a `String` with single quotes around the given expression.
/// - Ensure the expression implements `Display`, as `format!` requires this trait.
#[macro_export]
macro_rules! quote {
    ($value:expr) => {
        format!("'{}'", $value)
    };
}
