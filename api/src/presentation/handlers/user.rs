use axum::{Extension, Json, extract::State};
use reqwest::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    application::{error::AppError, traits::types::CurrentUser},
    domain::{
        shared::media_type::MediaType,
        user::{display_name::DisplayName, error::UserError, model::UpdateUserPayload},
    },
    presentation::{
        error::ErrorResponse,
        request::user::{UserAvatarUpdateInput, UserUpdateInput},
        response::{
            auth_response::UserAuthDetailResponse, presigned_uri_response::PresignedUriResponse,
            user_response::CurrentUserResponse,
        },
    },
};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = UserAuthDetailResponse),
        (status = 401, body = ErrorResponse, example = json!({ "code": "UNAUTHORIZED", "message": "unauthorized" })),
        (status = 429, body = ErrorResponse, example = json!({ "code": "GLOBAL_RATE_LIMITED", "message": "too many requests" })),
        (status = 500, body = ErrorResponse, example = json!({ "code": "INTERNAL_ERROR", "message": "internal server error" })),
    )
)]
async fn get_me(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(StatusCode, Json<UserAuthDetailResponse>), AppError> {
    let result = state.user_service.get_user(current_user.id).await?;
    Ok((
        StatusCode::OK,
        Json(UserAuthDetailResponse::from_domain(
            result,
            state.cdn_base_url,
        )),
    ))
}

#[utoipa::path(
    patch,
    path = "/",
    request_body = UserUpdateInput,
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 400, body = ErrorResponse,
            examples(
                ("Invalid DisplayName" = (value = json!({ "code": "INVALID_DISPLAY_NAME", "message": "invalid display name" }))),
            )
        ),
        (status = 401, body = ErrorResponse, example = json!({ "code": "UNAUTHORIZED", "message": "unauthorized" })),
        (status = 429, body = ErrorResponse, example = json!({ "code": "GLOBAL_RATE_LIMITED", "message": "too many requests" })),
        (status = 500, body = ErrorResponse, example = json!({ "code": "INTERNAL_ERROR", "message": "internal server error" })),
    )
)]
async fn update_me(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(input): Json<UserUpdateInput>,
) -> Result<(StatusCode, Json<CurrentUserResponse>), AppError> {
    let update_user = UpdateUserPayload::default()
        .display_name(DisplayName::try_new(input.display_name).map_err(UserError::from)?);

    let result = state
        .user_service
        .update_user(current_user.id, update_user)
        .await?;
    Ok((
        StatusCode::OK,
        Json(CurrentUserResponse::from_domain(result, state.cdn_base_url)),
    ))
}

#[utoipa::path(
    post,
    path = "/avatar/presigned_uri",
    request_body = UserAvatarUpdateInput,
    responses(
        (status = 200, body = PresignedUriResponse),
        (status = 400, body = ErrorResponse, example = json!({ "code": "INVALID_MEDIA_TYPE", "message": "invalid media type" }) ),
        (status = 401, body = ErrorResponse, example = json!({ "code": "UNAUTHORIZED", "message": "unauthorized" })),
        (status = 429, body = ErrorResponse, example = json!({ "code": "GLOBAL_RATE_LIMITED", "message": "too many requests" })),
        (status = 500, body = ErrorResponse, example = json!({ "code": "INTERNAL_ERROR", "message": "internal server error" })),
    )
)]
async fn presigned_uri(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(input): Json<UserAvatarUpdateInput>,
) -> Result<(StatusCode, Json<PresignedUriResponse>), AppError> {
    let result = state
        .user_service
        .presigned_uri(current_user.id, MediaType::try_new(input.media_type)?)
        .await?;

    Ok((
        StatusCode::OK,
        Json(PresignedUriResponse {
            presigned_uri: result,
        }),
    ))
}

#[utoipa::path(
    patch,
    path = "/avatar",
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 401, body = ErrorResponse, example = json!({ "code": "UNAUTHORIZED", "message": "unauthorized" })),
        (status = 429, body = ErrorResponse, example = json!({ "code": "GLOBAL_RATE_LIMITED", "message": "too many requests" })),
        (status = 500, body = ErrorResponse, example = json!({ "code": "INTERNAL_ERROR", "message": "internal server error" })),
    )
)]
async fn update_avatar(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(StatusCode, Json<CurrentUserResponse>), AppError> {
    let result = state.user_service.update_avatar(current_user.id).await?;

    Ok((
        StatusCode::OK,
        Json(CurrentUserResponse::from_domain(result, state.cdn_base_url)),
    ))
}

pub fn create_user_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_me))
        .routes(routes!(update_me))
        .routes(routes!(presigned_uri))
        .routes(routes!(update_avatar))
}
