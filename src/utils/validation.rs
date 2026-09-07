//! Validation of SCIM resources against the required-attribute rules the
//! type system cannot express.
//!
//! Nearly every attribute in RFC 7643 is optional, so the models make almost
//! everything `Option` or defaulted. What `serde` therefore cannot enforce is
//! the handful of attributes the RFC marks REQUIRED. [`Validate`] carries
//! those checks, and reports failures using the **wire** attribute name
//! (`userName`, not `user_name`) so a server can echo the path straight back
//! in an RFC 7644 §3.12 error response.

use thiserror::Error;

#[cfg(feature = "models")]
use crate::models::errors::ScimHttpError;
#[cfg(feature = "models")]
use crate::schema_urns;

/// Why a resource failed validation.
///
/// `#[non_exhaustive]`: further checks (attribute-type and mutability
/// conformance, canonical-value membership) will add variants in minor
/// releases.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationErrorKind {
    /// An attribute RFC 7643 marks REQUIRED is absent, `null`, or empty.
    #[error("required attribute is missing or empty")]
    MissingRequiredAttribute,
    /// The attribute is present but its value is not permissible.
    #[error("{0}")]
    InvalidValue(String),
}

/// A single validation failure, located by its SCIM wire path.
///
/// `path` is dotted wire notation as it would appear in a filter or PATCH
/// path — `userName`, `name.familyName`, `meta.resourceType` — never the Rust
/// field name.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{path}: {kind}")]
pub struct ValidationError {
    path: String,
    kind: ValidationErrorKind,
}

impl ValidationError {
    /// A REQUIRED attribute was missing or empty.
    pub fn missing_required(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ValidationErrorKind::MissingRequiredAttribute,
        }
    }

    /// An attribute carried a value that is not permissible.
    pub fn invalid_value(path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ValidationErrorKind::InvalidValue(detail.into()),
        }
    }

    /// The SCIM wire path of the offending attribute.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Why validation failed.
    pub fn kind(&self) -> &ValidationErrorKind {
        &self.kind
    }

    /// The RFC 7644 §3.12 `scimType` keyword for this failure.
    ///
    /// Every current variant maps to `invalidValue`, which §3.12 defines as
    /// "A required value was missing, or the value specified was not
    /// compatible with the operation or attribute type ... or resource
    /// schema".
    pub fn scim_type(&self) -> &'static str {
        match self.kind {
            ValidationErrorKind::MissingRequiredAttribute
            | ValidationErrorKind::InvalidValue(_) => "invalidValue",
        }
    }

    /// Build the RFC 7644 §3.12 error body a server should return, at the
    /// given HTTP status (`"400"` for every current variant).
    ///
    /// Requires the `models` feature, which supplies [`ScimHttpError`].
    #[cfg(feature = "models")]
    pub fn to_http_error(&self, status: impl Into<String>) -> ScimHttpError {
        ScimHttpError {
            schemas: vec![schema_urns::ERROR.to_string()],
            scim_type: Some(self.scim_type().to_string()),
            detail: Some(self.to_string()),
            status: status.into(),
        }
    }
}

/// Check a resource against the attributes RFC 7643 marks REQUIRED.
///
/// Import the trait to call it: `use scim_v2::Validate;`
pub trait Validate {
    /// `Ok(())` when every REQUIRED attribute is present and permissible.
    fn validate(&self) -> Result<(), ValidationError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_required_reports_the_wire_path() {
        let err = ValidationError::missing_required("userName");
        assert_eq!(err.path(), "userName");
        assert_eq!(err.kind(), &ValidationErrorKind::MissingRequiredAttribute);
        assert_eq!(
            err.to_string(),
            "userName: required attribute is missing or empty"
        );
    }

    /// A nested attribute is reported in dotted wire notation, the same
    /// notation a filter or PATCH path uses, so a server can hand it straight
    /// back to the client.
    #[test]
    fn invalid_value_carries_its_detail_and_a_dotted_path() {
        let err = ValidationError::invalid_value("name.familyName", "must not be blank");
        assert_eq!(err.path(), "name.familyName");
        assert_eq!(
            err.kind(),
            &ValidationErrorKind::InvalidValue("must not be blank".to_string())
        );
        assert_eq!(err.to_string(), "name.familyName: must not be blank");
    }

    /// RFC 7644 §3.12: `invalidValue` covers "a required value was missing, or
    /// the value specified was not compatible with the operation or attribute
    /// type", which is both of the current kinds.
    #[test]
    fn both_kinds_map_to_invalid_value() {
        assert_eq!(
            ValidationError::missing_required("schemas").scim_type(),
            "invalidValue"
        );
        assert_eq!(
            ValidationError::invalid_value("x", "y").scim_type(),
            "invalidValue"
        );
    }

    /// The RFC 7644 §3.12 body a server returns. `status` is a JSON *string*
    /// per §3.12, not a number.
    #[cfg(feature = "models")]
    #[test]
    fn to_http_error_builds_the_rfc_error_body() {
        let err = ValidationError::missing_required("userName");
        let body = err.to_http_error("400");

        assert_eq!(body.schemas, vec![crate::schema_urns::ERROR]);
        assert_eq!(body.scim_type.as_deref(), Some("invalidValue"));
        assert_eq!(
            body.detail.as_deref(),
            Some("userName: required attribute is missing or empty")
        );
        assert_eq!(body.status, "400");

        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["scimType"], "invalidValue");
        assert_eq!(json["status"], "400", "status must serialize as a string");
    }

    /// `ValidationError` is comparable, which is what lets tests assert on a
    /// whole error rather than on its rendered string.
    #[test]
    fn errors_compare_by_path_and_kind() {
        assert_eq!(
            ValidationError::missing_required("a"),
            ValidationError::missing_required("a")
        );
        assert_ne!(
            ValidationError::missing_required("a"),
            ValidationError::missing_required("b")
        );
        assert_ne!(
            ValidationError::missing_required("a"),
            ValidationError::invalid_value("a", "detail")
        );
    }
}
