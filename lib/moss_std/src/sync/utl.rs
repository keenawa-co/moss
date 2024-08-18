#[async_trait]
pub trait AsyncTryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    async fn try_from_async(value: T) -> Result<Self, Self::Error>;
}
