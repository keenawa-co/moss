mod gql;
mod graphql;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::post,
    Extension, Router,
};

async fn graphql_handler(
    schema: Extension<gql::ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    return schema.execute(req.into_inner()).await.into();
}

pub async fn graphiql_handler() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn ping_handler() -> &'static str {
    "pong"
}

pub fn router() -> Router {
    return Router::new()
        .route("/ping", axum::routing::get(ping_handler))
        .route("/graphiql", axum::routing::get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(gql::graphql_schema()));
}
