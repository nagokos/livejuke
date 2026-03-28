use axum::{
    Json,
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    AppState,
    application::traits::access_token_provider::AccessTokenProvider,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(token) => token,
        None => return Err(unauthorized_response()),
    };

    if let Ok(current_user) = state.access_token_provider.verify(token) {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(unauthorized_response())
    }
}

fn unauthorized_response() -> Response {
    let error_response = ErrorResponse {
        code: ErrorCode::Unauthorized,
        message: "unauthorized".to_string(),
    };
    (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
}
