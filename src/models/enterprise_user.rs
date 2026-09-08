use crate::utils::validation::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
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
mod tests;
