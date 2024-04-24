pub mod bus;
pub mod message;
pub mod topic;

pub use bus::Bus;

use crate::message::simple_message::SimpleMessage;

#[macro_use]
extern crate anyhow;

pub trait Consumer: Send + Sync + 'static {
    fn process(&self, topic_name: &str, message: &SimpleMessage);
}

pub trait Producer {
    fn publish(&self, message: &SimpleMessage);
}
