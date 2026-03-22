use crate::domain::authentication::model::EmailCredentials;
use crate::domain::user::model::NewUser;
use crate::presentation::request::auth::{LoginEmailInput, RegisterEmailInput};
use crate::{
    AppState, application::error::AppError,
    presentation::response::user_response::CurrentUserResponse,
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    post,
    path = "/register/email",
    request_body = RegisterEmailInput,
    responses(
        (status = 201, body = CurrentUserResponse),
        (status = 400, description = "invalid email or password"),
        (status = 409, description = "email already exists"),
        (status = 500, description = "internal server error"),
    )
)]
async fn register_by_email(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(input): Json<RegisterEmailInput>,
) -> Result<(StatusCode, CookieJar, Json<CurrentUserResponse>), AppError> {
    let new_user = NewUser::try_new(input.display_name)?;
    let credentials = EmailCredentials::try_new(input.email, input.password)?;
    let (user, token) = state
        .auth_service
        .register_by_email(new_user, credentials)
        .await?;

    let cookie = Cookie::build(("token", token))
        .domain(state.cookie_config.domain.clone())
        .secure(state.cookie_config.secure)
        .same_site(state.cookie_config.same_site);

    Ok((StatusCode::CREATED, jar.add(cookie), Json(user.into())))
}

#[utoipa::path(
    post,
    path = "/login/email",
    request_body = LoginEmailInput,
    responses(
        (status = 200, body = CurrentUserResponse),
        (status = 401, description = "unauthorized error"),
        (status = 500, description = "internal server error"),
    )
)]
async fn login_by_email(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(input): Json<LoginEmailInput>,
) -> Result<(StatusCode, CookieJar, Json<CurrentUserResponse>), AppError> {
    let credentials = EmailCredentials::try_new(input.email, input.password)?;
    let (user, token) = state.auth_service.login_by_email(credentials).await?;

    let cookie = Cookie::build(("token", token))
        .domain(state.cookie_config.domain.clone())
        .secure(state.cookie_config.secure)
        .same_site(state.cookie_config.same_site);

    Ok((StatusCode::OK, jar.add(cookie), Json(user.into())))
}

pub fn create_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_by_email))
        .routes(routes!(login_by_email))
}
