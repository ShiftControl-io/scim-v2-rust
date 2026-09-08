use serde::{Deserialize, Serialize};

use crate::schema_urns;
use crate::utils::validation::{Validate, ValidationError, require_schema_urn};

/// Represents a SCIM HTTP Error.
///
/// This struct is used to represent an error message that conforms to the SCIM protocol specification.
/// The `schemas` field is a required array of strings containing the URI `urn:ietf:params:scim:api:messages:2.0:Error`.
/// The `scim_type` field is an optional [`ScimType`] keyword from RFC 7644 §3.12.
/// The `detail` field is an optional string that provides more detailed human-readable information.
/// The `status` field is a required string that is the HTTP status code expressed as a JSON string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScimHttpError {
    pub schemas: Vec<String>,
    #[serde(rename = "scimType", skip_serializing_if = "Option::is_none")]
    pub scim_type: Option<ScimType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub status: String,
}

/// The `scimType` keywords RFC 7644 §3.12 defines for a 400-class error
/// response.
///
/// `#[non_exhaustive]`, with an [`Other`](ScimType::Other) catch-all: the ten
/// keywords are the defined set, but a non-conformant server may send a label
/// outside it, and rejecting the whole error body for that would hide the
/// error it was reporting. Unknown labels round-trip verbatim.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String", into = "String")]
pub enum ScimType {
    /// The specified filter syntax was invalid, or the attribute and
    /// comparison combination is not supported. `GET`, `POST /.search`.
    InvalidFilter,
    /// The filter yields more results than the server is willing to return.
    TooMany,
    /// One or more attribute values are already in use or reserved. `POST`,
    /// `PUT`, `PATCH`.
    Uniqueness,
    /// The attempted modification is not compatible with the attribute's
    /// mutability or current state. `PUT`, `PATCH`.
    Mutability,
    /// The request body message structure was invalid or did not conform to
    /// the request schema. `POST /.search`, `POST /Bulk`.
    InvalidSyntax,
    /// The PATCH path attribute was invalid or malformed.
    InvalidPath,
    /// The PATCH path did not yield an attribute or value that could be
    /// operated on.
    NoTarget,
    /// A required value was missing, or the value specified was not
    /// compatible with the operation, attribute type, or resource schema.
    InvalidValue,
    /// The specified SCIM protocol version is not supported.
    InvalidVers,
    /// The request cannot be completed because it would violate a policy or
    /// expose sensitive information.
    Sensitive,
    /// A keyword outside the ten §3.12 defines, preserved verbatim.
    Other(String),
}

impl ScimType {
    /// The wire keyword.
    pub fn as_str(&self) -> &str {
        match self {
            ScimType::InvalidFilter => "invalidFilter",
            ScimType::TooMany => "tooMany",
            ScimType::Uniqueness => "uniqueness",
            ScimType::Mutability => "mutability",
            ScimType::InvalidSyntax => "invalidSyntax",
            ScimType::InvalidPath => "invalidPath",
            ScimType::NoTarget => "noTarget",
            ScimType::InvalidValue => "invalidValue",
            ScimType::InvalidVers => "invalidVers",
            ScimType::Sensitive => "sensitive",
            ScimType::Other(s) => s,
        }
    }
}

impl std::fmt::Display for ScimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<String> for ScimType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "invalidFilter" => ScimType::InvalidFilter,
            "tooMany" => ScimType::TooMany,
            "uniqueness" => ScimType::Uniqueness,
            "mutability" => ScimType::Mutability,
            "invalidSyntax" => ScimType::InvalidSyntax,
            "invalidPath" => ScimType::InvalidPath,
            "noTarget" => ScimType::NoTarget,
            "invalidValue" => ScimType::InvalidValue,
            "invalidVers" => ScimType::InvalidVers,
            "sensitive" => ScimType::Sensitive,
            _ => ScimType::Other(s),
        }
    }
}

impl From<ScimType> for String {
    fn from(t: ScimType) -> Self {
        t.as_str().to_string()
    }
}

/// Provides a default value for `ScimHttpError`.
///
/// This implementation of the `Default` trait provides a default value for `ScimHttpError`.
/// The `schemas` field is set to a vector containing the string "urn:ietf:params:scim:api:messages:2.0:Error".
/// The `scim_type` and `detail` fields are set to `None`.
/// The `status` field is set to an empty string.
impl Default for ScimHttpError {
    fn default() -> Self {
        ScimHttpError {
            schemas: vec![schema_urns::ERROR.to_string()],
            scim_type: None,
            detail: None,
            status: "".to_string(),
        }
    }
}

impl Validate for ScimHttpError {
    /// RFC 7644 §3.12: `schemas` carries the Error URN and `status` is the
    /// numeric HTTP status rendered as a JSON string.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::ERROR)?;
        match self.status.parse::<u16>() {
            Ok(code) if (100..=599).contains(&code) => Ok(()),
            _ => Err(ValidationError::invalid_value(
                "status",
                format!(
                    "must be an HTTP status code rendered as a string, got {:?}",
                    self.status
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests;
