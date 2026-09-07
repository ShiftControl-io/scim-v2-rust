use crate::utils::validation::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::schema_urns;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceProviderConfig {
    /// RFC 7643: the schema URN(s) this resource conforms to.
    /// `#[serde(default)]` because RFC 7643 §§6-7 allow the discovery
    /// resources to be served without it; a present value round-trips.
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
    #[serde(rename = "authenticationSchemes")]
    pub authentication_schemes: Vec<AuthenticationScheme>,
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
            authentication_schemes: vec![],
            meta: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationScheme {
    pub name: String,
    pub r#type: String,
    pub description: String,
    #[serde(rename = "specUri")]
    pub spec_uri: String,
    #[serde(rename = "documentationUri", skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl Default for AuthenticationScheme {
    fn default() -> Self {
        AuthenticationScheme {
            name: "".to_string(),
            r#type: "".to_string(),
            description: "".to_string(),
            spec_uri: "".to_string(),
            documentation_uri: Some("".to_string()),
            primary: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub supported: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
}

/// Advertises no filter support, with the REQUIRED `maxResults` present and
/// zero. See [`Bulk`]'s `Default` for why the limit is not a positive number.
impl Default for Filter {
    fn default() -> Self {
        Filter {
            supported: false,
            max_results: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bulk {
    pub supported: bool,
    #[serde(rename = "maxOperations")]
    pub max_operations: i64,
    #[serde(rename = "maxPayloadSize")]
    pub max_payload_size: i64,
}

/// Advertises no bulk support. RFC 7643 §5 marks `maxOperations` and
/// `maxPayloadSize` REQUIRED, so they are present but zero: a non-zero limit
/// alongside `supported: false` claims a capacity the server does not have.
///
/// Before 1.0 this returned 1000 and 1048576 while
/// `ServiceProviderConfig::default()` hand-built the same struct with zeros,
/// so the two disagreed depending on which constructor you went through.
impl Default for Bulk {
    fn default() -> Self {
        Bulk {
            supported: false,
            max_operations: 0,
            max_payload_size: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Supported {
    pub supported: bool,
}

/// Converts a JSON string into a `ServiceProviderConfig` struct.
///
/// This method attempts to parse a JSON string to construct a `ServiceProviderConfig` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `ServiceProviderConfig` object.
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

impl ServiceProviderConfig {}

impl Validate for ServiceProviderConfig {
    /// RFC 7643 §5 marks `authenticationSchemes` REQUIRED, so an empty list is
    /// a conformance failure. `patch`, `bulk`, `filter`, `changePassword`,
    /// `sort` and `etag` are REQUIRED too, but they are non-`Option` fields, so
    /// `serde` already refuses a payload that omits them.
    ///
    /// Before 1.0 this returned `MissingRequiredField` whenever any of those
    /// six reported `supported: false`, which rejected valid configurations:
    /// §5 makes the `supported` **field** required, not its value true. A
    /// server that does not implement bulk correctly advertises
    /// `"bulk": {"supported": false}`.
    fn validate(&self) -> Result<(), ValidationError> {
        if self.authentication_schemes.is_empty() {
            return Err(ValidationError::missing_required("authenticationSchemes"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    /// RFC 7643 §5's example ServiceProviderConfig, as reproduced in this
    /// module's own documentation.
    const RFC_S5_EXAMPLE: &str = r#"{
        "schemas": ["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig"],
        "documentationUri": "http://example.com/help/scim.html",
        "patch": { "supported": true },
        "bulk": { "supported": true, "maxOperations": 1000, "maxPayloadSize": 1048576 },
        "filter": { "supported": true, "maxResults": 200 },
        "changePassword": { "supported": true },
        "sort": { "supported": true },
        "etag": { "supported": true },
        "authenticationSchemes": [{
            "name": "OAuth Bearer Token",
            "description": "Authentication scheme using the OAuth Bearer Token Standard",
            "specUri": "http://www.rfc-editor.org/info/rfc6750",
            "documentationUri": "http://example.com/help/oauth.html",
            "type": "oauthbearertoken",
            "primary": true
        }]
    }"#;

    /// The `schemas` attribute is modelled from 1.0 on. RFC 7643 §5's own
    /// example carries it, and before 1.0 there was no field to hold it, so a
    /// present value was dropped and never round-tripped.
    #[test]
    fn schemas_round_trips() {
        let config = ServiceProviderConfig::try_from(RFC_S5_EXAMPLE)
            .expect("RFC 7643 §5 example must deserialize");
        assert_eq!(config.schemas, vec![schema_urns::SERVICE_PROVIDER_CONFIG]);

        let back = serde_json::to_value(&config).unwrap();
        assert_eq!(
            back["schemas"],
            serde_json::json!([schema_urns::SERVICE_PROVIDER_CONFIG])
        );
    }

    /// RFC 7643 §§6-7 let a discovery resource arrive without `schemas`, so
    /// absence must not be an error; `#[serde(default)]` leaves it empty.
    #[test]
    fn missing_schemas_is_tolerated() {
        let json = RFC_S5_EXAMPLE.replace(
            r#""schemas": ["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig"],"#,
            "",
        );
        let config = ServiceProviderConfig::try_from(json.as_str())
            .expect("a config without `schemas` must still deserialize");
        assert!(config.schemas.is_empty());
    }

    /// RFC 7643 §5 marks `authenticationSchemes` REQUIRED, and it is the only
    /// attribute on this resource that `serde` cannot already enforce.
    #[test]
    fn validate_requires_authentication_schemes() {
        let mut config = ServiceProviderConfig::try_from(RFC_S5_EXAMPLE).unwrap();
        assert!(config.validate().is_ok());

        config.authentication_schemes.clear();
        let err = config.validate().expect_err("empty list must fail");
        assert_eq!(err.path(), "authenticationSchemes");
        assert_eq!(err.scim_type(), "invalidValue");
    }

    /// Regression guard for the pre-1.0 bug. §5 makes the `supported` *field*
    /// required, not its value true: a server that does not implement bulk
    /// advertises `"bulk": {"supported": false}`, and that is conformant.
    /// `validate` used to reject it.
    #[test]
    fn a_server_supporting_nothing_is_still_valid() {
        let mut config = ServiceProviderConfig::try_from(RFC_S5_EXAMPLE).unwrap();
        config.patch.supported = false;
        config.bulk.supported = false;
        config.filter.supported = false;
        config.change_password.supported = false;
        config.sort.supported = false;
        config.etag.supported = false;

        assert!(
            config.validate().is_ok(),
            "advertising no optional features is conformant, not invalid"
        );
    }

    /// `TryFrom<&str>` surfaces malformed JSON rather than panicking.
    #[test]
    fn try_from_rejects_malformed_json() {
        assert!(ServiceProviderConfig::try_from("{").is_err());
        assert!(ServiceProviderConfig::try_from(r#"{"patch": 7}"#).is_err());
    }

    /// `Default` is the "supports nothing" config, which needs to be
    /// constructible and serializable even though it fails `validate` for
    /// want of an authentication scheme.
    #[test]
    fn default_is_constructible_and_advertises_nothing() {
        let config = ServiceProviderConfig::default();
        assert_eq!(config.schemas, vec![schema_urns::SERVICE_PROVIDER_CONFIG]);
        assert!(!config.patch.supported);
        assert!(!config.bulk.supported);
        assert_eq!(config.bulk.max_operations, 0);
        assert_eq!(config.filter.max_results, 0);
        assert!(config.authentication_schemes.is_empty());
        assert!(config.validate().is_err());

        // The nested Defaults are reachable on their own too.
        assert!(!Filter::default().supported);
        assert_eq!(Bulk::default().max_payload_size, 0);
        assert!(AuthenticationScheme::default().name.is_empty());

        serde_json::to_string(&config).expect("Default must serialize");
    }

    #[test]
    fn service_provider_config_deserialization() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig"],
            "documentationUri": "http://example.com/help/scim.html",
            "patch": { "supported": true },
            "bulk": {
                "supported": true,
                "maxOperations": 1000,
                "maxPayloadSize": 1048576
            },
            "filter": {
                "supported": true,
                "maxResults": 200
            },
            "changePassword": { "supported": true },
            "sort": { "supported": true },
            "etag": { "supported": true },
            "authenticationSchemes": [
                {
                    "name": "OAuth Bearer Token",
                    "description": "Authentication scheme using the OAuth Bearer Token Standard",
                    "specUri": "http://www.rfc-editor.org/info/rfc6750",
                    "documentationUri": "http://example.com/help/oauth.html",
                    "type": "oauthbearertoken",
                    "primary": true
                },
                {
                    "name": "HTTP Basic",
                    "description": "Authentication scheme using the HTTP Basic Standard",
                    "specUri": "http://www.rfc-editor.org/info/rfc2617",
                    "documentationUri": "http://example.com/help/httpBasic.html",
                    "type": "httpbasic"
                }
            ]
        }"#;

        let config: Result<ServiceProviderConfig, serde_json::Error> =
            serde_json::from_str(json_data);

        if let Err(e) = &config {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.documentation_uri,
            Some("http://example.com/help/scim.html".to_string())
        );
        assert_eq!(config.patch.supported, true);
        assert_eq!(config.bulk.supported, true);
        assert_eq!(config.bulk.max_operations, 1000);
        assert_eq!(config.bulk.max_payload_size, 1048576);
        assert_eq!(config.filter.supported, true);
        assert_eq!(config.filter.max_results, 200);
        assert_eq!(config.change_password.supported, true);
        assert_eq!(config.sort.supported, true);
        assert_eq!(config.etag.supported, true);
        assert_eq!(config.authentication_schemes.len(), 2);
        let oauth_scheme = &config.authentication_schemes[0];
        assert_eq!(oauth_scheme.name, "OAuth Bearer Token");
        assert_eq!(
            oauth_scheme.description,
            "Authentication scheme using the OAuth Bearer Token Standard"
        );
        assert_eq!(
            oauth_scheme.spec_uri,
            "http://www.rfc-editor.org/info/rfc6750"
        );
        assert_eq!(
            oauth_scheme.documentation_uri,
            Some("http://example.com/help/oauth.html".to_string())
        );
        assert_eq!(oauth_scheme.r#type, "oauthbearertoken");
        assert_eq!(oauth_scheme.primary, Some(true));
        let http_scheme = &config.authentication_schemes[1];
        assert_eq!(http_scheme.name, "HTTP Basic");
        assert_eq!(
            http_scheme.description,
            "Authentication scheme using the HTTP Basic Standard"
        );
        assert_eq!(
            http_scheme.spec_uri,
            "http://www.rfc-editor.org/info/rfc2617"
        );
        assert_eq!(
            http_scheme.documentation_uri,
            Some("http://example.com/help/httpBasic.html".to_string())
        );
        assert_eq!(http_scheme.r#type, "httpbasic");
    }
}
