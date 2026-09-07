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
    use serde_json::json;

    /// Unset `Manager` fields are omitted on serialize, not emitted as
    /// `null` (RFC 7643 §2.5 treats the two as equivalent, but omission
    /// keeps output compact and matches every other model in the crate).
    #[test]
    fn manager_omits_unset_fields_on_serialize() {
        let manager = Manager {
            value: Some("26118915-6090-4610-87e4-49d8ca9f808d".to_string()),
            r#ref: None,
            display_name: None,
        };
        let serialized = serde_json::to_value(&manager).unwrap();
        assert_eq!(
            serialized,
            json!({ "value": "26118915-6090-4610-87e4-49d8ca9f808d" })
        );
    }

    /// A fully-unset `Manager` serializes to an empty object rather than
    /// one populated with `null`s. This pins the `value` field too, which
    /// `manager_omits_unset_fields_on_serialize` leaves set.
    #[test]
    fn manager_omits_all_unset_fields_on_serialize() {
        let manager = Manager {
            value: None,
            r#ref: None,
            display_name: None,
        };
        assert_eq!(serde_json::to_value(&manager).unwrap(), json!({}));
    }

    /// The read path must keep accepting explicit `null` sub-attributes from
    /// peers that send them (RFC 7643 §2.5) — a missing key, `null`, and `{}`
    /// all deserialize to `None`. Regression guard against a future
    /// `#[serde(default)]`, `deny_unknown_fields`, or custom deserializer
    /// silently changing this (cf. `Role.primary`, CHANGELOG 0.4.2).
    #[test]
    fn manager_explicit_null_deserializes_to_none() {
        let manager: Manager = serde_json::from_str(
            r#"{"value": "26118915-6090-4610-87e4-49d8ca9f808d", "$ref": null, "displayName": null}"#,
        )
        .unwrap();
        assert_eq!(
            manager.value.as_deref(),
            Some("26118915-6090-4610-87e4-49d8ca9f808d")
        );
        assert_eq!(manager.r#ref, None);
        assert_eq!(manager.display_name, None);

        let empty: Manager = serde_json::from_str("{}").unwrap();
        assert_eq!(empty.value, None);
        assert_eq!(empty.r#ref, None);
        assert_eq!(empty.display_name, None);
    }

    #[test]
    fn manager_round_trips_all_fields() {
        let raw = json!({
            "value": "26118915-6090-4610-87e4-49d8ca9f808d",
            "$ref": "https://example.com/v2/Users/26118915-6090-4610-87e4-49d8ca9f808d",
            "displayName": "John Smith"
        });
        let manager: Manager = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(&manager).unwrap(), raw);
    }
}
