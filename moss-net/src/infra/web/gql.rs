use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response::IntoResponse, routing::post, Extension, Router};

use crate::infra::graphql::SchemaRoot;

async fn graphql_handler(schema: Extension<SchemaRoot>, req: GraphQLRequest) -> GraphQLResponse {
    return schema.execute(req.into_inner()).await.into();
}

async fn graphiql_handler() -> impl IntoResponse {
    axum::response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/graphiql", axum::routing::get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
}
