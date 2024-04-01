use crate::api::graphql::SchemaRoot;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::post,
    Extension, Router,
};

async fn graphql_handler(schema: Extension<SchemaRoot>, req: GraphQLRequest) -> GraphQLResponse {
    return schema.execute(req.into_inner()).await.into();
}

async fn graphiql_handler() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    return Router::new()
        .route("/graphiql", axum::routing::get(graphiql_handler))
        .route("/graphql", post(graphql_handler));
    // .layer(Extension(schema));
}
