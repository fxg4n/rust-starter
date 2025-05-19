use serde::Serialize;
use std::collections::HashMap;
use crate::core::errors::types::ApiError;
use crate::core::types::pagination::PageResult;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Data(T),
    PagedData(PageResult<T>),
    Error {
        message: String,
        errors: Option<HashMap<String, String>>,
    }
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self::Data(data)
    }

    pub fn paginate(page_result: PageResult<T>) -> Self {
        Self::PagedData(page_result)
    }

    pub fn error(error: ApiError) -> Self {
        Self::Error {
            message: error.to_string(),
            errors: match &error {
                ApiError::ValidationError(msg) => {
                    let mut map = HashMap::new();
                    map.insert("validation".to_string(), msg.clone());
                    Some(map)
                }
                _ => None
            }
        }
    }
}

impl<T> From<Result<T, ApiError>> for ApiResponse<T> {
    fn from(result: Result<T, ApiError>) -> Self {
        match result {
            Ok(data) => Self::success(data),
            Err(error) => Self::error(error),
        }
    }
}

impl<T> From<Result<PageResult<T>, ApiError>> for ApiResponse<T> {
    fn from(result: Result<PageResult<T>, ApiError>) -> Self {
        match result {
            Ok(page_result) => Self::paginate(page_result),
            Err(error) => Self::error(error),
        }
    }
}