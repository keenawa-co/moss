use clap::Args;
use std::net::SocketAddr;

use crate::loader;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "The hostname and port to listen for connections on")]
    #[arg(default_value = "127.0.0.1:1355")]
    bind: SocketAddr,
    #[arg(default_value = "builtin/conf/preference.toml")]
    ubp_path: String,
}

pub async fn init(RunCmdArgs { bind, ubp_path }: RunCmdArgs) -> anyhow::Result<()> {
    let preference = loader::load_preference_file(ubp_path)?;
    let _ = moss_net::CONF.set(moss_net::Config { bind, preference });

    // let inmemdb = prepare_inmemdb(ubp_path).await?;

    moss_net::bind().await.expect("Failed to start the server");
    return Ok(());
}

// async fn prepare_inmemdb(ubp_filepath: String) -> anyhow::Result<SurrealInMem> {
//     let db = Surreal::new::<Mem>(()).await?;
//     db.use_ns(core::constant::APP_NAME)
//         .use_db(server::infra::DEFAULT_INMEM_DB_NAME)
//         .await?;

//     let inmemdb = server::infra::surrealdb::inmem::SurrealInMem::new(db);

//     let data = load_ubp_config(ubp_filepath)?;

//     {
//         for category in data {
//             inmemdb
//                 .user_repo
//                 .create_preference_category(category)
//                 .await?;
//         }
//     }

//     Ok(inmemdb)
// }
