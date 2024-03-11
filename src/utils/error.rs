use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SCIMError {
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    InvalidJsonFormat,
    MissingRequiredField(String),
    InvalidFieldValue(String),
}


impl Display for SCIMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SCIMError::SerializationError(e) => write!(f, "Serialization error: {}", e.to_string()),
            SCIMError::DeserializationError(e) => write!(f, "Deserialization error: {}", e.to_string()),
            SCIMError::InvalidJsonFormat => write!(f, "Invalid JSON format"),
            SCIMError::MissingRequiredField(field) => write!(f, "Missing required field: {}", field),
            SCIMError::InvalidFieldValue(field) => write!(f, "Invalid field value: {}", field),
        }
    }
}
