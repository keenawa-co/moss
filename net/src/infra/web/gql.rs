use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use http::HeaderMap;

use crate::infra::graphql::SchemaRoot;

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(
    schema: Extension<SchemaRoot>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    return schema.execute(req.into_inner().data(headers)).await.into();
}

pub fn router<S>(schema: SchemaRoot) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route(
            "/graphql",
            post(graphql_handler).get_service(GraphQLSubscription::new(schema)),
        )
}
