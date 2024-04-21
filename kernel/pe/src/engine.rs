use anyhow::Context;
use disp::signal::{FileSignal, Origin, Signal};
use fs::fw::FileWatcher;
use futures::{stream::StreamExt, Stream};
use std::collections::HashSet;
use std::pin::Pin;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::policy::Report;

pub struct Engine {
    fw: Arc<FileWatcher>,
    watch_list: Mutex<HashSet<PathBuf>>,
}

impl Engine {
    pub fn new(fw: Arc<FileWatcher>) -> Arc<Self> {
        let engine = Self {
            fw,
            watch_list: Mutex::new(HashSet::new()),
        };

        Arc::new(engine)
    }

    pub fn watch_path_list<P>(self: &Arc<Self>, paths: Vec<P>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut list_lock = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;

        for path in paths {
            let path_buf = path.as_ref().to_path_buf();
            if list_lock.insert(path_buf.clone()) {
                self.fw
                    .watch_path(&path_buf)
                    .with_context(|| format!("failed to watch path: {:?}", path_buf))?;
            }
        }

        Ok(())
    }

    pub fn run(
        self: &Arc<Self>,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Report>> + Send>>> {
        let mut rx = self.fw.subscribe()?;

        let stream = async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(signal) => match signal.origin {
                        Origin::FileWatcher(FileSignal::Modify(data)) => {
                            for p in data {
                                let report = Report { source: path_buf_to_string(p).unwrap() };
                                yield Ok(report);
                            }
                        },
                    },
                    Err(e) => {
                        let err = anyhow!("Error receiving signal: {}", e);
                        yield Err(err);
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}

fn path_buf_to_string(path_buf: PathBuf) -> Result<String, String> {
    match path_buf.to_str() {
        Some(path_str) => Ok(path_str.to_string()),
        None => Err("invalid path".to_string()),
    }
}
