use common::id::MNID;
use std::any::Any;

pub trait MessageBody: Any + Send + Sync {
    fn body<T: Any + Send + Sync + 'static>(&self) -> anyhow::Result<&T>;
}

pub trait Message {
    fn id(&self) -> &MNID;
}

pub mod simple_message {
    use super::MNID;
    use std::any::{Any, TypeId};

    pub struct SimpleMessage {
        pub(crate) id: MNID,
        pub(crate) type_id: TypeId,
        pub(crate) body: Box<dyn Any + Send + Sync>,
    }

    impl super::Message for SimpleMessage {
        fn id(&self) -> &MNID {
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
                id: MNID::new(),
                type_id: TypeId::of::<T>(),
                body,
            }
        }
    }
}
