use axum::{
    Json,
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    AppState,
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

    let Some(token) = token else {
        return Err(unauthorized_response(ErrorCode::Unauthorized));
    };

    if let Ok(current_user) = state.access_token_provider.verify(token) {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(unauthorized_response(ErrorCode::InvalidAccessToken))
    }
}

fn unauthorized_response(code: ErrorCode) -> Response {
    let error_response = ErrorResponse {
        code,
        message: "unauthorized".to_string(),
    };
    (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
}
