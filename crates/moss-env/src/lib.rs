/// This macro allows for deferred parsing of an environment variable into a specified type.
/// If the variable is missing or the parsing fails, a default value is returned instead.
///
/// # Arguments
///
/// - `$key`: An expression that specifies the name of the environment variable.
/// - `$t`: The target type for parsing the environment variable's value.
/// - `$default`: A default value to fall back on if the environment variable is undefined
///   or if the parsing process encounters an error.
///
/// # Returns
///
/// Produces a `once_cell::sync::Lazy` static variable that contains either the parsed value
/// from the environment variable or the provided default value.
#[macro_export]
macro_rules! lazy_env_parse {
    ($key:expr, $t:ty, $default:expr) => {
        once_cell::sync::Lazy::new(|| {
            std::env::var($key)
                .and_then(|s| Ok(s.parse::<$t>().unwrap_or($default)))
                .unwrap_or($default)
        })
    };
}

/// This macro lazily converts an environment variable into a specified type. If the environment variable is absent
/// or the conversion fails, a provided default value or function is used.
///
/// # Arguments
///
/// - `$key`: A string expression representing the environment variable's name.
/// - `$t`: The data type to convert the environment variable's value into.
/// - `$default`: A function or constant that will be returned if the environment variable is not set or conversion fails.
///
/// # Returns
///
/// Generates a `once_cell::sync::Lazy` static variable that holds either the successfully parsed value or the fallback value.
#[macro_export]
macro_rules! lazy_env_parse_or_else {
    ($key:expr, $t:ty, $default:expr) => {
        once_cell::sync::Lazy::new(|| {
            std::env::var($key)
                .and_then(|s| Ok(s.parse::<$t>().unwrap_or_else($default)))
                .unwrap_or_else($default)
        })
    };
}
