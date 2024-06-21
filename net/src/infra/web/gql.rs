use app::{context::AsyncAppContext, context_compact::AppContextCompact};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    body::Body,
    http::Request,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use http::HeaderMap;
use tower::{Service, ServiceExt};

use crate::{domain::model::session::SessionTokenClaims, infra::graphql::SchemaRoot};

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(
    Extension(ctx): Extension<AppContextCompact>,
    Extension(schema): Extension<SchemaRoot>,
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

    return schema.execute(req.data(ctx).data(headers)).await.into();
}

async fn graphql_subscription_handler(
    Extension(schema): Extension<SchemaRoot>,
    headers: HeaderMap,
    mut req: Request<Body>,
) -> impl IntoResponse {
    if let Some(value) = headers.get("session-token") {
        match SessionTokenClaims::try_from(value) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
            }
            Err(e) => {
                return GraphQLResponse::from(async_graphql::Response::from_errors(vec![e.into()]))
                    .into_response();
            }
        };
    }

    req.extensions_mut().insert(headers);

    GraphQLSubscription::new(schema)
        .call(req)
        .await
        .into_response()
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route(
            "/graphql",
            post(graphql_handler), //.get_service(GraphQLSubscription::new(schema)
        )
        .route("/graphql", get(graphql_subscription_handler))
}
