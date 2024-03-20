use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SCIMError {
    ConflictError(String),
    DeserializationError(serde_json::Error),
    InvalidFieldValue(String),
    InvalidJsonFormat,
    MissingRequiredField(String),
    OtherError(String),
    RequestError(String),
    ResourceTypeNotFound(String),
    SchemaNotFound(String),
    SerializationError(serde_json::Error),
}

impl Display for SCIMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SCIMError::ConflictError(msg) => write!(f, "Conflict error: {}", msg),
            SCIMError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            SCIMError::InvalidFieldValue(field) => write!(f, "Invalid field value: {}", field),
            SCIMError::InvalidJsonFormat => write!(f, "Invalid JSON format"),
            SCIMError::MissingRequiredField(field) => write!(f, "Missing required field: {}", field),
            SCIMError::OtherError(msg) => write!(f, "Other Error: {}", msg),
            SCIMError::RequestError(msg) => write!(f, "Request error: {}", msg),
            SCIMError::ResourceTypeNotFound(field) => write!(f, "Resource type not found: {}", field),
            SCIMError::SchemaNotFound(field) => write!(f, "Schema not found: {}", field),
            SCIMError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<serde_json::Error> for SCIMError {
    fn from(err: serde_json::Error) -> SCIMError {
        SCIMError::DeserializationError(err)
    }
}