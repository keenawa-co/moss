use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::rc::Rc;
use std::{collections::HashMap, path::Path};

use fs::{real, FS};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LOCAL_FOLDER_NAME: Lazy<String> = Lazy::new(|| { format!(".{}", common::APP_NAME) });
    static ref GITIGNORE_FILE_CONTENT: &'static str = ".cache";
    static ref README_FILE_CONTENT: &'static str = "";
}

lazy_static! {
    static ref INIT_LIST: HashMap<PathBuf, String> = {
        let mut m = HashMap::new();
        m.insert(
            PathBuf::from(".gitignore"),
            GITIGNORE_FILE_CONTENT.to_string(),
        );
        m.insert(PathBuf::from("README.md"), README_FILE_CONTENT.to_string());
        m
    };
}

pub async fn create_from_scratch(project_dir: &str, fs: &real::FileSystem) -> anyhow::Result<()> {
    // TODO: check the folder for an already initialized working directory

    fs.create_dir(Path::new(&format!(
        "{project_dir}/{}",
        LOCAL_FOLDER_NAME.as_str()
    )))
    .await?;
    Ok(())
}
