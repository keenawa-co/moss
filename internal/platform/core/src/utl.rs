use anyhow::Result;

pub trait FlattenAnyhowResult<T> {
    fn flatten(self) -> Result<T>;
}

impl<T> FlattenAnyhowResult<T> for Result<Result<T>> {
    fn flatten(self) -> Result<T> {
        self?
    }
}

impl<T> FlattenAnyhowResult<T> for Result<T> {
    fn flatten(self) -> Result<T> {
        self
    }
}
