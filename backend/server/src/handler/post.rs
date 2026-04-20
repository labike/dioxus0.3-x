use axum::http::StatusCode;
use axum::{async_trait, Json};
use crate::AppState;
use crate::extractor::{DbConnection, UserSession};
use crate::handler::AuthorizatedApiRequest;
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_query::post::Post;
use crate::error::ApiResult;

#[async_trait]
impl AuthorizatedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(
            session.user_id,
            self.content,
            self.options
        )?;

        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((
            StatusCode::OK,
            Json(NewPostOk { post_id }),
        ))
    }
}
