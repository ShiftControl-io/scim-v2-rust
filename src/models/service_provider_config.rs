use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationScheme {
    pub name: String,
    pub description: String,
    #[serde(rename = "specUri")]
    pub spec_uri: String,
    #[serde(rename = "documentationUri")]
    pub documentation_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub supported: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bulk {
    pub supported: bool,
    #[serde(rename = "maxOperations")]
    pub max_operations: i64,
    #[serde(rename = "maxPayloadSize")]
    pub max_payload_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supported {
    pub supported: bool,
}

impl Default for ServiceProviderConfig {
    fn default() -> Self {
        ServiceProviderConfig {
            documentation_uri: None,
            patch: Supported { supported: false },
            bulk: Bulk {
                supported: false,
                max_operations: 0,
                max_payload_size: 0
            },
            filter: Filter {
                supported: false,
                max_results: 0
            },
            change_password: Supported { supported: false },
            sort: Supported { supported: false },
            etag: Supported { supported: false },
            authentication_schemes: vec![],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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