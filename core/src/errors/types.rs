use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    ValidationError(String),
    InternalError(String),
    Forbidden(String),
    BadRequest(String),
    Conflict(String),
    RateLimitExceeded,
    DatabaseError(String),
    ExternalServiceError(String),
    InvalidCredentials,
    TokenExpired,
    InvalidToken(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal server error: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            ApiError::InvalidCredentials => write!(f, "Invalid credentials"),
            ApiError::TokenExpired => write!(f, "Token has expired"),
            ApiError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
