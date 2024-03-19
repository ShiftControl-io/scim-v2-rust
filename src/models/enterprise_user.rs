use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
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
            return Err(SCIMError::MissingRequiredField("employee_number".to_string()));
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Manager {
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}
