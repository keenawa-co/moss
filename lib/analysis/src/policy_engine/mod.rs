use anyhow::Context;
use async_graphql::SimpleObject;
use bus::message::simple_message::SimpleMessage;
use bus::{Bus, Producer};

use fs::fw::FileWatcher;
use futures::Stream;
use std::collections::HashSet;
use std::pin::Pin;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[derive(Clone, Serialize, Deserialize, Debug, SimpleObject)]
pub struct Report {
    pub source: String,
}

pub struct PolicyEngine {
    fw: Arc<FileWatcher>,
    bus: Arc<Bus>,
    watch_list: Mutex<HashSet<PathBuf>>,
}

impl PolicyEngine {
    pub fn new(fw: Arc<FileWatcher>, bus: Arc<Bus>) -> Self {
        Self {
            fw,
            bus,
            watch_list: Mutex::new(HashSet::new()),
        }
    }

    pub fn register_watch_list<P>(&self, paths: Vec<P>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut watch_list_lock = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;

        for path in paths {
            let path_buf = path.as_ref().to_path_buf();
            if watch_list_lock.insert(path_buf.clone()) {
                self.fw
                    .watch_path(&path_buf)
                    .with_context(|| format!("failed to watch path: {:?}", path_buf))?;
            }
        }

        Ok(())
    }

    pub async fn subscribe(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Report>> + Send>>> {
        let mut rx = self.fw.subscribe()?;

        self.bus
            .publish("general", SimpleMessage::new::<i32>(Box::new(32)))
            .await?;

        let stream = async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(event) => match event.kind {
                        notify::EventKind::Modify(kind) => match kind {
                            notify::event::ModifyKind::Any => todo!(),
                            notify::event::ModifyKind::Data(_) => {
                                for p in event.paths {
                                    let report = Report { source: path_buf_to_string(p).unwrap() };
                                    yield Ok(report);
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
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
