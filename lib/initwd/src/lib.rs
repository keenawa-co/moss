use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use fs::FS;

#[macro_use]
extern crate lazy_static;

lazy_static! {
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

fn create_from_scratch(fs: Rc<dyn FS>) -> anyhow::Result<()> {
    todo!()
}
