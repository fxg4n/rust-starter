use validator::ValidationError as ValidatorError;
use regex::Regex;
use crate::core::constants::validation::{
    PASSWORD_MIN_LENGTH,
    PASSWORD_MAX_LENGTH,
    EMAIL_MAX_LENGTH,
    USERNAME_MIN_LENGTH,
    USERNAME_MAX_LENGTH,
};

pub fn validate_url(url: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^https?://[\w\-\.]+(:\d+)?(/[\w\-\./%\+@&#=\(\)]*)?$").unwrap();
    if !re.is_match(url) {
        return Err(ValidatorError::new("invalid_url"));
    }
    Ok(())
}

pub fn validate_date_format(date: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !re.is_match(date) {
        return Err(ValidatorError::new("invalid_date_format"));
    }
    Ok(())
}

pub fn validate_hex_color(color: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    if !re.is_match(color) {
        return Err(ValidatorError::new("invalid_hex_color"));
    }
    Ok(())
}

pub fn validate_ip_address(ip: &str) -> Result<(), ValidatorError> {
    let ipv4_re = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    let ipv6_re = Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();
    
    if !ipv4_re.is_match(ip) && !ipv6_re.is_match(ip) {
        return Err(ValidatorError::new("invalid_ip_address"));
    }
    Ok(())
}

pub fn validate_slug(slug: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    if !re.is_match(slug) {
        return Err(ValidatorError::new("invalid_slug"));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidatorError> {
    if password.len() < PASSWORD_MIN_LENGTH {
        return Err(ValidatorError::new("password_too_short"));
    }
    if password.len() > PASSWORD_MAX_LENGTH {
        return Err(ValidatorError::new("password_too_long"));
    }
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(ValidatorError::new("password_requirements_not_met"));
    }

    Ok(())
}

pub fn validate_phone_number(phone: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    if !re.is_match(phone) {
        return Err(ValidatorError::new("invalid_phone_number"));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    
    if email.len() > EMAIL_MAX_LENGTH {
        return Err(ValidatorError::new("email_too_long"));
    }
    
    if !re.is_match(email) {
        return Err(ValidatorError::new("invalid_email"));
    }
    
    if !email.contains('@') || !email.contains('.') {
        return Err(ValidatorError::new("invalid_email"));
    }
    
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), ValidatorError> {
    if username.len() < USERNAME_MIN_LENGTH {
        return Err(ValidatorError::new("username_too_short"));
    }
    if username.len() > USERNAME_MAX_LENGTH {
        return Err(ValidatorError::new("username_too_long"));
    }
    
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !re.is_match(username) {
        return Err(ValidatorError::new("invalid_username_format"));
    }
    Ok(())
}