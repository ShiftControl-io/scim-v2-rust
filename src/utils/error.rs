use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SCIMError {
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    InvalidJsonFormat,
    MissingRequiredField(String),
    InvalidFieldValue(String),
    SchemaNotFound(String),
    ResourceTypeNotFound(String),
    RequestError(String),
    OtherError(String),
}


impl Display for SCIMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SCIMError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            SCIMError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            SCIMError::InvalidJsonFormat => write!(f, "Invalid JSON format"),
            SCIMError::MissingRequiredField(field) => write!(f, "Missing required field: {}", field),
            SCIMError::InvalidFieldValue(field) => write!(f, "Invalid field value: {}", field),
            SCIMError::SchemaNotFound(field) => write!(f, "Schema not found: {}", field),
            SCIMError::ResourceTypeNotFound(field) => write!(f, "Resource type not found: {}", field),
            SCIMError::RequestError(msg) => write!(f, "Request error: {}", msg),
            SCIMError::OtherError(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl From<serde_json::Error> for SCIMError {
    fn from(err: serde_json::Error) -> SCIMError {
        SCIMError::DeserializationError(err)
    }
}