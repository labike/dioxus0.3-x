use axum::{async_trait, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::AppState;
use crate::error::ApiResult;
use crate::extractor::{DbConnection, UserSession};

pub mod user;

#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;

    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where Req:PublicApiRequest + Deserialize<'a>
{
    payload.process_request(conn, state).await
}

// 会话请求认证
#[async_trait]
pub trait AuthorizatedApiRequest {
    type Response: IntoResponse;

    async fn process_request(
        self,
        conn: DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_handler<'a, Req>(
    conn: DbConnection,
    session: UserSession,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where Req:AuthorizatedApiRequest + Deserialize<'a>
{
    payload.process_request(conn, session, state).await
}