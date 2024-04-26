use clap::Args;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio_util::sync::CancellationToken as TokioCancellationToken;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    /// Specifies the server address on which the server should listen for
    /// incoming connections. This should include both the hostname (or IP
    /// address) and the port number. For example, to listen on all
    /// interfaces on port 8080, you would specify "0.0.0.0:8080". If not
    /// specified, the server will not bind to any address, and you must
    /// configure this elsewhere.
    #[arg(
        long = "bind",
        help = "Specify the hostname and port to listen for connections, e.g., '0.0.0.0:8080'.",
        help_heading = "NETWORK"
    )]
    bind: Option<SocketAddr>,

    /// Path to the server behaver preferences configuration file.
    /// This file should contain settings specific to server behaver
    /// preferences such as timeouts, retries, etc. The default path is
    /// set to 'conf/net_pref.toml', which should be adjusted according
    /// to your deployment.
    #[arg(
        default_value = "conf/net_pref.toml",
        long = "net_pref_path",
        help = "Path to the server preferences configuration file.",
        help_heading = "CONFIGURATION"
    )]
    net_pref_path: PathBuf,

    /// Path to the server configuration file.
    /// This file configures the server parameters such as which server
    /// to connect, port configurations etc. By default, this path is set
    /// to 'conf/net_conf.toml', but it should be adjusted to suit your
    /// environment.
    #[arg(
        default_value = "conf/net_conf.toml",
        long = "net_conf_path",
        help = "Path to the server configuration file.",
        help_heading = "CONFIGURATION"
    )]
    net_conf_path: PathBuf,
}

pub async fn init(
    RunCmdArgs {
        bind,
        net_pref_path: preference_filepath,
        net_conf_path,
    }: RunCmdArgs,
) -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let conf: crate::config::Config = super::utl::load_toml_file(&net_conf_path)?;
    let conn = super::utl::db_connection(&PathBuf::from("./")).await?;

    //  cancel_token is passed to all async functions requiring graceful termination
    let cancel_token = TokioCancellationToken::new();
    let _ = moss_net::CONF.set(moss_net::Config {
        bind: bind.unwrap_or(conf.net.endpoint_addr()),
        preference: super::utl::load_toml_file(&preference_filepath)?,
        conn: Arc::new(conn),
    });

    moss_net::bind(cancel_token).await?;

    Ok(())
}
