use fs::{real, FS};
use futures::{Stream, StreamExt};
use hashbrown::HashSet;
use project::Project;
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use types::file::json_file::JsonFile;

use crate::domain::model::{result::Result, OptionExtension};

pub struct ProjectService {
    realfs: Arc<real::FileSystem>,
    project: Option<Project>,
}

impl ProjectService {
    pub fn new(realfs: Arc<real::FileSystem>) -> Self {
        Self {
            realfs,
            project: None,
        }
    }

    pub async fn start_project(
        &mut self,
        project_path: &PathBuf,
        settings_file: Arc<JsonFile>,
    ) -> Result<()> {
        self.project = Some(Project::new(project_path, settings_file).await?);

        Ok(())
    }

    pub async fn watch_project(&self) -> Result<Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>> {
        let ignored_list: Box<HashSet<String>> = Box::new(
            self.project_ref()?
                .settings
                .fetch_exclude_list()
                .await
                .into_iter()
                .flatten()
                .collect(),
        );

        let stream = self
            .realfs
            .watch(&self.project.as_ref().unwrap().dir, Duration::from_secs(1))
            .await
            .filter_map(move |event_paths| path_filtration(event_paths, Box::clone(&ignored_list)));

        Ok(Box::pin(stream))
    }

    pub async fn append_to_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        Ok(self
            .project_ref()?
            .settings
            .append_to_monitoring_exclude_list(input_list)
            .await?)
    }

    pub async fn remove_from_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        Ok(self
            .project_ref()?
            .settings
            .remove_from_monitoring_exclude_list(input_list)
            .await?)
    }
}

impl ProjectService {
    fn project_ref(&self) -> Result<&Project> {
        Ok(self
            .project
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)?)
    }
}

async fn path_filtration(
    event_paths: Vec<PathBuf>,
    ignore_list: Box<HashSet<String>>,
) -> Option<Vec<PathBuf>> {
    let filtered_paths = event_paths
        .into_iter()
        .filter(|path| {
            !ignore_list
                .iter()
                .any(|ignore_path| path.starts_with(ignore_path))
        })
        .collect::<Vec<PathBuf>>();

    if filtered_paths.is_empty() {
        None
    } else {
        Some(filtered_paths)
    }
}
