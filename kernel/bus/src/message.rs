use std::any::Any;
use types::id::NanoId;

pub trait MessageBody: Any + Send + Sync {
    fn body<T: Any + Send + Sync + 'static>(&self) -> anyhow::Result<&T>;
}

pub trait Message {
    fn id(&self) -> &NanoId;
}

pub mod simple_message {
    use super::NanoId;
    use std::any::{Any, TypeId};

    pub struct SimpleMessage {
        pub(crate) id: NanoId,
        pub(crate) type_id: TypeId,
        pub(crate) body: Box<dyn Any + Send + Sync>,
    }

    impl super::Message for SimpleMessage {
        fn id(&self) -> &NanoId {
            &self.id
        }
    }

    impl super::MessageBody for SimpleMessage {
        fn body<T: Send + Sync + 'static>(&self) -> anyhow::Result<&T> {
            self.body
                .downcast_ref::<T>()
                .ok_or_else(|| anyhow!("Failed to downcast to the specified type"))
        }
    }

    impl SimpleMessage {
        pub fn new<T: Send + Sync + 'static>(body: Box<dyn Any + Send + Sync>) -> Self {
            Self {
                id: NanoId::new(),
                type_id: TypeId::of::<T>(),
                body,
            }
        }
    }
}
