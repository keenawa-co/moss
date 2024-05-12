use common::{id::NanoId, thing::Thing};
use fs::{real, FS};
use futures::{Stream, StreamExt};
use hashbrown::HashSet;
use manifest::model::ignored::IgnoredSource;
use project::Project;
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};

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

    pub async fn watch_project(&self) -> Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>> {
        let ignored_list = self
            .project
            .as_ref()
            .unwrap()
            .manifest
            .fetch_ignored_list()
            .await
            .clone();

        let stream = self
            .realfs
            .watch(&self.project.as_ref().unwrap().root, Duration::from_secs(1))
            .await
            .filter_map(move |event_paths| path_filtration(event_paths, ignored_list.clone()));

        Box::pin(stream)
    }

    pub async fn append_to_ignore_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> Result<Vec<IgnoredSource>> {
        Ok(self
            .project
            .as_ref()
            .unwrap() // FIXME: .ok_or_resource_precondition_required("Session must be initialized first", None)
            .manifest
            .append_to_ignored_list(input_list)
            .await?)
    }

    pub async fn remove_from_ignore_list(&self, id: &NanoId) -> Result<Thing> {
        Ok(self
            .project
            .as_ref()
            .unwrap() // FIXME: .ok_or_resource_precondition_required("Session must be initialized first", None)
            .manifest
            .remove_from_ignore_list(id)
            .await?
            .ok_or_resource_not_found(&format!("project with id {} does not exist", id), None)?)
        // TODO: use quote! macros
    }
}

async fn path_filtration(
    event_paths: Vec<PathBuf>,
    ignore_list: Arc<HashSet<IgnoredSource>>,
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
