use axum::{Extension, Json, extract::State};
use reqwest::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    application::{error::AppError, traits::types::CurrentUser},
    domain::{authentication::error::AuthenticationError, user::error::UserError},
    presentation::{error::ErrorResponse, response::user_response::CurrentUserResponse},
};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn get_me(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(StatusCode, Json<CurrentUserResponse>), AppError> {
    let result = state
        .user_service
        .get_user(current_user.id)
        .await
        .map_err(|e| match e {
            AppError::User(UserError::NotFound) => AuthenticationError::AuthenticationFailed.into(),
            _ => e,
        })?;
    Ok((StatusCode::OK, Json(result.into())))
}

pub fn create_user_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_me))
}
