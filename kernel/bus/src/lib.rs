pub mod bus;
pub mod message;
pub mod topic;

pub use bus::Bus;

use crate::message::simple_message::SimpleMessage;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate async_trait;

#[async_trait]
pub trait Consumer: Send + Sync + 'static {
    fn process(&self, topic_name: &str, message: &SimpleMessage);
}

#[async_trait]
pub trait Producer: Send + Sync + 'static {
    async fn publish(&self, topic_name: &str, message: SimpleMessage) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Subscriber: Consumer + Producer {}
