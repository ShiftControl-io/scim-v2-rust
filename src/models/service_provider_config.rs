use crate::utils::validation::{
    Validate, ValidationError, at_most_one_primary, require_schema_urn,
};
use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::multi::Multi;
use crate::schema_urns;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServiceProviderConfig {
    /// RFC 7643 §3 requires this schema URN. The RFC's §8.5 example
    /// includes it. The `#[serde(default)]` attribute lets the crate read
    /// a payload from a non-conformant provider that omits the field. The
    /// `validate()` method reports the omission.
    #[serde(default)]
    pub schemas: Vec<String>,
    #[serde(rename = "documentationUri", skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    pub patch: Supported,
    pub bulk: Bulk,
    pub filter: Filter,
    #[serde(rename = "changePassword")]
    pub change_password: Supported,
    pub sort: Supported,
    pub etag: Supported,
    #[serde(
        rename = "authenticationSchemes",
        default = "Multi::absent",
        skip_serializing_if = "Multi::is_absent"
    )]
    pub authentication_schemes: Multi<AuthenticationScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl Default for ServiceProviderConfig {
    fn default() -> Self {
        ServiceProviderConfig {
            schemas: vec![schema_urns::SERVICE_PROVIDER_CONFIG.to_string()],
            documentation_uri: None,
            patch: Supported::default(),
            bulk: Bulk::default(),
            filter: Filter::default(),
            change_password: Supported { supported: false },
            sort: Supported { supported: false },
            etag: Supported { supported: false },
            authentication_schemes: Multi::absent(),
            meta: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthenticationScheme {
    pub name: String,
    /// RFC 7643 §5 marks `type` REQUIRED, with the values "oauth", "oauth2",
    /// "oauthbearertoken", "httpbasic" and "httpdigest". This field is
    /// `Option` here for one reason. The §8.7 schema representation omits
    /// `type`. §8 calls that representation non-normative. Real servers
    /// follow the representation anyway. A discovery document must still
    /// parse. [`Validate`] enforces the §5 requirement.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub description: String,
    /// OPTIONAL per RFC 7643 §5.
    #[serde(rename = "specUri", skip_serializing_if = "Option::is_none")]
    pub spec_uri: Option<String>,
    #[serde(rename = "documentationUri", skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::serde::deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Filter {
    pub supported: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
}

/// Advertises no filter support. The REQUIRED `maxResults` field is
/// present and set to zero. See [`Bulk`]'s `Default` impl for the reason
/// the limit is not a positive number.
impl Default for Filter {
    fn default() -> Self {
        Filter {
            supported: false,
            max_results: 0,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Bulk {
    pub supported: bool,
    #[serde(rename = "maxOperations")]
    pub max_operations: i64,
    #[serde(rename = "maxPayloadSize")]
    pub max_payload_size: i64,
}

/// Advertises no bulk support. RFC 7643 §5 marks `maxOperations` and
/// `maxPayloadSize` REQUIRED. Both fields are therefore present. Each
/// field is zero. A non-zero limit alongside `supported: false` would
/// claim a capacity the server does not have.
///
/// Before 1.0, this method returned 1000 and 1048576.
/// `ServiceProviderConfig::default()` built the same struct by hand, with
/// zeros instead. The two constructors disagreed, depending on which one
/// a caller used.
impl Default for Bulk {
    fn default() -> Self {
        Bulk {
            supported: false,
            max_operations: 0,
            max_payload_size: 0,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Supported {
    pub supported: bool,
}

/// Converts a JSON string into a `ServiceProviderConfig` struct.
///
/// Use this method when an external source, such as a web request, sends
/// you a JSON representation of a service provider config. The method
/// parses the string and builds a strongly-typed `ServiceProviderConfig`
/// object from it.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` when the JSON string does not
/// parse into a `ServiceProviderConfig` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::service_provider_config::ServiceProviderConfig;
///
/// let config_json = r#"{
///             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig"],
///             "documentationUri": "http:///example.com/help/scim.html",
///             "patch": { "supported": true },
///             "bulk": {
///                 "supported": true,
///                 "maxOperations": 1000,
///                 "maxPayloadSize": 1048576
///             },
///             "filter": {
///                 "supported": true,
///                 "maxResults": 200
///             },
///             "changePassword": { "supported": true },
///             "sort": { "supported": true },
///             "etag": { "supported": true },
///             "authenticationSchemes": [
///                 {
///                     "name": "OAuth Bearer Token",
///                     "description": "Authentication scheme using the OAuth Bearer Token Standard",
///                     "specUri": "http:///www.rfc-editor.org/info/rfc6750",
///                     "documentationUri": "http:///example.com/help/oauth.html"
///                 },
///                 {
///                     "name": "HTTP Basic",
///                     "description": "Authentication scheme using the HTTP Basic Standard",
///                     "specUri": "http:///www.rfc-editor.org/info/rfc2617",
///                     "documentationUri": "http:///example.com/help/httpBasic.html"
///                 }
///             ]
///         }"#;
/// match ServiceProviderConfig::try_from(config_json) {
///     Ok(config) => println!("Successfully converted JSON to ServiceProviderConfig: {:?}", config),
///     Err(e) => println!("Error converting from JSON to ServiceProviderConfig: {}", e),
/// }
/// ```
impl TryFrom<&str> for ServiceProviderConfig {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl Validate for AuthenticationScheme {
    /// RFC 7643 §5 marks `type`, `name` and `description` REQUIRED. RFC
    /// 7643 §5 marks `specUri` and `documentationUri` OPTIONAL.
    fn validate(&self) -> Result<(), ValidationError> {
        if self.r#type.as_deref().is_none_or(str::is_empty) {
            return Err(ValidationError::missing_required("type"));
        }
        if self.name.is_empty() {
            return Err(ValidationError::missing_required("name"));
        }
        if self.description.is_empty() {
            return Err(ValidationError::missing_required("description"));
        }
        Ok(())
    }
}

impl Validate for ServiceProviderConfig {
    /// RFC 7643 §5 marks `authenticationSchemes` REQUIRED. An empty list is
    /// therefore a conformance failure. This method checks each scheme in
    /// turn and reports its index in the path. RFC 7643 §5 also marks
    /// `patch`, `bulk`, `filter`, `changePassword`, `sort` and `etag`
    /// REQUIRED. These six fields are non-`Option` fields. Because of
    /// that, `serde` already refuses a payload that omits any of them.
    ///
    /// Before 1.0, this method returned `MissingRequiredField` whenever any
    /// of those six fields reported `supported: false`. That behavior
    /// rejected valid configurations. §5 makes the `supported` **field**
    /// required, not its value `true`. A server that does not implement
    /// bulk correctly advertises `"bulk": {"supported": false}`.
    fn validate(&self) -> Result<(), ValidationError> {
        // §5: "id is not required". `schemas` is: RFC 7643 §3 says "all
        // representations of SCIM schemas MUST include a non-empty array", and
        // the §8.5 example carries it. Deserialization still tolerates absence
        // so a non-conformant provider's config can be read; this reports it.
        require_schema_urn(&self.schemas, schema_urns::SERVICE_PROVIDER_CONFIG)?;
        if self.authentication_schemes.is_empty() {
            return Err(ValidationError::missing_required("authenticationSchemes"));
        }
        for (i, scheme) in self.authentication_schemes.iter().enumerate() {
            scheme
                .validate()
                .map_err(|e| e.under(&format!("authenticationSchemes[{i}]")))?;
        }
        at_most_one_primary(
            &self.authentication_schemes,
            |a| a.primary,
            "authenticationSchemes",
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
