use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceType {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub schema: String,
    #[serde(rename = "schemaExtensions")]
    pub schema_extensions: Option<Vec<SchemaExtension>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaExtension {
    pub schema: String,
    pub required: bool,
}

impl Default for ResourceType {
    fn default() -> Self {
        ResourceType {
            id: None,
            name: "".to_string(),
            description: None,
            endpoint: "".to_string(),
            schema: "".to_string(),
            schema_extensions: None,
        }
    }
}

/// Converts a JSON string into a `ResourceType` struct.
///
/// This method attempts to parse a JSON string to construct a `ResourceType` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `ResourceType` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::resource_types::ResourceType;///
/// let resource_type_data = r#"{
///                 "schemas":
///                 [
///                     "urn:ietf:params:scim:schemas:core:2.0:ResourceType"
///                 ],
///                 "id": "User",
///                 "name": "User",
///                 "endpoint": "/Users",
///                 "description": "User Account",
///                 "schema": "urn:ietf:params:scim:schemas:core:2.0:User",
///                 "schemaExtensions":
///                 [
///                     {
///                         "schema": "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
///                         "required": true
///                     }
///                 ],
///                 "meta":
///                 {
///                     "location": "https:///example.com/v2/ResourceTypes/User",
///                     "resourceType": "ResourceType"
///                 }
///             }"#;
/// match ResourceType::try_from(resource_type_data) {
///     Ok(resource_type) => println!("Successfully converted JSON to ResourceType: {:?}", resource_type),
///     Err(e) => println!("Error converting from JSON to ResourceType: {}", e),
/// }
/// ```
impl TryFrom<&str> for ResourceType {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl ResourceType {
    /// Validates a resource type.
    ///
    /// This function checks if the resource type has `name`, `endpoint`, and `schema`. If any of these fields are missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - A reference to a ResourceType instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the resource type is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    ///
    /// # Example
    ///
    /// ```
    /// use scim_v2::models::resource_types::ResourceType;
    ///
    /// let resource_type = ResourceType {
    ///     name: "User".to_string(),
    ///     endpoint: "/Users".to_string(),
    ///     schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
    ///     // other fields...
    ///     ..Default::default()
    /// };
    ///
    /// match resource_type.validate() {
    ///     Ok(_) => println!("ResourceType is valid."),
    ///     Err(e) => println!("ResourceType is invalid: {}", e),
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), SCIMError> {
        if self.name.is_empty() {
            return Err(SCIMError::MissingRequiredField("name".to_string()));
        }
        if self.endpoint.is_empty() {
            return Err(SCIMError::MissingRequiredField("endpoint".to_string()));
        }
        if self.schema.is_empty() {
            return Err(SCIMError::MissingRequiredField("schema".to_string()));
        }
        Ok(())
    }
    /// Serializes the `ResourceType` instance to a JSON string, using the custom SCIMError for error handling.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<String, SCIMError>`, where `Ok(String)` contains
    /// the JSON string representation of the `ResourceType` instance, and `Err(SCIMError)` contains
    /// the custom error encountered during serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::resource_types::ResourceType;
    ///
    /// let resource_type = ResourceType {
    ///     name: "User".to_string(),
    ///     endpoint: "/Users".to_string(),
    ///     schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
    ///     // other fields...
    ///     ..Default::default()
    /// };
    ///
    /// match resource_type.serialize() {
    ///     Ok(json) => println!("Serialized ResourceType: {}", json),
    ///     Err(e) => println!("Serialization error: {}", e),
    /// }
    /// ```
    pub fn serialize(&self) -> Result<String, SCIMError> {
        serde_json::to_string(&self).map_err(SCIMError::SerializationError)
    }

    /// Deserializes a JSON string into a `ResourceType` instance, using the custom SCIMError for error handling.
    ///
    /// # Parameters
    ///
    /// * `json` - A string slice that holds the JSON representation of a `ResourceType`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<ResourceType, SCIMError>`, where `Ok(ResourceType)` is the deserialized `ResourceType` instance,
    /// and `Err(SCIMError)` is the custom error encountered during deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::resource_types::ResourceType;
    ///
    ///
    /// let resource_type_data = r#"{
    ///                 "schemas":
    ///                 [
    ///                     "urn:ietf:params:scim:schemas:core:2.0:ResourceType"
    ///                 ],
    ///                 "id": "User",
    ///                 "name": "User",
    ///                 "endpoint": "/Users",
    ///                 "description": "User Account",
    ///                 "schema": "urn:ietf:params:scim:schemas:core:2.0:User",
    ///                 "schemaExtensions":
    ///                 [
    ///                     {
    ///                         "schema": "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
    ///                         "required": true
    ///                     }
    ///                 ],
    ///                 "meta":
    ///                 {
    ///                     "location": "https:///example.com/v2/ResourceTypes/User",
    ///                     "resourceType": "ResourceType"
    ///                 }
    ///             }"#;
    /// match ResourceType::deserialize(resource_type_data) {
    ///     Ok(resource_type) => println!("Deserialized ResourceType: {:?}", resource_type),
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
    fn resource_type_deserialization_user() {
        let json_data = r#"{
                "schemas":
                [
                    "urn:ietf:params:scim:schemas:core:2.0:ResourceType"
                ],
                "id": "User",
                "name": "User",
                "endpoint": "/Users",
                "description": "User Account",
                "schema": "urn:ietf:params:scim:schemas:core:2.0:User",
                "schemaExtensions":
                [
                    {
                        "schema": "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
                        "required": true
                    }
                ],
                "meta":
                {
                    "location": "https://example.com/v2/ResourceTypes/User",
                    "resourceType": "ResourceType"
                }
            }"#;

        let resource_type: Result<ResourceType, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &resource_type {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(resource_type.is_ok());
        let resource_type = resource_type.unwrap();
        std::assert_eq!(resource_type.id, Some("User".to_string()));
        std::assert_eq!(resource_type.name, "User");
        std::assert_eq!(resource_type.endpoint, "/Users");
        std::assert_eq!(resource_type.description, Some("User Account".to_string()));
        std::assert_eq!(resource_type.schema, "urn:ietf:params:scim:schemas:core:2.0:User");
        let schema_extensions = resource_type.schema_extensions.unwrap();
        std::assert_eq!(schema_extensions.len(), 1);
        let schema_extension = &schema_extensions[0];
        std::assert_eq!(schema_extension.schema, "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User");
        std::assert_eq!(schema_extension.required, true);
    }

    #[test]
    fn resource_type_deserialization_group() {
        let json_data = r#"{
                "schemas":
                [
                    "urn:ietf:params:scim:schemas:core:2.0:ResourceType"


                ],
                "id": "Group",
                "name": "Group",
                "endpoint": "/Groups",
                "description": "Group",
                "schema": "urn:ietf:params:scim:schemas:core:2.0:Group",
                "meta":
                {
                    "location": "https://example.com/v2/ResourceTypes/Group",
                    "resourceType": "ResourceType"
                }
            }"#;

        let resource_type: Result<ResourceType, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &resource_type {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(resource_type.is_ok());
        let resource_type = resource_type.unwrap();
        assert_eq!(resource_type.id, Some("Group".to_string()));
        assert_eq!(resource_type.name, "Group");
        assert_eq!(resource_type.endpoint, "/Groups");
        assert_eq!(resource_type.description, Some("Group".to_string()));
        assert_eq!(resource_type.schema, "urn:ietf:params:scim:schemas:core:2.0:Group");
    }
}