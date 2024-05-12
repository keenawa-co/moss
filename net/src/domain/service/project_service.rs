use fs::{real, FS};
use futures::{Stream, StreamExt};
use hashbrown::HashSet;
use manifest::model::ignored::{IgnoredSource, ToHashSet};
use project::Project;
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use types::{id::NanoId, thing::Thing};

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

    pub async fn start_project(&mut self, project_path: &PathBuf) -> Result<()> {
        self.project = Some(Project::new(project_path).await?);

        Ok(())
    }

    pub async fn watch_project(&self) -> Result<Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>> {
        let ignored_list = Box::new(
            self.project_ref()?
                .manifest
                .fetch_ignored_list()
                .await?
                .to_hash_set(),
        );

        let stream = self
            .realfs
            .watch(&self.project.as_ref().unwrap().root, Duration::from_secs(1))
            .await
            .filter_map(move |event_paths| path_filtration(event_paths, Box::clone(&ignored_list)));

        Ok(Box::pin(stream))
    }

    pub async fn append_to_ignore_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> Result<Vec<IgnoredSource>> {
        Ok(self
            .project_ref()?
            .manifest
            .append_to_ignored_list(input_list)
            .await?)
    }

    pub async fn remove_from_ignore_list(&self, id: &NanoId) -> Result<Thing> {
        Ok(self
            .project_ref()?
            .manifest
            .remove_from_ignore_list(id)
            .await?
            .ok_or_resource_not_found(
                &format!("ignored path with id {} does not exist", quote!(id)),
                None,
            )?)
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
    ignore_list: Box<HashSet<IgnoredSource>>,
) -> Option<Vec<PathBuf>> {
    let filtered_paths = event_paths
        .into_iter()
        .filter(|path| {
            !ignore_list
                .iter()
                .any(|ignore_path| path.starts_with(ignore_path.source.clone()))
        })
        .collect::<Vec<PathBuf>>();

    if filtered_paths.is_empty() {
        None
    } else {
        Some(filtered_paths)
    }
}
