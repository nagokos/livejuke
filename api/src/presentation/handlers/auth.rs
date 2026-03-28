use crate::domain::authentication::model::EmailCredentials;
use crate::domain::session::model::DeviceInfo;
use crate::domain::user::model::NewUser;
use crate::presentation::error::ErrorResponse;
use crate::presentation::request::auth::{AuthGoogleInput, LoginEmailInput, RegisterEmailInput};
use crate::{
    AppState, application::error::AppError, presentation::response::auth_response::AuthResponse,
};
use axum::{Json, extract::State, http::StatusCode};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    post,
    path = "/register/email",
    request_body = RegisterEmailInput,
    responses(
        (status = 201, body = AuthResponse),
        (status = 400, body = ErrorResponse, description = "invalid email or password"),
        (status = 409, body = ErrorResponse, description = "email already exists"),
        (status = 500, body = ErrorResponse,description = "internal server error"),
    )
)]
async fn register_by_email(
    State(state): State<AppState>,
    Json(input): Json<RegisterEmailInput>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let new_user = NewUser::try_new(input.display_name, input.email.clone())?;
    let credentials = EmailCredentials::try_new(input.email, input.password)?;
    let device_info = DeviceInfo::new(
        input.device_info.device_name,
        input.device_info.model_name,
        input.device_info.os,
    );
    let result = state
        .auth_service
        .register_by_email(new_user, credentials, device_info)
        .await?;

    Ok((StatusCode::CREATED, Json(result.into())))
}

#[utoipa::path(
    post,
    path = "/login/email",
    request_body = LoginEmailInput,
    responses(
        (status = 200, body = AuthResponse),
        (status = 401, body = ErrorResponse, description = "unauthorized error"),
        (status = 500, body = ErrorResponse, description = "internal server error"),
    )
)]
async fn login_by_email(
    State(state): State<AppState>,
    Json(input): Json<LoginEmailInput>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let credentials = EmailCredentials::try_new(input.email, input.password)?;
    let device_info = DeviceInfo::new(
        input.device_info.device_name,
        input.device_info.model_name,
        input.device_info.os,
    );

    let result = state
        .auth_service
        .login_by_email(credentials, device_info)
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
    let device_info = DeviceInfo::new(
        input.device_info.device_name,
        input.device_info.model_name,
        input.device_info.os,
    );

    let result = state
        .auth_service
        .auth_google(input.id_token, device_info)
        .await?;

    Ok((StatusCode::OK, Json(result.into())))
}

pub fn create_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_by_email))
        .routes(routes!(login_by_email))
        .routes(routes!(auth_google))
}
