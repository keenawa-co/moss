use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription, GraphQLWebSocket};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

use crate::infra::graphql::SchemaRoot;

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(schema: Extension<SchemaRoot>, req: GraphQLRequest) -> GraphQLResponse {
    return schema.execute(req.into_inner()).await.into();
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
