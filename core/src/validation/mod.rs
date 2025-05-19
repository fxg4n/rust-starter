pub use validator::{Validate, ValidationError as ValidatorError, ValidationErrors};
pub mod format;
pub mod file;
pub mod conversion;
pub mod length;

pub use conversion::IntoApiError;
use crate::core::constants::validation::*;

fn get_error_message(code: &str, field: &str) -> String {
    match code {
        "required" => format!("The field '{}' is required", field),
        "password_too_short" => format!("Password must be at least {} characters", PASSWORD_MIN_LENGTH),
        "password_too_long" => format!("Password must not exceed {} characters", PASSWORD_MAX_LENGTH),
        "username_too_short" => format!("Username must be at least {} characters", USERNAME_MIN_LENGTH),
        "username_too_long" => format!("Username must not exceed {} characters", USERNAME_MAX_LENGTH),
        "email_too_long" => format!("Email must not exceed {} characters", EMAIL_MAX_LENGTH),
        "name_too_long" => format!("Name must not exceed {} characters", NAME_MAX_LENGTH),
        "description_too_long" => format!("Description must not exceed {} characters", DESCRIPTION_MAX_LENGTH),
        "title_too_long" => format!("Title must not exceed {} characters", TITLE_MAX_LENGTH),
        "too_many_tags" => format!("Cannot have more than {} tags", MAX_TAGS),
        "tag_too_long" => format!("Tag must not exceed {} characters", TAG_MAX_LENGTH),
        "password_requirements_not_met" => "Password must contain uppercase, lowercase, number, and special character".to_string(),
        "invalid_phone_number" => "Invalid phone number format".to_string(),
        "invalid_url" => "Invalid URL format".to_string(),
        "invalid_date_format" => "Date must be in YYYY-MM-DD format".to_string(),
        "invalid_hex_color" => "Invalid hex color code".to_string(),
        "invalid_ip_address" => "Invalid IP address".to_string(),
        "invalid_slug" => "Invalid slug format".to_string(),
        "file_too_large" => format!("File size exceeds maximum allowed for field '{}'", field),
        "no_file_extension" => "File must have an extension".to_string(),
        _ => format!("Validation failed for field '{}'", field),
    }
}