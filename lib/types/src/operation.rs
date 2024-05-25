use std::{future::Future, pin::Pin};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type OperationFunc = Box<dyn Fn() -> BoxFuture<'static, Result<(), Box<dyn std::error::Error>>>>;

// type OperationFunc = dyn Fn() -> Result<(), Box<dyn std::error::Error>>;

pub struct Operation {
    apply: Box<OperationFunc>,
    rollback: Box<OperationFunc>,
}

pub struct Transaction(Vec<Operation>);

impl Transaction {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append_operation(&mut self, value: Operation) {
        self.0.push(value)
    }

    pub async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut completed: Vec<&Operation> = Vec::new();

        for operation in &self.0 {
            if let Err(e) = (operation.apply)().await {
                for completed_operation in completed.iter().rev() {
                    (completed_operation.rollback)().await?;
                }

                return Err(e);
            } else {
                completed.push(operation)
            }
        }

        Ok(())
    }
}
