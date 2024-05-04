use async_graphql::{http, ErrorExtensions};

pub mod path;

#[macro_use]
extern crate serde;

pub trait GraphQLExtendError<T, E: ErrorExtensions> {
    fn extend_error(self) -> Result<T, async_graphql::Error>;
    fn extend_error_with_status_code(self, status_code: usize) -> Result<T, async_graphql::Error>;
}

type ExtendedGraphQLResult<T> = Result<T, async_graphql::Error>;

impl<T, E: ErrorExtensions> GraphQLExtendError<T, E> for Result<T, E> {
    fn extend_error(self) -> ExtendedGraphQLResult<T> {
        self.map_err(|e| e.extend())
    }

    fn extend_error_with_status_code(self, status_code: usize) -> Result<T, async_graphql::Error> {
        self.map_err(|e| e.extend_with(|_, e| e.set("status_code", status_code.to_string())))
    }
}
