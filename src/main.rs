use std::process::ExitCode;

fn main() -> ExitCode {
    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(moss_cli::init());
}
