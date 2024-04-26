use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    message::simple_message::SimpleMessage,
    topic::{Topic, TopicConfig},
    Producer, Subscriber,
};

pub struct Bus {
    topics: RwLock<HashMap<String, Arc<Topic>>>,
}

impl Bus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            topics: RwLock::new(HashMap::new()),
        })
    }

    pub async fn create_topic(&self, topic_name: &str, conf: TopicConfig) {
        let mut topics_lock = self.topics.write().await;
        topics_lock
            .entry(topic_name.to_string())
            .or_insert_with(|| Arc::new(Topic::new(topic_name.into(), conf)));
    }

    pub async fn subscribe_topic<T: Any + Send + Sync>(
        &self,
        topic_name: &str,
        subscriber: Arc<dyn Subscriber>,
    ) -> anyhow::Result<()> {
        let topics_lock = self.topics.read().await;
        if let Some(topic) = topics_lock.get(topic_name) {
            let mut roster_lock = topic.roster.write().await;

            let callback = {
                let processor = subscriber.clone();
                Arc::new(move |topic_name: &str, message: &SimpleMessage| {
                    processor.process(topic_name, message)
                })
            };

            roster_lock.insert(TypeId::of::<T>(), callback);
            topic.start().await;

            Ok(())
        } else {
            Err(anyhow!("Topic {topic_name} not found"))
        }
    }
}

#[async_trait]
impl Producer for Bus {
    async fn publish(&self, topic_name: &str, message: SimpleMessage) -> anyhow::Result<()> {
        let topics_lock = self.topics.read().await;
        if let Some(topic) = topics_lock.get(topic_name) {
            topic.tx.send(message).await?;
            Ok(())
        } else {
            Err(anyhow!("Topic {topic_name} not found"))
        }
    }
}
