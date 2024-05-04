use sea_orm::{DatabaseConnection, DatabaseTransaction, IsolationLevel, TransactionTrait};
use std::{future::Future, ops::Deref, sync::Arc};

pub struct TransactionHandle(Arc<Option<DatabaseTransaction>>);

impl Deref for TransactionHandle {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().as_ref().unwrap()
    }
}

pub async fn weak_transaction<F, Fut, T>(conn: &DatabaseConnection, f: F) -> anyhow::Result<T>
where
    F: Send + Fn(TransactionHandle) -> Fut,
    Fut: Send + Future<Output = anyhow::Result<T>>,
{
    let tx = conn
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await?;

    let mut tx = Arc::new(Some(tx));
    let result = f(TransactionHandle(tx.clone())).await;
    let Some(tx) = Arc::get_mut(&mut tx).and_then(|tx| tx.take()) else {
        return Err(anyhow!(
            "couldn't complete transaction because it's still in use"
        ))?;
    };

    match result {
        Ok(result) => match tx.commit().await.map_err(Into::into) {
            Ok(()) => return Ok(result),
            Err(error) => {
                return Err(error);
            }
        },
        Err(error) => {
            tx.rollback().await?;
            return Err(error);
        }
    }
}
