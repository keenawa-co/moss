use anyhow::anyhow;
use fs::{
    real::{self, types::FileSystemEntity},
    CreateOptions, FS,
};
use futures::io::Cursor;
use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
};

static CONTENT: Lazy<anyhow::Result<FileSystemEntity>> = Lazy::new(|| {
    let result = FileSystemEntity::Directory {
        name: ".moss".to_string(),
        children: Some(vec![
            FileSystemEntity::Directory {
                name: "cache".to_string(),
                children: None,
            },
            FileSystemEntity::File {
                name: ".gitignore".to_string(),
                content: Some("cache".to_string()),
            },
            FileSystemEntity::File {
                name: "moss.json".to_string(),
                content: Some(
                    serde_json::to_string_pretty(&manifest::model::file::RootFile::default())
                        .unwrap_or_else(|e| {
                            format!("manifest file content cannot be serialized: {}", e)
                        }),
                ),
            },
            FileSystemEntity::File {
                name: "README.md".to_string(),
                content: None,
            },
        ]),
    };

    Ok(result)
});

pub async fn create_from_scratch<P: AsRef<Path>>(
    project_path: P,
    fs: &real::FileSystem,
) -> anyhow::Result<PathBuf> {
    // TODO: check the folder for an already initialized working directory

    let content = CONTENT
        .as_ref()
        .map_err(|e| anyhow!("Failed to prepare content: {e}"))?;

    save_on_disk(&project_path.as_ref().to_path_buf(), &content, fs).await?;

    Ok(project_path.as_ref().join(content.name()))
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
