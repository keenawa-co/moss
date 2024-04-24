use std::collections::HashMap;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GITIGNORE_FILE_CONTENT: &'static str = ".cache";
    static ref README_FILE_CONTENT: &'static str = "";
}

lazy_static! {
    static ref MY_MAP: HashMap<PathBuf, String> = {
        let mut m = HashMap::new();
        m.insert(
            PathBuf::from(".gitignore"),
            GITIGNORE_FILE_CONTENT.to_string(),
        );
        m.insert(PathBuf::from("README.md"), README_FILE_CONTENT.to_string());
        m
    };
}
