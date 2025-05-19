use validator::ValidationError as ValidatorError;
use crate::core::constants::validation::*;

pub fn validate_name_length(name: &str) -> Result<(), ValidatorError> {
    if name.len() > NAME_MAX_LENGTH {
        return Err(ValidatorError::new("name_too_long"));
    }
    Ok(())
}

pub fn validate_description_length(description: &str) -> Result<(), ValidatorError> {
    if description.len() > DESCRIPTION_MAX_LENGTH {
        return Err(ValidatorError::new("description_too_long"));
    }
    Ok(())
}

pub fn validate_title_length(title: &str) -> Result<(), ValidatorError> {
    if title.len() > TITLE_MAX_LENGTH {
        return Err(ValidatorError::new("title_too_long"));
    }
    Ok(())
}

pub fn validate_tags(tags: &[String]) -> Result<(), ValidatorError> {
    if tags.len() > MAX_TAGS {
        return Err(ValidatorError::new("too_many_tags"));
    }
    
    for tag in tags {
        if tag.len() > TAG_MAX_LENGTH {
            return Err(ValidatorError::new("tag_too_long"));
        }
    }
    Ok(())
}