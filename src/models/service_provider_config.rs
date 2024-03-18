use serde::{Deserialize, Serialize};

use crate::utils::error::SCIMError;
use crate::models::scim_schema::Meta;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceProviderConfig {
    #[serde(rename = "documentationUri")]
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
    pub meta: Option<Meta>,
}

impl Default for ServiceProviderConfig {
    fn default() -> Self {
        ServiceProviderConfig {
            documentation_uri: None,
            patch: Supported { supported: false },
            bulk: Bulk {
                supported: false,
                max_operations: 0,
                max_payload_size: 0,
            },
            filter: Filter {
                supported: false,
                max_results: 0,
            },
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
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    #[serde(rename = "specUri")]
    pub spec_uri: String,
    #[serde(rename = "documentationUri")]
    pub documentation_uri: String,
    pub primary: Option<bool>,
}

impl Default for AuthenticationScheme {
    fn default() -> Self {
        AuthenticationScheme {
            name: "".to_string(),
            type_: "".to_string(),
            description: "".to_string(),
            spec_uri: "".to_string(),
            documentation_uri: "".to_string(),
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

impl Default for Filter {
    fn default() -> Self {
        Filter {
            supported: false,
            max_results: 100,
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

impl Default for Bulk {
    fn default() -> Self {
        Bulk {
            supported: false,
            max_operations: 1000,
            max_payload_size: 1048576,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
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

impl ServiceProviderConfig {
    /// Validates a service provider config.
    ///
    /// This function checks if the service provider config has `patch`, `bulk`, `filter`, `change_password`, `sort`, and `etag`. If any of these fields are missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to a ServiceProviderConfig instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the service provider config is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    ///
    /// # Example
    ///
    /// ```
    /// use scim_v2::models::service_provider_config::ServiceProviderConfig;
    ///
    /// let config = ServiceProviderConfig {
    ///     // Initialize config fields here...
    ///     // ...
    ///     ..Default::default()
    /// };
    ///
    /// match config.validate() {
    ///     Ok(_) => println!("ServiceProviderConfig is valid."),
    ///     Err(e) => println!("ServiceProviderConfig is invalid: {}", e),
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), SCIMError> {
        if !self.patch.supported {
            return Err(SCIMError::MissingRequiredField("patch".to_string()));
        }
        if !self.bulk.supported {
            return Err(SCIMError::MissingRequiredField("bulk".to_string()));
        }
        if !self.filter.supported {
            return Err(SCIMError::MissingRequiredField("filter".to_string()));
        }
        if !self.change_password.supported {
            return Err(SCIMError::MissingRequiredField("change_password".to_string()));
        }
        if !self.sort.supported {
            return Err(SCIMError::MissingRequiredField("sort".to_string()));
        }
        if !self.etag.supported {
            return Err(SCIMError::MissingRequiredField("etag".to_string()));
        }
        Ok(())
    }

    /// Serializes the `ServiceProviderConfig` instance to a JSON string, using the custom SCIMError for error handling.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<String, SCIMError>`, where `Ok(String)` contains
    /// the JSON string representation of the `ServiceProviderConfig` instance, and `Err(SCIMError)` contains
    /// the custom error encountered during serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::service_provider_config::ServiceProviderConfig;
    ///
    /// let config = ServiceProviderConfig {
    ///     // Initialize config fields here...
    ///     // ...
    ///     ..Default::default()
    /// };
    ///
    /// match config.serialize() {
    ///     Ok(json) => println!("Serialized ServiceProviderConfig: {}", json),
    ///     Err(e) => println!("Serialization error: {}", e),
    /// }
    /// ```
    pub fn serialize(&self) -> Result<String, SCIMError> {
        serde_json::to_string(&self).map_err(SCIMError::SerializationError)
    }

    /// Deserializes a JSON string into a `ServiceProviderConfig` instance, using the custom SCIMError for error handling.
    ///
    /// # Parameters
    ///
    /// * `json` - A string slice that holds the JSON representation of a `ServiceProviderConfig`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<ServiceProviderConfig, SCIMError>`, where `Ok(ServiceProviderConfig)` is the deserialized `ServiceProviderConfig` instance,
    /// and `Err(SCIMError)` is the custom error encountered during deserialization.
    ///
    /// # Examples
    ///
    /// ```
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
    /// match ServiceProviderConfig::deserialize(config_json) {
    ///     Ok(user) => println!("Deserialized User: {:?}", user),
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
                    "documentationUri": "http://example.com/help/oauth.html"
                },
                {
                    "name": "HTTP Basic",
                    "description": "Authentication scheme using the HTTP Basic Standard",
                    "specUri": "http://www.rfc-editor.org/info/rfc2617",
                    "documentationUri": "http://example.com/help/httpBasic.html"
                }
            ]
        }"#;

        let config: Result<ServiceProviderConfig, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &config {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.documentation_uri, Some("http://example.com/help/scim.html".to_string()));
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
        assert_eq!(oauth_scheme.description, "Authentication scheme using the OAuth Bearer Token Standard");
        assert_eq!(oauth_scheme.spec_uri, "http://www.rfc-editor.org/info/rfc6750");
        assert_eq!(oauth_scheme.documentation_uri, "http://example.com/help/oauth.html");
        let http_scheme = &config.authentication_schemes[1];
        assert_eq!(http_scheme.name, "HTTP Basic");
        assert_eq!(http_scheme.description, "Authentication scheme using the HTTP Basic Standard");
        assert_eq!(http_scheme.spec_uri, "http://www.rfc-editor.org/info/rfc2617");
        assert_eq!(http_scheme.documentation_uri, "http://example.com/help/httpBasic.html");
    }
}