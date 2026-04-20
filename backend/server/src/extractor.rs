use std::str::FromStr;
use axum::{async_trait, Extension, Json, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use tracing::info;
use uchat_domain::ids::{SessionId, UserId};
use uchat_endpoint::RequestFailed;
use uchat_query::OwnedAsyncConnection;
use crate::AppState;

pub struct DbConnection(
    pub OwnedAsyncConnection
);

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where S: Send + Sync
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S
    ) -> Result<Self, Self::Rejection> {
        let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();
        let connection = state.db_pool.get_owned().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to connect to database",
            )
        })?;

        Ok(Self(connection))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct UserSession {
    pub user_id: UserId,
    pub session_id: SessionId,
}
#[async_trait]
impl<S> FromRequestParts<S> for UserSession
where S: Send + Sync
{
    type Rejection = (StatusCode, Json<RequestFailed>);

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S
    ) -> Result<Self, Self::Rejection> {
        let unauthorizated = || {
            (
                StatusCode::UNAUTHORIZED,
                Json(RequestFailed {
                    msg: "unauthorized".into(),
                })
            )
        };

        let DbConnection(mut conn) = parts.extract::<DbConnection>().await.unwrap();
        let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();

        let cookies = parts.headers.get(
            header::COOKIE
        ).and_then(
            |header| header.to_str().ok()
        ).ok_or_else(unauthorizated)?;

        let session_id = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_ID).and_then(|id| {
            SessionId::from_str(id).ok()
        }).ok_or_else(unauthorizated)?;

        let signature = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_SIGNATURE).and_then(
            |sign| {
                uchat_crypto::decode_base64(sign).ok()
            }
        ).and_then(
            |sign| {
                uchat_crypto::sign::signature_from_bytes(sign).ok()
            }
        ).ok_or_else(unauthorizated)?;

        state.signing_keys.verify(
            session_id.as_uuid().as_bytes(),
            signature
        ).map_err(|_| unauthorizated())?;

        let session = uchat_query::session::get(&mut conn, session_id).ok().flatten().ok_or_else(unauthorizated)?;

        info!(
            user_id = session.user_id.into_inner().to_string(),
            "user logged in"
        );

        Ok(Self {
            user_id: session.user_id,
            session_id
        })
    }
}