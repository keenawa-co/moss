use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use http::HeaderMap;

use crate::{domain::model::session::SessionTokenClaims, infra::graphql::SchemaRoot};

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(
    schema: Extension<SchemaRoot>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(value) = headers.get("session-token") {
        match SessionTokenClaims::try_from(value) {
            Ok(claims) => req = req.data(claims),
            Err(e) => {
                return GraphQLResponse::from(async_graphql::Response::from_errors(vec![e.into()]));
            }
        };
    }

    return schema.execute(req.data(headers)).await.into();
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
