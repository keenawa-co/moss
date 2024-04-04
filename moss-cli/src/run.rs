use clap::Args;
use std::net::SocketAddr;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "The hostname and port to listen for connections on")]
    #[arg(default_value = "127.0.0.1:1355")]
    bind: SocketAddr,
    #[arg(default_value = "builtin/conf/ubp.toml")]
    ubp_path: String,
}

pub async fn init(RunCmdArgs { bind, ubp_path }: RunCmdArgs) -> anyhow::Result<()> {
    let _ = moss_net::CONF.set(moss_net::Config { bind });

    // let inmemdb = prepare_inmemdb(ubp_path).await?;
    let user_settings = crate::loader::load_behaver_preference_file(ubp_path)?;

    moss_net::init(user_settings)
        .await
        .expect("Failed to start the server");
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

// fn load_ubp_config(file_path: String) -> anyhow::Result<Vec<PreferenceCategory>> {
//     let contents = fs::read_to_string(file_path)?;
//     let toml_value: toml::Value = toml::from_str(&contents)?;
//     let config: HashMap<String, Map<String, toml::Value>> = toml::from_str(&contents).unwrap();

//     let mut categories = Vec::new();

//     if let toml::Value::Table(items) = toml_value {
//         for (category_name, category_value) in items {
//             let mut category_items = Vec::new();

//             if let toml::Value::Table(item_values) = category_value {
//                 for (item_name, item_value) in item_values {
//                     let json_value: Value = serde_json::from_str(&item_value.to_string())?;
//                     category_items.push(PreferenceItem {
//                         name: item_name,
//                         value: json_value,
//                     });
//                 }
//             }

//             categories.push(PreferenceCategory {
//                 name: category_name,
//                 content: category_items,
//             });
//         }
//     }

//     Ok(categories)
// }
