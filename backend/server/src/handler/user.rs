use axum::http::StatusCode;
use axum::{async_trait, Json};
use tracing::info;
use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk};
use crate::AppState;
use crate::error::ApiResult;
use crate::extractor::DbConnection;
use crate::handler::PublicApiRequest;

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(&self.password)?;
        let user_id = uchat_query::user::new(
            &mut conn,
            password_hash,
            &self.username
        )?;
        info!(username = self.username.as_ref(), "new user created");
        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username
            })
        ))
    }
}