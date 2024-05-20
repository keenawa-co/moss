use fs::{real, FS};
use futures::{Stream, StreamExt};
use hashbrown::HashSet;
use project::{worktree::local::WorkTreeEvent, Project};
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use types::file::json_file::JsonFile;

use crate::domain::model::{result::Result, OptionExtension};

pub struct ProjectService {
    realfs: Arc<real::FileSystem>,
    project: Option<Project>,
}

impl ProjectService {
    pub fn init(realfs: Arc<real::FileSystem>) -> Self {
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
        let arc_path: Arc<Path> = Arc::from(project_path.clone().into_boxed_path());

        self.project = Some(Project::new(self.realfs.clone(), arc_path, settings_file).await?);

        Ok(())
    }

    pub async fn explorer_event_feed(
        &self,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = WorkTreeEvent>>>> {
        let stream = self.project_ref()?.worktree_event_stream().await;

        Ok(Box::pin(stream))
    }

    pub async fn watch_project(&self) -> Result<Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>> {
        // dbg!(s);
        // let project = self.project_ref()?;
        // let exclude_watch_rx = project.settings.watch_monitoring_exclude_list();
        // let r = Arc::new(project.dir);
        // let stream = self
        //     .realfs
        //     .watch(&project.dir, Duration::from_secs(1))
        //     .await
        //     .filter_map(move |event_paths| {
        //         let mut exclude_rx = exclude_watch_rx.clone();
        //         async move {
        //             let exclude = exclude_rx.borrow_and_update().clone();

        //             path_filtration(event_paths, exclude).await
        //         }
        //     });

        // Ok(Box::pin(stream))

        unimplemented!()
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
    ignore_list: HashSet<PathBuf>,
) -> Option<Vec<PathBuf>> {
    dbg!(&ignore_list);
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
