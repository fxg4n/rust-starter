use axum::response::{IntoResponse, Response};
use crate::core::errors::types::ApiError;
use axum::http::StatusCode;
use serde_json::json;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ApiError::TokenExpired => StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
        };

        let body = json!({
            "error": {
                "status": status.as_u16(),
                "message": self.to_string()
            }
        });

        (status, axum::Json(body)).into_response()
    }
}
