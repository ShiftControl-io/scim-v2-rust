use serde::{Deserialize, Serialize};

use crate::schema_urns;
use crate::utils::validation::{Validate, ValidationError, require_schema_urn};

/// Represents a SCIM HTTP Error.
///
/// This struct represents an error message that conforms to the SCIM
/// protocol.
///
/// The `schemas` field is a required array of strings. It holds the URI
/// `urn:ietf:params:scim:api:messages:2.0:Error`. The `scim_type` field is
/// an optional [`ScimType`] keyword from RFC 7644 §3.12. The `detail` field
/// is an optional string. It gives more detail for a human reader. The
/// `status` field is a required string. It holds the HTTP status code as a
/// JSON string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScimHttpError {
    pub schemas: Vec<String>,
    #[serde(rename = "scimType", skip_serializing_if = "Option::is_none")]
    pub scim_type: Option<ScimType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub status: String,
}

/// This enum models the `scimType` keywords that RFC 7644 §3.12 defines
/// for a 400-class error response.
///
/// This enum carries the `#[non_exhaustive]` attribute and the
/// [`Other`](ScimType::Other) catch-all variant. RFC 7644 defines ten
/// keywords. A non-conformant server may still send a label outside this
/// set. A rejection of the whole error body for an unknown label would
/// hide the error that body reports. Unknown labels round-trip verbatim.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String", into = "String")]
pub enum ScimType {
    /// The filter syntax is invalid, or the server does not support the
    /// attribute and comparison combination. This keyword applies to `GET`
    /// and `POST /.search`.
    InvalidFilter,
    /// The filter yields more results than the server will return.
    TooMany,
    /// One or more attribute values are already in use or reserved. This
    /// keyword applies to `POST`, `PUT` and `PATCH`.
    Uniqueness,
    /// The attempted modification does not fit the attribute's mutability
    /// or its current state. This keyword applies to `PUT` and `PATCH`.
    Mutability,
    /// The request body message structure is invalid, or it does not
    /// conform to the request schema. This keyword applies to
    /// `POST /.search` and `POST /Bulk`.
    InvalidSyntax,
    /// The PATCH path attribute is invalid or malformed.
    InvalidPath,
    /// The PATCH path does not yield an attribute or a value that the
    /// server can act on.
    NoTarget,
    /// A required value is missing, or the specified value does not fit
    /// the operation, the attribute type, or the resource schema.
    InvalidValue,
    /// The server does not support the specified SCIM protocol version.
    InvalidVers,
    /// The request would violate a policy, or it would expose sensitive
    /// information. The server cannot complete the request.
    Sensitive,
    /// This variant holds a keyword outside the ten that §3.12 defines.
    /// The crate keeps the keyword's text exactly as received.
    Other(String),
}

impl ScimType {
    /// This method returns the wire keyword.
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

/// Returns a default `ScimHttpError`.
///
/// This implementation sets the `schemas` field to a vector that holds the
/// string `"urn:ietf:params:scim:api:messages:2.0:Error"`. It sets the
/// `scim_type` and `detail` fields to `None`. It sets the `status` field
/// to an empty string.
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
    /// RFC 7644 §3.12 requires the Error URN in the `schemas` field. The
    /// `status` field holds the numeric HTTP status as a JSON string.
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
