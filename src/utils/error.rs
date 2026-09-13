use thiserror::Error;

/// This enum defines the errors that this crate's parsing, serialization
/// and lookup paths return.
///
/// This enum carries the `#[non_exhaustive]` attribute. RFC 7644 §3.12
/// defines SCIM detail error keywords that this crate does not yet model.
/// New variants can appear in a minor release. Add a trailing `_ =>` arm to
/// every match on this enum.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SCIMError {
    #[error("Conflict error: {0}")]
    ConflictError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Invalid field value: {0}")]
    InvalidFieldValue(String),
    #[error("Invalid JSON format")]
    InvalidJsonFormat,
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),
    #[error("Not found error: {0}")]
    NotFoundError(String),
    #[error("Other Error: {0}")]
    OtherError(String),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Resource type not found: {0}")]
    ResourceTypeNotFound(String),
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[source] serde_json::Error),
}
