use common::{id::NanoId, thing::Thing};
use fs::{real, FS};
use futures::{Stream, StreamExt};
use hashbrown::HashSet;
use project::{ignored::IgnoredSource, Project};
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};

use crate::{
    domain::model::{result::Result, OptionExtension},
    infra::adapter::sqlite::{CacheMigrator, CacheSQLiteAdapter},
};

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
        // FIXME: load ignore_list form DB
        let mut ignored_paths = HashSet::new();
        ignored_paths.insert("/Users/g10z3r/Project/4rchr4y/moss/target/".to_string());

        let conn = dbutl::sqlite::conn::<CacheMigrator>(
            &project_path
                // FIXME: This values must be obtained from the configuration file. UPD: from ProjectConf
                .join(PathBuf::from(".moss/cache"))
                .join(PathBuf::from("cache.db")),
        )
        .await?;

        self.project = Some(Project {
            root: project_path.clone(),
            ignored_list: Arc::new(ignored_paths),
            cache: Arc::new(CacheSQLiteAdapter::new(Arc::new(conn))),
        });

        Ok(())
    }

    pub async fn watch_project(&self) -> Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>> {
        let ignored_list = self.project.as_ref().unwrap().ignored_list.clone();

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
            .cache
            .ignore_list_repo()
            .await
            .create_from_list(input_list)
            .await?)
    }

    pub async fn remove_from_ignore_list(&self, id: &NanoId) -> Result<Thing> {
        let result = self
            .project
            .as_ref()
            .unwrap()
            .cache
            .ignore_list_repo()
            .await
            .delete_by_id(id)
            .await?
            .ok_or_resource_not_found(&format!("project with id {} does not exist", id), None)?;

        Ok(result)
    }
}

async fn path_filtration(
    event_paths: Vec<PathBuf>,
    ignore_list: Arc<HashSet<String>>,
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
