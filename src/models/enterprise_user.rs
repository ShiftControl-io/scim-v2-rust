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

/// Converts a JSON string into an `EnterpriseUser` struct.
///
/// This method parses a JSON string into an `EnterpriseUser` object. Use
/// this method when you receive a JSON representation of a user from an
/// external source, for example a web request. This method gives you a
/// strongly typed object for use in your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if this method cannot parse
/// the JSON string into an `EnterpriseUser` object.
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
/// This struct omits an unassigned field from serialized output instead of
/// writing `null`. RFC 7643 §2.5 treats the two forms as equivalent in
/// resource state.
///
/// This omission has a consequence for `PUT`. Per RFC 7644 §3.5.1, an
/// omitted `readWrite` attribute (`value`, `$ref`) is "not asserted by the
/// client". The server MAY assume that the existing values are to be
/// cleared. The server MAY assign a default value instead. An omission is
/// therefore *not* a deterministic clear. A caller that needs to clear a
/// field on `PUT` should use a `PATCH` operation instead. A caller can
/// also hand-build a `serde_json::Value` that carries an explicit `null`.
///
/// `displayName` is `readOnly`. RFC 7644 §3.5.1 therefore says the server
/// SHALL ignore it.
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
    /// This method always returns `Ok`.
    ///
    /// RFC 7643 §4.3 defines no REQUIRED attribute on the enterprise User
    /// extension. The schema this crate embeds
    /// (`src/schemas/enterprise_user.json`) marks `employeeNumber`,
    /// `costCenter`, `organization`, `division`, `department` and
    /// `manager` all as `required: false`.
    ///
    /// Before version 1.0, this method required a value in all six
    /// attributes. It rejected every conformant `EnterpriseUser` that left
    /// any attribute unset. This crate keeps the implementation instead of
    /// dropping it. This choice keeps the trait uniform across every
    /// resource type. This choice also gives a future attribute-level
    /// check a place to live.
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
