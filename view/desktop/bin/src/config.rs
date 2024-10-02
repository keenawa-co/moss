use once_cell::sync::Lazy;
use platform_core::{lazy_env_parse, lazy_env_parse_or_else};

pub static RUNTIME_MAX_BLOCKING_THREADS: Lazy<usize> =
    lazy_env_parse!("MOSS_RUNTIME_MAX_BLOCKING_THREADS", usize, 512);

pub static RUNTIME_STACK_SIZE: Lazy<usize> =
    lazy_env_parse_or_else!("MOSS_RUNTIME_STACK_SIZE", usize, |_| {
        // In debug mode, stack frames tend to be larger.
        if cfg!(debug_assertions) {
            20 * 1024 * 1024 // 20MiB in debug mode
        } else {
            10 * 1024 * 1024 // 10MiB in release mode
        }
    });
