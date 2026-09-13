use crate::utils::validation::{Validate, ValidationError, require_schema_urn};
use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::multi::Multi;
use crate::schema_urns;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResourceType {
    /// RFC 7643 §3 requires this schema URN. The RFC's §8.6 example
    /// includes it. The `#[serde(default)]` attribute lets the crate read
    /// a payload from a non-conformant provider that omits the field. The
    /// `validate()` method reports the omission.
    #[serde(default)]
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub endpoint: String,
    pub schema: String,
    #[serde(
        rename = "schemaExtensions",
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub schema_extensions: Multi<SchemaExtension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl Default for ResourceType {
    fn default() -> Self {
        ResourceType {
            schemas: vec![schema_urns::RESOURCE_TYPE.to_string()],
            id: None,
            name: "".to_string(),
            description: None,
            endpoint: "".to_string(),
            schema: "".to_string(),
            schema_extensions: Multi::absent(),
            meta: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

/// Builds a list of `ResourceType` instances for the given names.
///
/// The function builds a default `ResourceType` for "user" and for "group"
/// when the caller lists that name. When the caller lists
/// "enterprise_user", the "user" `ResourceType` also gets the enterprise
/// user schema extension.
///
/// # Parameters
///
/// * `resource_type_names` - The names of the resource types to build. The
///   valid names are `user`, `group` and `enterprise_user`.
///
/// # Returns
///
/// * `Ok(Vec<ResourceType>)` - The function found every requested resource
///   type. The vector holds the built `ResourceType` instances.
/// * `Err(SCIMError::ResourceTypeNotFound)` - The function did not find a
///   requested resource type. The error names the resource type it did not
///   find.
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
pub fn get_resource_types(
    mut resource_type_names: Vec<&str>,
) -> Result<Vec<ResourceType>, SCIMError> {
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
                    schemas: vec![schema_urns::RESOURCE_TYPE.to_string()],
                    id: Some("User".to_string()),
                    name: "User".to_string(),
                    endpoint: "/Users".to_string(),
                    description: Some("User Account".to_string()),
                    schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
                    // RFC 7643 Figure 8 omits `schemaExtensions` on the Group
                    // resource type rather than sending an empty array.
                    schema_extensions: if has_enterprise_user {
                        Multi::from(vec![SchemaExtension {
                            schema: "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"
                                .to_string(),
                            required: true,
                        }])
                    } else {
                        Multi::absent()
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
                    schemas: vec![schema_urns::RESOURCE_TYPE.to_string()],
                    id: Some("Group".to_string()),
                    name: "Group".to_string(),
                    endpoint: "/Groups".to_string(),
                    description: Some("Group".to_string()),
                    schema: "urn:ietf:params:scim:schemas:core:2.0:Group".to_string(),
                    schema_extensions: Multi::absent(),
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
            _ => {
                return Err(SCIMError::ResourceTypeNotFound(
                    resource_type_name.to_string(),
                ));
            }
        }
    }
    Ok(resource_types)
}

/// Converts a JSON string into a `ResourceType` struct.
///
/// Use this method when an external source, such as a web request, sends
/// you a JSON representation of a resource type. The method parses the
/// string and builds a strongly-typed `ResourceType` object from it.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` when the JSON string does not
/// parse into a `ResourceType` object.
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

impl Validate for ResourceType {
    /// RFC 7643 §6 marks `name`, `endpoint` and `schema` REQUIRED. RFC 7643
    /// §6 also marks each schema extension's own `schema` REQUIRED. RFC
    /// 7643 §6 says `id` "is not required for the resource type". RFC 7643
    /// §3 marks `schemas` REQUIRED on every representation. Both §8.6
    /// examples carry `schemas`. Deserialization tolerates the absence of
    /// `schemas`. This tolerance lets a non-conformant provider's payload
    /// still be read. The `validate` method reports the absence.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, schema_urns::RESOURCE_TYPE)?;
        if self.name.is_empty() {
            return Err(ValidationError::missing_required("name"));
        }
        if self.endpoint.is_empty() {
            return Err(ValidationError::missing_required("endpoint"));
        }
        if self.schema.is_empty() {
            return Err(ValidationError::missing_required("schema"));
        }
        // §6 on each schema extension: `schema` "MUST be equal to the id
        // attribute of a Schema resource. REQUIRED."
        for (i, ext) in self.schema_extensions.iter().enumerate() {
            if ext.schema.is_empty() {
                return Err(ValidationError::missing_required(format!(
                    "schemaExtensions[{i}].schema"
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
