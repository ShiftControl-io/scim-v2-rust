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
