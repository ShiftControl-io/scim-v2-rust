use crate::utils::validation::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EnterpriseUser {
    #[serde(rename = "employeeNumber", skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(rename = "costCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<Manager>,
}

/// Converts a JSON string into a `EnterpriseUser` struct.
///
/// This method attempts to parse a JSON string to construct a `EnterpriseUser` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `EnterpriseUser` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::enterprise_user::EnterpriseUser;
///
/// let ent_user_json = r#"{
///             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
///             "id": "2819c223-7f76-453a-919d-413861904646",
///             "userName": "bjensen@example.com"
///         }"#;
///
/// match EnterpriseUser::try_from(ent_user_json) {
///     Ok(user) => println!("Successfully converted JSON to EnterpriseUser: {:?}", user),
///     Err(e) => println!("Error converting from JSON to EnterpriseUser: {}", e),
/// }
/// ```
impl TryFrom<&str> for EnterpriseUser {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl EnterpriseUser {}

/// The user's manager.
///
/// Unassigned fields are omitted from serialized output rather than emitted as
/// `null`; RFC 7643 §2.5 treats the two as equivalent in resource state. Note
/// the consequence for `PUT`: per RFC 7644 §3.5.1 an omitted `readWrite`
/// attribute (`value`, `$ref`) is "not asserted by the client" and the server
/// MAY keep the existing value or apply a default — it is *not* a deterministic
/// clear. Callers that need to clear a field on `PUT` should use a `PATCH`
/// operation or hand-build a `serde_json::Value` carrying an explicit `null`.
/// (`displayName` is `readOnly`, so §3.5.1 says the server SHALL ignore it.)
#[derive(Serialize, Deserialize, Debug)]
pub struct Manager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Validate for EnterpriseUser {
    /// Always `Ok`. RFC 7643 §4.3 defines no REQUIRED attribute on the
    /// enterprise User extension: `employeeNumber`, `costCenter`,
    /// `organization`, `division`, `department` and `manager` are all
    /// `required: false` in the schema the crate embeds
    /// (`src/schemas/enterprise_user.json`).
    ///
    /// Before 1.0 this demanded all six and so rejected every conformant
    /// `EnterpriseUser` that left any of them unset. The impl is kept, rather
    /// than dropped, so the trait is uniform across resources and a future
    /// attribute-level check has somewhere to live.
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validate;

    /// Regression guard for the pre-1.0 bug. Every attribute in RFC 7643 §4.3
    /// is `required: false` in the schema this crate embeds, so a wholly empty
    /// extension is conformant. `validate` used to demand all six.
    #[test]
    fn an_empty_extension_is_valid() {
        assert!(EnterpriseUser::default().validate().is_ok());
    }

    /// A partially populated extension is the common real case — a provider
    /// that tracks a department but no cost centre.
    #[test]
    fn a_partial_extension_is_valid() {
        let partial = EnterpriseUser {
            department: Some("Engineering".to_string()),
            ..Default::default()
        };
        assert!(partial.validate().is_ok());
    }

    /// RFC 7643 §4.3's own example, carrying every attribute including the
    /// nested `manager`.
    #[test]
    fn full_extension_round_trips() {
        let raw = r#"{
            "employeeNumber": "701984",
            "costCenter": "4130",
            "organization": "Universal Studios",
            "division": "Theme Park",
            "department": "Tour Operations",
            "manager": {
                "value": "26118915-6090-4610-87e4-49d8ca9f808d",
                "$ref": "../Users/26118915-6090-4610-87e4-49d8ca9f808d",
                "displayName": "John Smith"
            }
        }"#;

        let eu: EnterpriseUser = serde_json::from_str(raw).expect("§4.3 example must deserialize");
        assert_eq!(eu.employee_number.as_deref(), Some("701984"));
        assert_eq!(eu.division.as_deref(), Some("Theme Park"));
        let manager = eu.manager.as_ref().expect("manager must deserialize");
        assert_eq!(manager.display_name.as_deref(), Some("John Smith"));
        assert!(eu.validate().is_ok());

        let back: serde_json::Value = serde_json::to_value(&eu).unwrap();
        let original: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert_eq!(back, original, "no §4.3 attribute may be dropped");
    }

    /// An unassigned extension serializes to an empty object rather than one
    /// padded with nulls, per RFC 7643 §2.5.
    #[test]
    fn default_serializes_to_an_empty_object() {
        assert_eq!(
            serde_json::to_value(EnterpriseUser::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
