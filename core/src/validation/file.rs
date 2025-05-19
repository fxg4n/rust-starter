use validator::ValidationError as ValidatorError;

pub fn validate_file_size(size: u64, max_size: u64) -> Result<(), ValidatorError> {
    if size > max_size {
        return Err(ValidatorError::new("file_too_large"));
    }
    Ok(())
}

pub fn validate_file_extension(filename: &str, allowed: &[&str]) -> Result<(), ValidatorError> {
    if let Some(ext) = filename.split('.').last() {
        if !allowed.contains(&ext.to_lowercase().as_str()) {
            return Err(ValidatorError::new("invalid_file_extension"));
        }
        Ok(())
    } else {
        Err(ValidatorError::new("no_file_extension"))
    }
}