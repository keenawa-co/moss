use axum::Router;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    return Router::new().route("/status", axum::routing::get(status_handler));
}

async fn status_handler() {}
