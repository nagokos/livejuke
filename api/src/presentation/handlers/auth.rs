use crate::{
    AppState,
    application::error::AppError,
    domain::{
        authentication::{email::Email, model::EmailCredentials},
        user::model::{NewUser, User},
    },
    presentation::response::user_response::CurrentUserResponse,
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(Deserialize, Clone, ToSchema)]
pub struct AuthEmailInput {
    display_name: String,
    #[schema(value_type = String)]
    email: Email,
    password: String,
}

impl AuthEmailInput {
    fn into_parts(self) -> (NewUser, EmailCredentials) {
        (
            NewUser {
                display_name: self.display_name,
            },
            EmailCredentials {
                email: self.email,
                password: self.password,
            },
        )
    }
}

#[utoipa::path(
    post,
    path = "/email",
    request_body = AuthEmailInput,
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
    Json(input): Json<AuthEmailInput>,
) -> Result<(StatusCode, CookieJar, Json<CurrentUserResponse>), AppError> {
    let (new_user, credentials) = input.into_parts();
    let (user, token): (User, String) = state
        .auth_service
        .register_by_email(new_user, credentials)
        .await?;

    let cookie = Cookie::build(("token", token))
        .domain(state.cookie_config.domain.clone())
        .secure(state.cookie_config.secure)
        .same_site(state.cookie_config.same_site);

    Ok((StatusCode::CREATED, jar.add(cookie), Json(user.into())))
}

pub fn create_auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(register_by_email))
}
