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
mod tests {

    /// The ten §3.12 keywords round-trip as typed variants; an unknown
    /// keyword survives as `Other` rather than failing the error body.
    #[test]
    fn scim_type_round_trips_every_keyword_and_preserves_unknowns() {
        for (wire, variant) in [
            ("invalidFilter", ScimType::InvalidFilter),
            ("tooMany", ScimType::TooMany),
            ("uniqueness", ScimType::Uniqueness),
            ("mutability", ScimType::Mutability),
            ("invalidSyntax", ScimType::InvalidSyntax),
            ("invalidPath", ScimType::InvalidPath),
            ("noTarget", ScimType::NoTarget),
            ("invalidValue", ScimType::InvalidValue),
            ("invalidVers", ScimType::InvalidVers),
            ("sensitive", ScimType::Sensitive),
        ] {
            let parsed: ScimType = serde_json::from_str(&format!("\"{wire}\"")).unwrap();
            assert_eq!(parsed, variant, "{wire}");
            assert_eq!(
                serde_json::to_string(&variant).unwrap(),
                format!("\"{wire}\"")
            );
        }
        let unknown: ScimType = serde_json::from_str("\"vendorSpecific\"").unwrap();
        assert_eq!(unknown, ScimType::Other("vendorSpecific".to_string()));
        assert_eq!(unknown.as_str(), "vendorSpecific");
    }

    /// RFC 7644 §3.12: the Error URN and a numeric status rendered as a string.
    #[test]
    fn validate_checks_urn_and_status() {
        let good = ScimHttpError {
            status: "400".to_string(),
            ..Default::default()
        };
        assert!(good.validate().is_ok());
        let bad_status = ScimHttpError {
            status: "400 Bad Request".to_string(),
            ..Default::default()
        };
        assert_eq!(
            bad_status.validate().expect_err("not a code").path(),
            "status"
        );
        let bad_urn = ScimHttpError {
            schemas: vec![schema_urns::USER.to_string()],
            status: "400".to_string(),
            ..Default::default()
        };
        assert_eq!(bad_urn.validate().expect_err("wrong URN").path(), "schemas");
    }
    use serde_json::json;

    use super::*;

    #[test]
    fn scim_http_error_default_creates_expected_error() {
        let error = ScimHttpError::default();
        assert_eq!(
            error.schemas,
            vec!["urn:ietf:params:scim:api:messages:2.0:Error".to_string()]
        );
        assert_eq!(error.scim_type, None);
        assert_eq!(error.detail, None);
        assert_eq!(error.status, "".to_string());
    }

    #[test]
    fn scim_http_error_deserialize_from_valid_json() {
        let json = json!({
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:Error"],
            "scimType": "invalidValue",
            "detail": "Invalid email address",
            "status": "400"
        });

        let error: ScimHttpError = serde_json::from_value(json).unwrap();
        assert_eq!(
            error.schemas,
            vec!["urn:ietf:params:scim:api:messages:2.0:Error".to_string()]
        );
        assert_eq!(error.scim_type, Some(ScimType::InvalidValue));
        assert_eq!(error.detail, Some("Invalid email address".to_string()));
        assert_eq!(error.status, "400".to_string());
    }

    #[test]
    fn scim_http_error_deserialize_from_invalid_json() {
        let json = json!({
        "status": "400"
         });

        let error: Result<ScimHttpError, _> = serde_json::from_value(json);
        assert!(error.is_err());
    }

    /// RFC 7644 §3.6 — unnumbered example, the 404 body from "Client's attempt
    /// to retrieve the previously deleted User". The error body returned for a
    /// DELETE of a resource that does not exist. Verbatim from the RFC;
    /// exercises an `Error` payload that carries `detail` and `status` but no
    /// `scimType`.
    #[test]
    fn rfc7644_s3_6_error_response_round_trips() {
        let raw = include_str!("../test_data/rfc7644/s3.6_error_response.json");
        let err: ScimHttpError =
            serde_json::from_str(raw).expect("RFC 7644 §3.6 error body must deserialize");
        assert_eq!(err.status, "404");
        assert_eq!(
            err.detail.as_deref(),
            Some("Resource 2819c223-7f76-453a-919d-413861904646 not found")
        );
        assert_eq!(err.scim_type, None);

        let reserialized: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&err).unwrap()).unwrap();
        let original: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert_eq!(reserialized, original);
    }

    #[test]
    fn scim_http_error_serialize_to_json() {
        let error = ScimHttpError {
            schemas: vec![schema_urns::ERROR.to_string()],
            scim_type: Some(ScimType::InvalidValue),
            detail: Some("Invalid email address".to_string()),
            status: "400".to_string(),
        };

        let json = serde_json::to_value(&error).unwrap();
        assert_eq!(
            json,
            json!({
                "schemas": ["urn:ietf:params:scim:api:messages:2.0:Error"],
                "scimType": "invalidValue",
                "detail": "Invalid email address",
                "status": "400"
            })
        );
    }
}
