use moss_env::{lazy_env_parse, lazy_env_parse_or_else};
use once_cell::sync::Lazy;

pub const MAIN_WINDOW_PREFIX: &str = "main_";
pub const OTHER_WINDOW_PREFIX: &str = "other_";

pub const MIN_WINDOW_WIDTH: f64 = 800.0;
pub const MIN_WINDOW_HEIGHT: f64 = 600.0;

pub const DEFAULT_WINDOW_WIDTH: f64 = 1160.0;
pub const DEFAULT_WINDOW_HEIGHT: f64 = 720.0;

pub const RUNTIME_MAX_BLOCKING_THREADS: Lazy<usize> =
    lazy_env_parse!("MOSS_RUNTIME_MAX_BLOCKING_THREADS", usize, 512);

pub const RUNTIME_STACK_SIZE: Lazy<usize> =
    lazy_env_parse_or_else!("MOSS_RUNTIME_STACK_SIZE", usize, |_| {
        // In debug mode, stack frames tend to be larger.
        if cfg!(debug_assertions) {
            20 * 1024 * 1024 // 20MiB in debug mode
        } else {
            10 * 1024 * 1024 // 10MiB in release mode
        }
    });
