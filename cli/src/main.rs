use crate::cli::cli_handler;

mod cli;

#[tokio::main]
async fn main() {
    cli_handler().await;
}
