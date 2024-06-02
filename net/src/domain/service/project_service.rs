use app::event::PlatformEvent;
use arc_swap::ArcSwapOption;
use fs::{real, FS};
use futures::{Stream, StreamExt};
use project::Project;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use types::file::json_file::JsonFile;

use crate::domain::{
    self,
    model::{result::Result, OptionExtension},
};

pub struct ProjectService<'a> {
    realfs: Arc<dyn FS>,
    project: ArcSwapOption<Project>,
    test: &'a i128,
}

impl<'a> ProjectService<'a> {
    pub fn init(realfs: Arc<dyn FS>) -> Arc<Self> {
        Arc::new(Self {
            realfs,
            project: ArcSwapOption::from(None),
            test: &10,
        })
    }

    pub async fn start_project(
        self: &Arc<Self>,
        project_path: &PathBuf,
        settings_file: Arc<JsonFile>,
    ) -> Result<()> {
        let arc_path: Arc<Path> = Arc::from(project_path.clone().into_boxed_path());

        let project = Project::new(self.realfs.clone(), arc_path, settings_file).await?;
        self.project.store(Some(Arc::new(project)));

        Ok(())
    }

    pub async fn event_live_stream(
        self: &'a Arc<Self>,
    ) -> Result<
        Pin<Box<dyn Send + Stream<Item = PlatformEvent<'a>> + 'a>>,
        domain::model::error::Error,
    > {
        let stream = self.get_project()?.event_stream().await;

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
        self: &Arc<Self>,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        Ok(self
            .get_project()?
            .settings
            .append_to_monitoring_exclude_list(input_list)
            .await?)
    }

    pub async fn remove_from_monitoring_exclude_list(
        self: &Arc<Self>,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        Ok(self
            .get_project()?
            .settings
            .remove_from_monitoring_exclude_list(input_list)
            .await?)
    }
}

impl<'a> ProjectService<'a> {
    fn get_project(&'a self) -> Result<Arc<Project>> {
        Ok(self
            .project
            .load_full()
            .ok_or_resource_precondition_required("Session must be initialized first", None)?)
    }
}
