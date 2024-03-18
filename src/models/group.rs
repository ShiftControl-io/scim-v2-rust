//Schema for group
use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub members: Option<Vec<Member>>,
    pub meta: Option<Meta>,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
            id: "default_id".to_string(),
            display_name: "default_display_name".to_string(),
            members: None,
            meta: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Member {
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub display: Option<String>,
}




/// Converts a JSON string into a `Group` struct.
///
/// This method attempts to parse a JSON string to construct a `Group` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `Group` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::group::Group;
///
/// let group_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"], "id": "e9e30dba-f08f-4109-8486-d5c6a331660a", "displayName": "Tour Guides"}"#;
/// match Group::try_from(group_json) {
///     Ok(group) => println!("Successfully converted JSON to Group: {:?}", group),
///     Err(e) => println!("Error converting from JSON to Group: {}", e),
/// }
/// ```
impl TryFrom<&str> for Group {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl Group {
    /// Validates a group.
    ///
    /// This function checks if the group has `schemas`, `id`, and `display_name`. If any of these fields are missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `group` - A reference to a Group instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the group is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    ///
    /// # Example
    ///
    /// ```
    /// use scim_v2::models::group::Group;
    ///
    /// let group = Group {
    ///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
    ///     id: "e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(),
    ///     display_name: "Tour Guides".to_string(),
    ///     // other fields...
    ///     ..Default::default()
    /// };
    ///
    /// match group.validate() {
    ///     Ok(_) => println!("Group is valid."),
    ///     Err(e) => println!("Group is invalid: {}", e),
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), SCIMError> {
        if self.schemas.is_empty() {
            return Err(SCIMError::MissingRequiredField("schemas".to_string()));
        }
        if self.id.is_empty() {
            return Err(SCIMError::MissingRequiredField("id".to_string()));
        }
        if self.display_name.is_empty() {
            return Err(SCIMError::MissingRequiredField("display_name".to_string()));
        }
        Ok(())
    }

    /// Serializes the `Group` instance to a JSON string, using the custom SCIMError for error handling.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<String, SCIMError>`, where `Ok(String)` contains
    /// the JSON string representation of the `Group` instance, and `Err(SCIMError)` contains
    /// the custom error encountered during serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::group::Group;
    ///
    /// let group = Group {
    ///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
    ///     id: "e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(),
    ///     display_name: "Tour Guides".to_string(),
    ///     // other fields...
    ///     ..Default::default()
    /// };
    ///
    /// match group.serialize() {
    ///     Ok(json) => println!("Serialized User: {}", json),
    ///     Err(e) => println!("Serialization error: {}", e),
    /// }
    /// ```
    pub fn serialize(&self) -> Result<String, SCIMError> {
        serde_json::to_string(&self).map_err(SCIMError::SerializationError)
    }

    /// Deserializes a JSON string into a `Group` instance, using the custom SCIMError for error handling.
    ///
    /// # Parameters
    ///
    /// * `json` - A string slice that holds the JSON representation of a `Group`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<Group, SCIMError>`, where `Ok(Group)` is the deserialized `Group` instance,
    /// and `Err(SCIMError)` is the custom error encountered during deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::group::Group;
    ///
    /// let group_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"], "id": "e9e30dba-f08f-4109-8486-d5c6a331660a", "displayName": "Tour Guides"}"#;
    /// match Group::deserialize(group_json) {
    ///     Ok(group) => println!("Deserialized Group: {:?}", group),
    ///     Err(e) => println!("Deserialization error: {}", e),
    /// }
    /// ```
    pub fn deserialize(json: &str) -> Result<Self, SCIMError> {
        serde_json::from_str(json).map_err(SCIMError::DeserializationError)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn group_deserialization_succeeds_for_valid_full_json() {
        let json_data = r#"   {
             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
             "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
             "displayName": "Tour Guides",
             "members": [
               {
                 "value": "2819c223-7f76-453a-919d-413861904646",
                 "$ref":
           "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646",
                 "display": "Babs Jensen"
               },
               {
                 "value": "902c246b-6245-4190-8e05-00816be7344a",
                 "$ref":
           "https://example.com/v2/Users/902c246b-6245-4190-8e05-00816be7344a",
                 "display": "Mandy Pepperidge"
               }
             ],
             "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location":
           "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
             }
           }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &group {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
        assert_eq!(group.display_name, "Tour Guides");

        // Check members
        assert_eq!(group.members.as_ref().unwrap().len(), 2);
        assert_eq!(group.members.as_ref().unwrap()[0].value, Some("2819c223-7f76-453a-919d-413861904646".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[0].display, Some("Babs Jensen".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[1].value, Some("902c246b-6245-4190-8e05-00816be7344a".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[1].display, Some("Mandy Pepperidge".to_string()));

        // Check meta
        let meta = group.meta.unwrap();
        assert_eq!(meta.resource_type, "Group");
        assert_eq!(meta.created, "2010-01-23T04:56:22Z");
        assert_eq!(meta.last_modified, "2011-05-13T04:42:34Z");
        assert_eq!(meta.version, "W/\"3694e05e9dff592\"");
        assert_eq!(meta.location, "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a");
    }

    #[test]
    fn group_deserialization_succeeds_for_valid_json() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides",
            "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }
        }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &group {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
        assert_eq!(group.display_name, "Tour Guides");
    }

    #[test]
    fn group_deserialization_fails_for_invalid_json() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": 12345
        },
        "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        assert!(group.is_err());
    }

    #[test]
    fn group_deserialization_handles_missing_optional_fields() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides"
        }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
        assert_eq!(group.display_name, "Tour Guides");
        assert!(group.members.is_none());
        assert!(group.meta.is_none());
    }
}