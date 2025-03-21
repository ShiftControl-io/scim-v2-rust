use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub endpoint: String,
    pub schema: String,
    #[serde(rename = "schemaExtensions", skip_serializing_if = "Option::is_none")]
    pub schema_extensions: Option<Vec<SchemaExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
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
            meta: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaExtension {
    pub schema: String,
    pub required: bool,
}

impl Default for SchemaExtension {
    fn default() -> Self {
        SchemaExtension {
            schema: "".to_string(),
            required: false,
        }
    }
}

/// Returns a vector of `ResourceType` instances based on the provided resource type names.
///
/// This function creates `ResourceType` instances for "user" and "group" with default values if their names are included in the `resource_type_names` vector.
/// If "enterprise_user" is included in the `resource_type_names` vector, the "user" `ResourceType` will include the enterprise user schema extension.
///
/// # Parameters
///
/// * `resource_type_names` - A vector of string slices that represent the names of the resource types to be returned. Options are: user, group, enterprise_user
///
/// # Returns
///
/// * `Ok(Vec<ResourceType>)` - If all requested resource types are found. The returned vector includes the requested `ResourceType` instances.
/// * `Err(SCIMError::ResourceTypeNotFound)` - If a requested resource type is not found. The error includes the name of the resource type that was not found.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::resource_types::get_resource_types;
///
/// let resource_type_names = vec!["user", "group", "enterprise_user"];
/// match get_resource_types(resource_type_names) {
///     Ok(resource_types) => {
///         for resource_type in resource_types {
///             println!("ResourceType: {:?}", resource_type);
///         }
///     }
///     Err(e) => println!("Error getting resource types: {}", e),
/// }
/// ```
pub fn get_resource_types(mut resource_type_names: Vec<&str>) -> Result<Vec<ResourceType>, SCIMError> {
    let mut resource_types = Vec::new();
    let has_enterprise_user = resource_type_names.contains(&"enterprise_user");
    // Remove "enterprise_user" from the vector
    if has_enterprise_user {
        resource_type_names.retain(|&name| name != "enterprise_user");
    }

    for resource_type_name in resource_type_names {
        match resource_type_name {
            "user" => {
                let user_resource_type = ResourceType {
                    id: Some("User".to_string()),
                    name: "User".to_string(),
                    endpoint: "/Users".to_string(),
                    description: Some("User Account".to_string()),
                    schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
                    schema_extensions: if has_enterprise_user {
                        Some(vec![
                            SchemaExtension {
                                schema: "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User".to_string(),
                                required: true,
                            },
                        ])
                    } else {
                        None
                    },
                    meta: Some(Meta {
                        location: Some("https://example.com/v2/ResourceTypes/User".to_string()),
                        resource_type: Some("ResourceType".to_string()),
                        created: None,
                        last_modified: None,
                        version: None,
                    }),
                };
                resource_types.push(user_resource_type);
            }
            "group" => {
                let group_resource_type = ResourceType {
                    id: Some("Group".to_string()),
                    name: "Group".to_string(),
                    endpoint: "/Groups".to_string(),
                    description: Some("Group".to_string()),
                    schema: "urn:ietf:params:scim:schemas:core:2.0:Group".to_string(),
                    schema_extensions: None,
                    meta: Some(Meta {
                        location: Some("https://example.com/v2/ResourceTypes/Group".to_string()),
                        resource_type: Some("ResourceType".to_string()),
                        created: None,
                        last_modified: None,
                        version: None,
                    }),
                };
                resource_types.push(group_resource_type);
            }
            _ => return Err(SCIMError::ResourceTypeNotFound(resource_type_name.to_string())),
        }
    }
    Ok(resource_types)
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
        assert_eq!(resource_type.id, Some("User".to_string()));
        assert_eq!(resource_type.name, "User");
        assert_eq!(resource_type.endpoint, "/Users");
        assert_eq!(resource_type.description, Some("User Account".to_string()));
        assert_eq!(resource_type.schema, "urn:ietf:params:scim:schemas:core:2.0:User");
        let schema_extensions = resource_type.schema_extensions.unwrap();
        assert_eq!(schema_extensions.len(), 1);
        let schema_extension = &schema_extensions[0];
        assert_eq!(schema_extension.schema, "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User");
        assert_eq!(schema_extension.required, true);
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

    #[test]
    fn test_get_resource_types() {
        let resource_type_names = vec!["user", "group", "enterprise_user"];
        let resource_types = get_resource_types(resource_type_names).unwrap();

        assert_eq!(resource_types.len(), 2);

        let user_resource_type = &resource_types[0];
        assert_eq!(user_resource_type.name, "User");
        assert_eq!(user_resource_type.endpoint, "/Users");
        assert_eq!(user_resource_type.schema, "urn:ietf:params:scim:schemas:core:2.0:User");
        assert!(user_resource_type.schema_extensions.is_some());

        let group_resource_type = &resource_types[1];
        assert_eq!(group_resource_type.name, "Group");
        assert_eq!(group_resource_type.endpoint, "/Groups");
        assert_eq!(group_resource_type.schema, "urn:ietf:params:scim:schemas:core:2.0:Group");
        assert!(group_resource_type.schema_extensions.is_none());
    }
}