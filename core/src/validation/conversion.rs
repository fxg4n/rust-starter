use std::collections::HashMap;
use validator::{ValidationErrors, ValidationError as ValidatorError};
use crate::core::errors::types::ApiError;
use crate::core::validation::get_error_message;

pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl IntoApiError for ValidationErrors {
    fn into_api_error(self) -> ApiError {
        let mut error_map = HashMap::new();
        
        for (field, errors) in self.field_errors() {
            if let Some(first_error) = errors.first() {
                let message = if let Some(msg) = &first_error.message {
                    msg.to_string()
                } else {
                    get_error_message(
                        first_error.code.as_ref(),
                        field.as_ref()
                    )
                };
                error_map.insert(field.to_string(), message);
            }
        }

        let error_message = if error_map.len() == 1 {
            error_map.values().next().unwrap().clone()
        } else {
            "Multiple validation errors occurred".to_string()
        };

        ApiError::ValidationError(error_message)
    }
}

impl IntoApiError for ValidatorError {
    fn into_api_error(self) -> ApiError {
        ApiError::ValidationError(
            self.message
                .map(|cow| cow.to_string())
                .unwrap_or_else(|| "Validation error occurred".to_string())
        )
    }
}