use crate::domain::authentication::email::Email;
use crate::domain::authentication::error::AuthenticationError;
use crate::presentation::error::ErrorResponse;
use crate::presentation::request::auth::{AuthGoogleInput, SendCodeInput, VerifyCodeInput};
use crate::presentation::response::verificatin_code_response::VerificationCodeResponse;
use crate::{
    AppState, application::error::AppError, presentation::response::auth_response::AuthResponse,
};
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
        .send_verification_code(Email::try_new(input.email).map_err(AuthenticationError::Email)?)
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
            Email::try_new(input.email).map_err(AuthenticationError::Email)?,
            input.code,
            input.device_info.into(),
        )
        .await?;

    Ok((StatusCode::OK, Json(result.into())))
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

    Ok((StatusCode::OK, Json(result.into())))
}

pub fn create_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(send_code))
        .routes(routes!(verify_code))
        .routes(routes!(auth_google))
}
