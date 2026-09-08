use crate::utils::validation::{
    Validate, ValidationError, at_most_one_primary, require_schema_urn,
};
use serde::{Deserialize, Serialize};

use crate::models::scim_schema::Meta;
use crate::schema_urns;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(
        rename = "authenticationSchemes",
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthenticationScheme {
    pub name: String,
    /// RFC 7643 §5 marks `type` REQUIRED ("oauth", "oauth2",
    /// "oauthbearertoken", "httpbasic", "httpdigest"). It is `Option` here
    /// only because the §8.7 schema representation — which §8 itself calls
    /// non-normative — omits it, and real servers follow that text; a
    /// discovery document should still parse. [`Validate`] enforces the §5
    /// requirement.
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

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
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

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Default, Clone)]
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

impl Validate for AuthenticationScheme {
    /// RFC 7643 §5: `type`, `name` and `description` are REQUIRED; `specUri`
    /// and `documentationUri` are OPTIONAL.
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
    /// RFC 7643 §5 marks `authenticationSchemes` REQUIRED, so an empty list is
    /// a conformance failure, and each scheme is checked in turn with its
    /// index in the reported path. `patch`, `bulk`, `filter`,
    /// `changePassword`, `sort` and `etag` are REQUIRED too, but they are
    /// non-`Option` fields, so `serde` already refuses a payload that omits
    /// them.
    ///
    /// Before 1.0 this returned `MissingRequiredField` whenever any of those
    /// six reported `supported: false`, which rejected valid configurations:
    /// §5 makes the `supported` **field** required, not its value true. A
    /// server that does not implement bulk correctly advertises
    /// `"bulk": {"supported": false}`.
    fn validate(&self) -> Result<(), ValidationError> {
        // §5: "id is not required"; `schemas` may be absent per §§6-7 practice,
        // but when present it must name this resource.
        if !self.schemas.is_empty() {
            require_schema_urn(&self.schemas, schema_urns::SERVICE_PROVIDER_CONFIG)?;
        }
        if self.authentication_schemes.is_empty() {
            return Err(ValidationError::missing_required("authenticationSchemes"));
        }
        for (i, scheme) in self.authentication_schemes.iter().enumerate() {
            if let Err(e) = scheme.validate() {
                return Err(ValidationError::missing_required(format!(
                    "authenticationSchemes[{i}].{}",
                    e.path()
                )));
            }
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

    /// The ninth `primary` carrier. `CHANGELOG` claimed all nine shared the
    /// lenient deserializer while this one did not, so a provider stringifying
    /// booleans made the *entire* discovery document unparseable — the one call
    /// a client makes before it knows any of the provider's quirks.
    #[test]
    fn authentication_scheme_primary_accepts_a_stringified_boolean() {
        for raw in [
            r#"{"name":"OAuth Bearer Token","description":"d","primary":true}"#,
            r#"{"name":"OAuth Bearer Token","description":"d","primary":"true"}"#,
            r#"{"name":"OAuth Bearer Token","description":"d","primary":"True"}"#,
        ] {
            let scheme: AuthenticationScheme =
                serde_json::from_str(raw).unwrap_or_else(|e| panic!("{raw}: {e}"));
            assert_eq!(scheme.primary, Some(true), "{raw}");
        }
        let absent: AuthenticationScheme =
            serde_json::from_str(r#"{"name":"n","description":"d"}"#).unwrap();
        assert_eq!(absent.primary, None);
    }

    /// A scheme without `type` parses — the §8.7 representation omits it and
    /// real servers follow that text — but does not validate, because §5 marks
    /// it REQUIRED. Before 1.0 the field was a mandatory `String` and such a
    /// document could not be read at all.
    #[test]
    fn a_scheme_without_type_parses_but_does_not_validate() {
        let scheme: AuthenticationScheme =
            serde_json::from_str(r#"{"name":"HTTP Basic","description":"RFC 2617"}"#)
                .expect("name + description is the RFC's required set");
        assert_eq!(scheme.r#type, None);
        assert_eq!(scheme.spec_uri, None);
        assert_eq!(
            scheme.validate().expect_err("§5: type is REQUIRED").path(),
            "type"
        );

        let back = serde_json::to_value(&scheme).unwrap();
        let obj = back.as_object().unwrap();
        assert!(!obj.contains_key("type"), "unset type must be omitted");
        assert!(
            !obj.contains_key("specUri"),
            "unset specUri must be omitted"
        );
    }

    /// L-10: the two-diverging-defaults fix asserted each constructor on a
    /// different field subset, so re-hand-building the struct while a
    /// component default drifted would have passed. Assert the delegation
    /// invariant directly instead.
    #[test]
    fn service_provider_config_default_delegates_to_component_defaults() {
        let config = ServiceProviderConfig::default();
        assert_eq!(config.patch, Supported::default());
        assert_eq!(config.change_password, Supported::default());
        assert_eq!(config.sort, Supported::default());
        assert_eq!(config.etag, Supported::default());
        assert_eq!(config.bulk, Bulk::default());
        assert_eq!(config.filter, Filter::default());
    }

    /// RFC 7643 §5 marks `type`, `name` and `description` REQUIRED on every
    /// scheme. §8's schema representation omits `type`, but §8 describes
    /// itself as non-normative, so the prose governs. The lenient type lets
    /// the document parse; `validate` reports the gap with its index.
    #[test]
    fn validate_requires_type_name_and_description_on_each_scheme() {
        let mut config = ServiceProviderConfig::try_from(RFC_S5_EXAMPLE).unwrap();
        assert!(config.validate().is_ok());

        config.authentication_schemes.push(AuthenticationScheme {
            name: "HTTP Basic".to_string(),
            description: "RFC 2617".to_string(),
            r#type: None,
            ..Default::default()
        });
        let err = config.validate().expect_err("missing type");
        assert_eq!(err.path(), "authenticationSchemes[1].type");

        config.authentication_schemes[1].r#type = Some("httpbasic".to_string());
        config.authentication_schemes[1].description.clear();
        assert_eq!(
            config.validate().expect_err("missing description").path(),
            "authenticationSchemes[1].description"
        );
    }

    /// §2.4 applies to `authenticationSchemes` too: it is multi-valued and
    /// carries `primary`.
    #[test]
    fn validate_rejects_two_primary_schemes() {
        let mut config = ServiceProviderConfig::try_from(RFC_S5_EXAMPLE).unwrap();
        let mut second = config.authentication_schemes[0].clone();
        second.name = "Second".to_string();
        second.primary = Some(true);
        config.authentication_schemes.push(second);
        assert_eq!(
            config.validate().expect_err("two primaries").path(),
            "authenticationSchemes"
        );
    }

    /// R2-M6: `"authenticationSchemes": null` must reach `validate` as an
    /// empty list, so the caller gets the wire-path error rather than a raw
    /// serde message.
    #[test]
    fn authentication_schemes_null_collapses_and_reaches_validate() {
        let json = RFC_S5_EXAMPLE.replace(
            r#""authenticationSchemes": [{"#,
            r#""authenticationSchemes": null, "x": [{"#,
        );
        let config = ServiceProviderConfig::try_from(json.as_str())
            .expect("null authenticationSchemes must deserialize");
        assert!(config.authentication_schemes.is_empty());
        assert_eq!(
            config.validate().expect_err("REQUIRED").path(),
            "authenticationSchemes"
        );
    }

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
        assert_eq!(err.scim_type_str(), "invalidValue");
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
            oauth_scheme.spec_uri.as_deref(),
            Some("http://www.rfc-editor.org/info/rfc6750")
        );
        assert_eq!(
            oauth_scheme.documentation_uri,
            Some("http://example.com/help/oauth.html".to_string())
        );
        assert_eq!(oauth_scheme.r#type.as_deref(), Some("oauthbearertoken"));
        assert_eq!(oauth_scheme.primary, Some(true));
        let http_scheme = &config.authentication_schemes[1];
        assert_eq!(http_scheme.name, "HTTP Basic");
        assert_eq!(
            http_scheme.description,
            "Authentication scheme using the HTTP Basic Standard"
        );
        assert_eq!(
            http_scheme.spec_uri.as_deref(),
            Some("http://www.rfc-editor.org/info/rfc2617")
        );
        assert_eq!(
            http_scheme.documentation_uri,
            Some("http://example.com/help/httpBasic.html".to_string())
        );
        assert_eq!(http_scheme.r#type.as_deref(), Some("httpbasic"));
    }
}
