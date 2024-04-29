use fs::{
    real::{self, types::FileSystemEntity},
    CreateOptions, FS,
};
use futures::io::Cursor;
use sea_orm::DatabaseConnection;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
};

use super::db;

lazy_static! {
    static ref README_FILE_CONTENT: &'static str = "";
    static ref GITIGNORE_FILE_CONTENT: &'static str = "cache";
}

lazy_static! {
    static ref ROOT: FileSystemEntity = FileSystemEntity::Directory {
        name: format!(".{}", common::APP_NAME),
        children: Some(vec![
            FileSystemEntity::Directory {
                name: "cache".to_string(),
                children: None
            },
            FileSystemEntity::File {
                name: ".gitignore".to_string(),
                content: Some(GITIGNORE_FILE_CONTENT.to_string())
            },
            FileSystemEntity::File {
                name: "README.md".to_string(),
                content: None
            },
        ]),
    };
}

pub async fn create_from_scratch<P: AsRef<Path>>(
    project_path: P,
    fs: &real::FileSystem,
) -> anyhow::Result<PathBuf> {
    // TODO: check the folder for an already initialized working directory

    save_on_disk(&project_path.as_ref().to_path_buf(), &ROOT, fs).await?;

    Ok(project_path.as_ref().join(ROOT.name()))
}

async fn save_on_disk(
    base_path: &PathBuf,
    entity: &FileSystemEntity,
    fs: &real::FileSystem,
) -> anyhow::Result<()> {
    use FileSystemEntity::{Directory, File};

    match entity {
        File { name, content } => {
            let file_path = base_path.join(name);
            if let Some(content) = content {
                let mut reader = Cursor::new(content.as_bytes());
                let content_pin = Pin::new(&mut reader);

                fs.create_file_with(&file_path, content_pin).await?;
            } else {
                fs.create_file(&file_path, CreateOptions::default()).await?;
            }
        }

        Directory { name, children } => {
            let dir_path = base_path.join(name);
            fs.create_dir(&dir_path).await?;

            if let Some(children) = children {
                for child in children {
                    Box::pin(save_on_disk(&dir_path, child, fs)).await?;
                }
            }
        }
    }
    Ok(())
}
