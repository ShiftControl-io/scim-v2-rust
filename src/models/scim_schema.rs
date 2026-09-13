use serde::{Deserialize, Serialize};

use crate::models::datetime::ScimDateTime;
use crate::multi::Multi;
use crate::utils::error::SCIMError;
use crate::utils::validation::{Validate, ValidationError, require_schema_urn};
#[cfg(feature = "schemas")]
use crate::{ENTERPRISE_USER_SCHEMA, GROUP_SCHEMA, USER_SCHEMA};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Meta {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// RFC 7643 §2.3.5 makes this an `xsd:dateTime`. Only [`ScimDateTime`]
    /// can hold this value. The field cannot hold a malformed date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<ScimDateTime>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<ScimDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Schema {
    /// RFC 7643 §7 notes that a schema resource is often served without a
    /// `schemas` attribute. This field defaults to an empty list for that
    /// reason. The crate keeps a value that is present. The crate does not
    /// drop the value silently.
    #[serde(default)]
    pub schemas: Vec<String>,
    pub id: String,
    pub name: String,
    pub description: String,
    pub attributes: Vec<Attributes>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub canonical_values: Multi<String>,
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
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub sub_attributes: Multi<SubAttributes>,
    #[serde(
        rename = "referenceTypes",
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub reference_types: Multi<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub canonical_values: Multi<String>,
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
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub reference_types: Multi<String>,
}

#[cfg(feature = "schemas")]
/// Retrieves a list of `Schema` instances for the given schema names.
///
/// The function matches each name against a predefined list of schema
/// contents and retrieves the matching `Schema` instances.
///
/// # Parameters
///
/// * `schema_names` - The names of the schemas to retrieve.
///
/// # Returns
///
/// `Ok(Vec<Schema>)` on success, holding the retrieved `Schema` instances.
/// `Err(SCIMError)` on failure, holding the error that occurred.
///
/// # Errors
///
/// The function returns `SCIMError::SchemaNotFound` when a given schema
/// name matches none of the predefined schema contents. The error names
/// the schema it could not find.
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
/// Use this method when an external source, such as a web request, sends
/// you a JSON representation of a schema. The method parses the string and
/// builds a strongly-typed `Schema` object from it.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` when the JSON string does not
/// parse into a `Schema` object.
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

impl Validate for Schema {
    /// RFC 7643 §7 makes `id` the schema URI. The RFC says "service
    /// providers MUST specify" it. `name` and `description` are OPTIONAL.
    ///
    /// `schemas` is the one attribute where this crate's validators
    /// tolerate absence. RFC 7643 §3 marks `schemas` REQUIRED on "all
    /// representations". The RFC's own §8.7 examples break that rule. The
    /// §8.7 definitions for User, Group, EnterpriseUser,
    /// ServiceProviderConfig, ResourceType and Schema carry no `schemas`
    /// attribute at all. Real providers follow that example. Rejecting
    /// what the RFC itself publishes would make `validate()` useless on a
    /// real `/Schemas` response. For this reason, absence passes
    /// validation here alone. A present array must still be non-empty. It
    /// must be unique. It must name this resource type. `ResourceType`
    /// and `ServiceProviderConfig` require `schemas`. Their §8.5 and §8.6
    /// examples carry it.
    fn validate(&self) -> Result<(), ValidationError> {
        if !self.schemas.is_empty() {
            require_schema_urn(&self.schemas, crate::schema_urns::SCHEMA)?;
        }
        if self.id.is_empty() {
            return Err(ValidationError::missing_required("id"));
        }
        Ok(())
    }
}

/// Tests for the `Schema` type itself, independent of the embedded
/// definitions, so they still run without the `schemas` feature.
#[cfg(test)]
mod schema_tests;

#[cfg(all(test, feature = "schemas"))]
mod tests;
