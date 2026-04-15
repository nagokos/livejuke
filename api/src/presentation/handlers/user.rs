use axum::{Extension, Json, extract::State};
use reqwest::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    application::{error::AppError, traits::types::CurrentUser},
    domain::{
        authentication::error::AuthenticationError,
        user::{display_name::DisplayName, error::UserError, model::UpdateUser},
    },
    presentation::{
        error::ErrorResponse, request::user::UserUpdateInput,
        response::user_response::CurrentUserResponse,
    },
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

#[utoipa::path(
    patch,
    path = "/",
    request_body = UserUpdateInput,
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 400, body = ErrorResponse, description = "invalid input"),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn update_me(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(input): Json<UserUpdateInput>,
) -> Result<(StatusCode, Json<CurrentUserResponse>), AppError> {
    let update_user = UpdateUser::new()
        .display_name(DisplayName::try_new(input.display_name).map_err(UserError::from)?);

    let result = state
        .user_service
        .update_user(current_user.id, update_user)
        .await?;
    Ok((StatusCode::OK, Json(result.into())))
}

pub fn create_user_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_me))
        .routes(routes!(update_me))
}
