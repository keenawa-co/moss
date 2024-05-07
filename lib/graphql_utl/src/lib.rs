use async_graphql::ErrorExtensions;

pub mod path;

#[macro_use]
extern crate serde;

pub trait GraphQLExtendError<T, E: ErrorExtensions> {
    fn extend_error(self) -> Result<T, async_graphql::Error>;
}

type ExtendedGraphQLResult<T> = Result<T, async_graphql::Error>;

impl<T, E: ErrorExtensions> GraphQLExtendError<T, E> for Result<T, E> {
    fn extend_error(self) -> ExtendedGraphQLResult<T> {
        self.map_err(|e| e.extend())
    }
}
