use serde::{Deserialize, Serialize};


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
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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