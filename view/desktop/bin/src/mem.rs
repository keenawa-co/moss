#[cfg(target_os = "linux")]
#[global_allocator]
pub static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(target_os = "macos")]
#[global_allocator]
pub static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
