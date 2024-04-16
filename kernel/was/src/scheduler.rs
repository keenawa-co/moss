// use mlua::Lua;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

#[async_trait]
pub trait MetricResultStorage {
    async fn create(&self, input: metric::FeedUpdateInput) -> anyhow::Result<metric::Chunk>;
}

pub trait Storage {
    fn metric(&self) -> dyn MetricResultStorage;
}

pub struct Scheduler {
    pub session_id: String,
    pub project_id: String,

    // pub context: Arc<Lua>,
    // pub storage: Box<dyn Storage>,
    pub tx: Arc<Sender<String>>,
}

impl Scheduler {
    pub async fn exec(&self, test_data: &str) -> anyhow::Result<usize> {
        Ok(self.tx.send(format!("Hello, {test_data}"))?)
    }
}

// #[async_trait]
// pub trait MetricResultStorage {
//     async fn create(&self, input: metric::FeedUpdateInput) -> anyhow::Result<metric::Chunk>;
// }

// pub trait Storage {
//     fn metric(&self) -> dyn MetricResultStorage;
// }

// pub struct Scheduler {
//     pub session_id: String,
//     pub project_id: String,

//     // pub context: Arc<Lua>,
//     // pub storage: Box<dyn Storage>,
//     pub tx: Arc<Sender<String>>,
// }
