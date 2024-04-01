mod gql;
mod graphql;
mod status;

use axum::Router;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let router = Router::new().merge(status::router()).merge(gql::router());

    // TODO: setup the graceful shutdown

    return router;
}
