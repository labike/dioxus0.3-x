use axum::http::HeaderValue;
use axum::Router;
use axum::routing::{get, post};
use hyper::header::CONTENT_TYPE;
use hyper::Method;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use uchat_endpoint::Endpoint;
use uchat_endpoint::post::endpoint::{Bookmark, NewPost, NewPostOk};
use uchat_endpoint::trending::endpoint::TrendingPosts;
use uchat_endpoint::user::endpoint::{CreateUser, Login, LoginOk};
use crate::AppState;
use crate::handler::{with_handler, with_public_handler};

pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(|| async { "this is the root page!" }))
        .route(CreateUser::URL, post(with_public_handler::<CreateUser>))
        .route(Login::URL, post(with_public_handler::<Login>));
    let authorized_routes = Router::new()
        .route(NewPost::URL, post(with_handler::<NewPost>))
        .route(TrendingPosts::URL, post(with_handler::<TrendingPosts>))
        .route(Bookmark::URL, post(with_handler::<Bookmark>));

    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http().make_span_with(
                    DefaultMakeSpan::new().include_headers(true)
                ).on_request(
                    DefaultOnRequest::new().level(Level::INFO),
                ).on_response(
                    DefaultOnResponse::new().level(Level::INFO).latency_unit(
                        LatencyUnit::Micros
                    ),
                ),
            ).layer(
                CorsLayer::new().allow_methods(
                    [Method::GET, Method::POST, Method::OPTIONS]
                ).allow_credentials(true).allow_origin(
                    std::env::var("FRONTEND_URL").unwrap().parse::<HeaderValue>().unwrap()
                ).allow_headers([CONTENT_TYPE])
            ).layer(
                axum::Extension(state.clone())
            )
        ).with_state(state)
}