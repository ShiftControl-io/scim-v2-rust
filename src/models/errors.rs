use serde::{Deserialize, Serialize};

use crate::schema_urns;

/// Represents a SCIM HTTP Error.
///
/// This struct is used to represent an error message that conforms to the SCIM protocol specification.
/// The `schemas` field is a required array of strings containing the URI `urn:ietf:params:scim:api:messages:2.0:Error`.
/// The `scim_type` field is an optional string that indicates a specification-defined SCIM error keyword.
/// The `detail` field is an optional string that provides more detailed human-readable information.
/// The `status` field is a required string that is the HTTP status code expressed as a JSON string.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScimHttpError {
    pub schemas: Vec<String>,
    #[serde(rename = "scimType", skip_serializing_if = "Option::is_none")]
    pub scim_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub status: String,
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

#[cfg(test)]
mod tests {
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
        assert_eq!(error.scim_type, Some("invalidValue".to_string()));
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
            scim_type: Some("invalidValue".to_string()),
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
