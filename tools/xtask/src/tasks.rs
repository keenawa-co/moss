pub mod license;
pub mod rust_workspace_audit;

use anyhow::Result;
use futures::future::join_all;
use std::{future::Future, mem};
use tokio::task::JoinHandle;

pub struct TaskRunner {
    jobs: Vec<JoinHandle<Result<()>>>,
}

impl TaskRunner {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub async fn run(&mut self) -> Result<()> {
        let jobs = mem::take(&mut self.jobs);
        for result in join_all(jobs).await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    return Err(e);
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    pub fn spawn_job(&mut self, job: impl Future<Output = Result<()>> + Send + 'static) {
        self.jobs.push(tokio::task::spawn(job));
    }
}
