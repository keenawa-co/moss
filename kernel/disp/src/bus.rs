use common::id::MNID;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use typemap::TypeMap;

#[derive(Debug, Clone)] // , Copy
pub enum Body {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    ByteArray(Vec<u8>),
    Boolean(bool),
    Char(char),
    Null,
    Ping,
}

// pub struct Message {
//     pub id: MNID,
//     body: Vec<u8>,
// }

// #[derive(Clone)]
// pub struct Topic {
//     name: String,
//     rx: Arc<Mutex<mpsc::Receiver<Message>>>,
//     pub tx: Arc<Mutex<mpsc::Sender<Message>>>,
// }

// impl Topic {
//     pub fn new(name: String) -> Self {
//         let (tx, rx) = mpsc::channel(100);
//         Topic {
//             name,
//             tx: Arc::new(Mutex::new(tx)),
//             rx: Arc::new(Mutex::new(rx)),
//         }
//     }
// }

// pub trait Consumer {
//     fn process(&self, message: Message);
// }

// pub trait Producer {
//     fn publish(&self, message: Message);
// }

// pub struct Bus {
//     topics: Mutex<HashMap<String, Topic>>,
// }

// impl Bus {
//     pub fn new() -> Arc<Self> {
//         Arc::new(Self {
//             topics: Mutex::new(HashMap::new()),
//         })
//     }

//     pub async fn create_topic(self: &Arc<Self>, topic_name: &str) {
//         let mut topics_lock = self.topics.lock().await;
//         topics_lock.insert(topic_name.to_string(), Topic::new(topic_name.to_string()));
//     }

//     pub async fn subscribe<T>(
//         self: &Arc<Self>,
//         topic_name: &str,
//         consumer: Arc<T>,
//     ) -> anyhow::Result<Topic>
//     where
//         T: Consumer + Send + Sync + 'static,
//     {
//         let topics_lock = self.topics.lock().await;

//         if let Some(topic) = topics_lock.get(topic_name) {
//             let rx_clone = topic.rx.clone();

//             task::spawn(async move {
//                 let mut rx_lock = rx_clone.lock().await;

//                 while let Some(message) = rx_lock.recv().await {
//                     consumer.process(message);
//                 }
//             });

//             Ok(topic.clone())
//         } else {
//             Err(anyhow!("no such topic"))
//         }
//     }
// }

pub struct Message {
    pub id: MNID,
    type_id: TypeId,
    pub body: Box<dyn Any + Send + Sync>,
}

impl Message {
    pub fn new<T: Send + Sync + 'static>(body: Box<dyn Any + Send + Sync>) -> Self {
        Self {
            id: MNID::new(),
            type_id: TypeId::of::<T>(),
            body,
        }
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[derive(Clone)]
pub struct Topic {
    name: String,
    rx: Arc<Mutex<mpsc::Receiver<Message>>>,
    pub tx: Arc<Mutex<mpsc::Sender<Message>>>,
    subscribers: Arc<Mutex<HashMap<TypeId, Arc<dyn Consumer>>>>,
}

impl Topic {
    pub fn new(name: String) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Topic {
            name,
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn subscribe<T: Any + Send + Sync + 'static>(&self, consumer: Arc<dyn Consumer>) {
        let mut subscribers = self.subscribers.lock().await;
        subscribers.insert(TypeId::of::<T>(), consumer);
    }
}

pub trait Consumer: Send + Sync + 'static {
    fn process(&self, message: Message);
}

pub trait Producer {
    fn publish(&self, message: Message);
}

pub struct Bus {
    topics: Mutex<HashMap<String, Topic>>,
}

impl Bus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            topics: Mutex::new(HashMap::new()),
        })
    }

    pub async fn create_topic(self: &Arc<Self>, topic_name: &str) -> Topic {
        let mut topics_lock = self.topics.lock().await;
        let topic = Topic::new(topic_name.to_string());
        topics_lock.insert(topic_name.to_string(), topic.clone());

        topic
    }

    pub async fn start_topic(self: &Arc<Self>, topic_name: &str) {
        let topic = {
            let topics = self.topics.lock().await;
            topics.get(topic_name).cloned()
        };

        if let Some(topic) = topic {
            let rx = topic.rx.clone();
            tokio::task::spawn(async move {
                let mut receiver = rx.lock().await;
                while let Some(message) = receiver.recv().await {
                    let subscribers = topic.subscribers.lock().await;
                    if let Some(consumer) = subscribers.get(&message.type_id) {
                        consumer.process(message);
                    }
                }
            });
        }
    }
}
