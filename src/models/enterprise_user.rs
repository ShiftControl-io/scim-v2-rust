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

impl EnterpriseUser {
    /// Validates an enterprise user.
    ///
    /// This function checks if the enterprise user has `employee_number`, `cost_center`, `organization`, `division`, `department`, and `manager`. If any of these fields are missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `enterprise_user` - A reference to an EnterpriseUser instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the enterprise user is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    ///
    /// # Example
    ///
    /// ```
    /// use scim_v2::models::enterprise_user::EnterpriseUser;
    ///
    /// let enterprise_user = EnterpriseUser {
    ///     // Initialize enterprise_user fields here...
    ///     // ...
    ///     ..Default::default()
    /// };
    ///
    /// match enterprise_user.validate() {
    ///     Ok(_) => println!("EnterpriseUser is valid."),
    ///     Err(e) => println!("EnterpriseUser is invalid: {}", e),
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), SCIMError> {
        if self.employee_number.is_none() {
            return Err(SCIMError::MissingRequiredField(
                "employee_number".to_string(),
            ));
        }
        if self.cost_center.is_none() {
            return Err(SCIMError::MissingRequiredField("cost_center".to_string()));
        }
        if self.organization.is_none() {
            return Err(SCIMError::MissingRequiredField("organization".to_string()));
        }
        if self.division.is_none() {
            return Err(SCIMError::MissingRequiredField("division".to_string()));
        }
        if self.department.is_none() {
            return Err(SCIMError::MissingRequiredField("department".to_string()));
        }
        if self.manager.is_none() {
            return Err(SCIMError::MissingRequiredField("manager".to_string()));
        }
        Ok(())
    }
    /// Serializes the `EnterpriseUser` instance to a JSON string, using the custom SCIMError for error handling.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<String, SCIMError>`, where `Ok(String)` contains
    /// the JSON string representation of the `EnterpriseUser` instance, and `Err(SCIMError)` contains
    /// the custom error encountered during serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::enterprise_user::EnterpriseUser;
    ///
    /// let ent_user = EnterpriseUser {
    ///     // Initialize enterprise_user fields here...
    ///     // ...
    ///     ..Default::default()
    /// };
    ///
    /// match ent_user.serialize() {
    ///     Ok(json) => println!("Serialized EnterpriseUser: {}", json),
    ///     Err(e) => println!("Serialization error: {}", e),
    /// }
    /// ```
    pub fn serialize(&self) -> Result<String, SCIMError> {
        serde_json::to_string(&self).map_err(SCIMError::SerializationError)
    }

    /// Deserializes a JSON string into a `EnterpriseUser` instance, using the custom SCIMError for error handling.
    ///
    /// # Parameters
    ///
    /// * `json` - A string slice that holds the JSON representation of a `EnterpriseUser`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<EnterpriseUser, SCIMError>`, where `Ok(EnterpriseUser)` is the deserialized `EnterpriseUser` instance,
    /// and `Err(SCIMError)` is the custom error encountered during deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::enterprise_user::EnterpriseUser;
    ///
    ///
    /// let ent_user_json = r#"{
    ///             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
    ///             "id": "2819c223-7f76-453a-919d-413861904646",
    ///             "userName": "bjensen@example.com"
    ///         }"#;
    /// match EnterpriseUser::deserialize(ent_user_json) {
    ///     Ok(ent_user) => println!("Deserialized User: {:?}", ent_user),
    ///     Err(e) => println!("Deserialization error: {}", e),
    /// }
    /// ```
    pub fn deserialize(json: &str) -> Result<Self, SCIMError> {
        serde_json::from_str(json).map_err(SCIMError::DeserializationError)
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Manager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
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
