use crate::application::traits::types::CurrentUser;
use crate::domain::authentication::email::Email;
use crate::domain::authentication::error::AuthenticationError;
use crate::presentation::error::ErrorResponse;
use crate::presentation::request::auth::{
    AuthGoogleInput, AuthRefreshInput, LogoutInput, SendCodeInput, UpdateEmailInput,
    VerifyCodeInput,
};
use crate::presentation::response::user_response::CurrentUserResponse;
use crate::presentation::response::verification_code_response::VerificationCodeResponse;
use crate::{
    AppState, application::error::AppError, presentation::response::auth_response::AuthResponse,
};
use axum::Extension;
use axum::{Json, extract::State, http::StatusCode};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    post,
    path = "/email/send-code",
    request_body = SendCodeInput,
    responses(
        (status = 200, body = VerificationCodeResponse),
        (status = 400, body = ErrorResponse, description = "invalid email"),
        (status = 429, body = ErrorResponse, description = "too many request"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn send_code(
    State(state): State<AppState>,
    Json(input): Json<SendCodeInput>,
) -> Result<(StatusCode, Json<VerificationCodeResponse>), AppError> {
    state
        .auth_service
        .send_verification_code(Email::try_new(input.email).map_err(AuthenticationError::from)?)
        .await?;

    Ok((
        StatusCode::OK,
        Json(VerificationCodeResponse::new(state.resend_cooldown_seconds)),
    ))
}

#[utoipa::path(
    post,
    path = "/email/verify-code",
    request_body = VerifyCodeInput,
    responses(
        (status = 200, body = AuthResponse),
        (status = 400, body = ErrorResponse, description = "invalid email"),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn verify_code(
    State(state): State<AppState>,
    Json(input): Json<VerifyCodeInput>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let result = state
        .auth_service
        .verify_code(
            Email::try_new(input.email).map_err(AuthenticationError::from)?,
            input.code,
            input.device_info.into(),
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse::from_domain(result, state.cdn_base_url)),
    ))
}

#[utoipa::path(
    patch,
    path = "/email",
    request_body = UpdateEmailInput,
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 400, body = ErrorResponse, description = "invalid email"),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn update_email(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(input): Json<UpdateEmailInput>,
) -> Result<(StatusCode, Json<CurrentUserResponse>), AppError> {
    let result = state
        .auth_service
        .upsert_email(
            current_user.id,
            Email::try_new(input.email).map_err(AuthenticationError::from)?,
            input.code,
        )
        .await?;

    Ok((
        StatusCode::OK,
        Json(CurrentUserResponse::from_domain(result, state.cdn_base_url)),
    ))
}

#[utoipa::path(
    post,
    path = "/google",
    request_body = AuthGoogleInput,
    responses(
        (status = 200, body = AuthResponse),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn auth_google(
    State(state): State<AppState>,
    Json(input): Json<AuthGoogleInput>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let result = state
        .auth_service
        .auth_google(input.id_token, input.device_info.into())
        .await?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse::from_domain(result, state.cdn_base_url)),
    ))
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = AuthRefreshInput,
    responses(
        (status = 200, body = AuthResponse),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn auth_refresh(
    State(state): State<AppState>,
    Json(input): Json<AuthRefreshInput>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let result = state
        .auth_service
        .auth_refresh(input.refresh_token.into())
        .await?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse::from_domain(result, state.cdn_base_url)),
    ))
}

#[utoipa::path(
    post,
    path = "/logout",
    request_body = LogoutInput,
    responses(
        (status = 204),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn logout(
    State(state): State<AppState>,
    Json(input): Json<LogoutInput>,
) -> Result<StatusCode, AppError> {
    state
        .auth_service
        .logout(input.refresh_token.into())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn create_public_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(send_code))
        .routes(routes!(verify_code))
        .routes(routes!(auth_google))
        .routes(routes!(auth_refresh))
        .routes(routes!(logout))
}

pub fn create_private_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(update_email))
}
