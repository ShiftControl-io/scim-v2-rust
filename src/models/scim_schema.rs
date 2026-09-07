use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;
use crate::{ENTERPRISE_USER_SCHEMA, GROUP_SCHEMA, USER_SCHEMA};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Meta {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    /// RFC 7643 §7: schema resources are commonly served without a
    /// `schemas` attribute, so this defaults to empty; a present value
    /// round-trips rather than being silently dropped.
    #[serde(default)]
    pub schemas: Vec<String>,
    pub id: String,
    pub name: String,
    pub description: String,
    pub attributes: Vec<Attributes>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub name: String,
    pub r#type: String,
    #[serde(rename = "multiValued")]
    pub multi_valued: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(
        rename = "canonicalValues",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub canonical_values: Vec<String>,
    #[serde(rename = "caseExact", skip_serializing_if = "Option::is_none")]
    pub case_exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniqueness: Option<String>,
    #[serde(
        rename = "subAttributes",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_attributes: Vec<SubAttributes>,
    #[serde(
        rename = "referenceTypes",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reference_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubAttributes {
    pub name: String,
    pub r#type: String,
    #[serde(rename = "multiValued")]
    pub multi_valued: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(
        rename = "canonicalValues",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub canonical_values: Vec<String>,
    #[serde(rename = "caseExact", skip_serializing_if = "Option::is_none")]
    pub case_exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniqueness: Option<String>,
    #[serde(
        rename = "referenceTypes",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reference_types: Vec<String>,
}

/// Retrieves a list of `Schema` instances based on the provided schema names.
///
/// This function takes a vector of schema names as input and attempts to retrieve the corresponding `Schema` instances.
/// It uses a predefined list of schema contents to match the input schema names.
///
/// # Parameters
///
/// * `schema_names` - A vector of string slices that represent the names of the schemas to retrieve.
///
/// # Returns
///
/// This function returns a `Result<Vec<Schema>, SCIMError>`. If the function succeeds, it returns `Ok(Vec<Schema>)`
/// where `Vec<Schema>` is a vector of the retrieved `Schema` instances. If the function fails, it returns `Err(SCIMError)`
/// where `SCIMError` is the error that occurred.
///
/// # Errors
///
/// This function will return an error if any of the provided schema names do not match any of the predefined schema contents.
/// The error will be of type `SCIMError::SchemaNotFound` and will contain the name of the schema that could not be found.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::scim_schema::get_schemas;
///
/// let schemas = get_schemas(vec!["user", "group"]);
/// match schemas {
///     Ok(schemas) => println!("Successfully retrieved schemas: {:?}", schemas),
///     Err(e) => println!("Error retrieving schemas: {}", e),
/// }
/// ```
pub fn get_schemas(schema_names: Vec<&str>) -> Result<Vec<Schema>, SCIMError> {
    let mut schemas = Vec::new();

    let schema_contents = [
        ("user", USER_SCHEMA),
        ("enterprise_user", ENTERPRISE_USER_SCHEMA),
        ("group", GROUP_SCHEMA),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<_, _>>();

    for schema_name in schema_names {
        if let Some(schema_content) = schema_contents.get(schema_name) {
            let schema: Schema = serde_json::from_str(schema_content)?;
            schemas.push(schema);
        } else {
            return Err(SCIMError::SchemaNotFound(schema_name.to_string()));
        }
    }
    Ok(schemas)
}

/// Converts a JSON string into a `Schema` struct.
///
/// This method attempts to parse a JSON string to construct a `Schema` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `Schema` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::scim_schema::Schema;
///
/// let schema_json = r#"{
///   "id": "urn:ietf:params:scim:schemas:core:2.0:ResourceType",
///   "name": "ResourceType",
///   "description": "Specifies the schema that describes a SCIM resource type",
///   "attributes": [
///     {
///       "name": "id",
///       "type": "string",
///       "multiValued": false,
///       "description": "The resource type's server unique id. May be the same as the 'name' attribute.",
///       "required": false,
///       "caseExact": false,
///       "mutability": "readOnly",
///       "returned": "default",
///       "uniqueness": "none"
///     },
///     {
///       "name": "name",
///       "type": "string",
///       "multiValued": false,
///       "description": "The resource type name.  When applicable, service providers MUST specify the name, e.g., 'User'.",
///       "required": true,
///       "caseExact": false,
///       "mutability": "readOnly",
///       "returned": "default",
///       "uniqueness": "none"
///     },
///     {
///       "name": "description",
///       "type": "string",
///       "multiValued": false,
///       "description": "The resource type's human-readable description.  When applicable, service providers MUST specify the description.",
///       "required": false,
///       "caseExact": false,
///       "mutability": "readOnly",
///       "returned": "default",
///       "uniqueness": "none"
///     },
///     {
///       "name": "endpoint",
///       "type": "reference",
///       "referenceTypes": [
///         "uri"
///       ],
///       "multiValued": false,
///       "description": "The resource type's HTTP-addressable endpoint relative to the Base URL, e.g., '/Users'.",
///       "required": true,
///       "caseExact": false,
///       "mutability": "readOnly",
///       "returned": "default",
///       "uniqueness": "none"
///     },
///     {
///       "name": "schema",
///       "type": "reference",
///       "referenceTypes": [
///         "uri"
///       ],
///       "multiValued": false,
///       "description": "The resource type's primary/base schema URI.",
///       "required": true,
///       "caseExact": true,
///       "mutability": "readOnly",
///       "returned": "default",
///       "uniqueness": "none"
///     },
///     {
///       "name": "schemaExtensions",
///       "type": "complex",
///       "multiValued": false,
///       "description": "A list of URIs of the resource type's schema extensions.",
///       "required": true,
///       "mutability": "readOnly",
///       "returned": "default",
///       "subAttributes": [
///         {
///           "name": "schema",
///           "type": "reference",
///           "referenceTypes": [
///             "uri"
///           ],
///           "multiValued": false,
///           "description": "The URI of a schema extension.",
///           "required": true,
///           "caseExact": true,
///           "mutability": "readOnly",
///           "returned": "default",
///           "uniqueness": "none"
///         },
///         {
///           "name": "required",
///           "type": "boolean",
///           "multiValued": false,
///           "description": "A Boolean value that specifies whether or not the schema extension is required for the resource type.  If true, a resource of this type MUST include this schema extension and also include any attributes declared as required in this schema extension. If false, a resource of this type MAY omit this schema extension.",
///           "required": true,
///           "mutability": "readOnly",
///           "returned": "default"
///         }
///       ]
///     }
///   ]
/// }"#;
/// match Schema::try_from(schema_json) {
///     Ok(schema) => println!("Successfully converted JSON to Schema: {:?}", schema),
///     Err(e) => println!("Error converting from JSON to Schema: {}", e),
/// }
/// ```
impl TryFrom<&str> for Schema {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_schemas_returns_correct_schemas_for_valid_input() {
        let schemas = get_schemas(vec!["user"]).unwrap();
        assert_eq!(schemas.len(), 1);
        assert_eq!(schemas[0].id, "urn:ietf:params:scim:schemas:core:2.0:User");
        assert_eq!(schemas[0].name, "User");
        assert_eq!(schemas[0].description, "User Account");
        assert_eq!(schemas[0].attributes.len(), 21);
        assert_eq!(
            schemas[0].meta.resource_type.as_ref(),
            Some(&"Schema".to_string())
        );
        assert_eq!(
            schemas[0].meta.location.as_ref(),
            Some(&"/v2/Schemas/urn:ietf:params:scim:schemas:core:2.0:User".to_string())
        );
    }

    #[test]
    fn get_schemas_returns_error_for_invalid_input() {
        let result = get_schemas(vec!["invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn get_schemas_returns_error_for_missing_file() {
        let result = get_schemas(vec!["missing"]);
        assert!(result.is_err());
    }
}
