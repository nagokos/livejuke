use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;

use crate::{AppState, domain::models::user::NewEmailUser};

#[derive(Deserialize)]
pub struct AuthEmailInput {
    display_name: String,
    email: String,
    password: String,
}

async fn auth_email_handler(State(state): State<AppState>, Json(input): Json<AuthEmailInput>) {
    state.auth_service.register_by_email(&input.into()).await;
}

impl From<AuthEmailInput> for NewEmailUser {
    fn from(value: AuthEmailInput) -> Self {
        Self {
            display_name: value.display_name,
            email: value.email,
            password: value.password,
        }
    }
}

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/auth/email", post(auth_email_handler))
}
