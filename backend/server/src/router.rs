use axum::Router;
use axum::routing::get;
use crate::AppState;

pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(|| async { "this is the root page!" }));
    let authorized_routes = Router::new();

    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
}