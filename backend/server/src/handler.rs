use std::path::PathBuf;
use axum::{async_trait, Json};
use axum::body::{Bytes, Full};
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose;
use hyper::header;
use serde::Deserialize;
use uuid::Uuid;
use uchat_domain::ids::ImageId;
use crate::AppState;
use crate::error::{ApiError, ApiResult};
use crate::extractor::{DbConnection, UserSession};

pub mod user;
pub mod post;

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

const USER_CONTENT_DIR: &str = "usercontent";

pub async fn save_image<T: AsRef<[u8]>>(id: ImageId, data: T) -> Result<(), ApiError> {
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    fs::create_dir_all(&path).await?;
    path.push(id.to_string());
    fs::write(&path, data).await?;

    Ok(())
}

pub async fn load_image(Path(img_id): Path<Uuid>) -> Result<Response<Full<Bytes>>, ApiError> {
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    path.push(img_id.to_string());

    let raw = fs::read_to_string(path).await?;
    let (header, image_data) = raw.split_once(',').unwrap();
    let mime = header.split_once("data:").unwrap().1.split_once(";base64").unwrap().0;

    {
        use base64::{engine::general_purpose, Engine as _};
        let image_data = general_purpose::STANDARD.decode(image_data).unwrap();
        Ok(
            Response::builder().status(StatusCode::OK).header(header::CONTENT_TYPE, mime).body(Full::from(image_data)).unwrap(),
        )
    }
}