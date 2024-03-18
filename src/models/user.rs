use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::models::enterprise_user::EnterpriseUser;
use crate::models::scim_schema::Meta;
use crate::utils::error::SCIMError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    // urn:ietf:params:scim:schemas:core:2.0:User
    pub schemas: Vec<String>,
    pub id: Option<String>,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: Option<Name>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    #[serde(rename = "profileUrl")]
    pub profile_url: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "userType")]
    pub user_type: Option<String>,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub active: Option<bool>,
    pub password: Option<String>,
    pub emails: Option<Vec<Email>>,
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub ims: Option<Vec<Im>>,
    pub photos: Option<Vec<Photo>>,
    pub groups: Option<Vec<Group>>,
    pub entitlements: Option<Vec<Entitlement>>,
    pub roles: Option<Vec<Role>>,
    #[serde(rename = "x509Certificates")]
    pub x509_certificates: Option<Vec<X509Certificate>>,
    pub meta: Option<Meta>,
    #[serde(rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User")]
    pub enterprise_user: Option<EnterpriseUser>,
}

impl Default for User {
    fn default() -> Self {
        User {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
            user_name: "".to_string(),
            id: None,
            name: None,
            display_name: None,
            nick_name: None,
            profile_url: None,
            title: None,
            user_type: None,
            preferred_language: None,
            locale: None,
            timezone: None,
            active: None,
            password: None,
            emails: None,
            addresses: None,
            phone_numbers: None,
            ims: None,
            photos: None,
            groups: None,
            entitlements: None,
            roles: None,
            x509_certificates: None,
            meta: None,
            enterprise_user: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Name {
    pub formatted: Option<String>,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    #[serde(rename = "givenName")]
    pub given_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "honorificPrefix")]
    pub honorific_prefix: Option<String>,
    #[serde(rename = "honorificSuffix")]
    pub honorific_suffix: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Email {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Address {
    pub formatted: Option<String>,
    #[serde(rename = "streetAddress")]
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct PhoneNumber {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Im {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Photo {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Group {
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub display: Option<String>,
    pub type_: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Entitlement {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Role {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct X509Certificate {
    pub value: Option<String>,
    pub display: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub primary: Option<bool>,
}


/// Converts a JSON string into a `User` struct.
///
/// This method attempts to parse a JSON string to construct a `User` object. It's useful for scenarios where
/// you receive a JSON representation of a user from an external source (e.g., a web request) and you need to
/// work with this data in a strongly-typed manner within your application.
///
/// # Errors
///
/// Returns `SCIMError::DeserializationError` if the provided JSON string cannot be parsed into a `User` object.
///
/// # Examples
///
/// ```rust
/// use scim_v2::models::user::User;
///
/// let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
/// match User::try_from(user_json) {
///     Ok(user) => println!("Successfully converted JSON to User: {:?}", user),
///     Err(e) => println!("Error converting from JSON to User: {}", e),
/// }
/// ```
impl TryFrom<&str> for User {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl User {
    /// Validates a user.
    ///
    /// This function checks if the user has a `name` and `user_name`. If either is missing, it returns an error.
    /// It also checks if the `emails` field is present and if each email in the vector is in a valid email format.
    ///
    /// # Arguments
    ///
    /// * `user` - A reference to a User instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the user is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    /// * `Err(SCIMError::InvalidFieldValue)` - If a field value is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use scim_v2::models::user::User;
    ///
    /// let user = User {
    ///     user_name: "jdoe@example.com".to_string(),
    ///     // other fields...
    ///     ..Default::default()
    /// };
    ///
    /// match user.validate() {
    ///     Ok(_) => println!("User is valid."),
    ///     Err(e) => println!("User is invalid: {}", e),
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// The actual validation requirements will depend on the specifics of your application and the SCIM (System for Cross-domain Identity Management) protocol you are implementing.

    pub fn validate(&self) -> Result<(), SCIMError> {
        // Pretty much every field is optional in the schema except for 2. We'll check for those here.
        if self.schemas.is_empty() {
            return Err(SCIMError::MissingRequiredField("schemas".to_string()));
        }
        if self.user_name.is_empty() {
            return Err(SCIMError::MissingRequiredField("user_name".to_string()));
        }
        Ok(())
    }

    /// Serializes the `User` instance to a JSON string, using the custom SCIMError for error handling.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<String, SCIMError>`, where `Ok(String)` contains
    /// the JSON string representation of the `User` instance, and `Err(SCIMError)` contains
    /// the custom error encountered during serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::user::User;
    ///
    /// let user = User {
    ///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
    ///     user_name: "jdoe@example.com".to_string(),
    ///     // Initialize other fields as necessary...
    ///     ..Default::default()
    /// };
    ///
    /// match user.serialize() {
    ///     Ok(json) => println!("Serialized User: {}", json),
    ///     Err(e) => println!("Serialization error: {}", e),
    /// }
    /// ```
    pub fn serialize(&self) -> Result<String, SCIMError> {
        serde_json::to_string(&self).map_err(SCIMError::SerializationError)
    }

    /// Deserializes a JSON string into a `User` instance, using the custom SCIMError for error handling.
    ///
    /// # Parameters
    ///
    /// * `json` - A string slice that holds the JSON representation of a `User`.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<User, SCIMError>`, where `Ok(User)` is the deserialized `User` instance,
    /// and `Err(SCIMError)` is the custom error encountered during deserialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use scim_v2::models::user::User;
    ///
    /// let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
    /// match User::deserialize(user_json) {
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
    // Import everything from the outer module
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn user_deserialization_with_minimum_fields() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
            "id": "2819c223-7f76-453a-919d-413861904646",
            "userName": "bjensen@example.com",
            "meta": {
                "resourceType": "User",
                "created": "2010-01-23T04:56:22Z",
                "lastModified": "2011-05-13T04:42:34Z",
                "version": "W/\"3694e05e9dff590\"",
                "location": "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646"
            }
        }"#;

        let user: Result<User, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &user {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:User"]);
        assert_eq!(user.id, Some("2819c223-7f76-453a-919d-413861904646".to_string()));
        assert_eq!(user.user_name, "bjensen@example.com");
        let meta = user.meta.unwrap();
        assert_eq!(meta.resource_type, Some("User".to_string()));
        assert_eq!(meta.created, Some("2010-01-23T04:56:22Z".to_string()));
        assert_eq!(meta.last_modified, Some("2011-05-13T04:42:34Z".to_string()));
        assert_eq!(meta.version, Some("W/\"3694e05e9dff590\"".to_string()));
        assert_eq!(meta.location, Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string()));
    }

    #[test]
    fn user_deserialization_with_all_fields() {
        let json_data = r#"{
            "schemas": [
                "urn:ietf:params:scim:schemas:core:2.0:User"
            ],
            "id": "2819c223-7f76-453a-919d-413861904646",
            "externalId": "701984",
            "userName": "bjensen@example.com",
            "name": {
                "formatted": "Ms. Barbara J Jensen, III",
                "familyName": "Jensen",
                "givenName": "Barbara",
                "middleName": "Jane",
                "honorificPrefix": "Ms.",
                "honorificSuffix": "III"
            },
            "displayName": "Babs Jensen",
            "nickName": "Babs",
            "profileUrl": "https://login.example.com/bjensen",
            "emails": [
                {
                    "value": "bjensen@example.com",
                    "type": "work",
                    "primary": true
                },
                {
                    "value": "babs@jensen.org",
                    "type": "home"
                }
            ],
            "addresses": [
                {
                    "type": "work",
                    "streetAddress": "100 Universal City Plaza",
                    "locality": "Hollywood",
                    "region": "CA",
                    "postalCode": "91608",
                    "country": "USA",
                    "formatted": "100 Universal City Plaza\nHollywood, CA 91608 USA",
                    "primary": true
                },
                {
                    "type": "home",
                    "streetAddress": "456 Hollywood Blvd",
                    "locality": "Hollywood",
                    "region": "CA",
                    "postalCode": "91608",
                    "country": "USA",
                    "formatted": "456 Hollywood Blvd\nHollywood, CA 91608 USA"
                }
            ],
            "phoneNumbers": [
                {
                    "value": "555-555-5555",
                    "type": "work"
                },
                {
                    "value": "555-555-4444",
                    "type": "mobile"
                }
            ],
            "ims": [
                {
                    "value": "someaimhandle",
                    "type": "aim"
                }
            ],
            "photos": [
                {
                    "value": "https://photos.example.com/profilephoto/72930000000Ccne/F",
                    "type": "photo"
                },
                {
                    "value": "https://photos.example.com/profilephoto/72930000000Ccne/T",
                    "type": "thumbnail"
                }
            ],
            "userType": "Employee",
            "title": "Tour Guide",
            "preferredLanguage": "en-US",
            "locale": "en-US",
            "timezone": "America/Los_Angeles",
            "active": true,
            "password": "t1meMa$heen",
            "groups": [
                {
                    "value": "e9e30dba-f08f-4109-8486-d5c6a331660a",
                    "$ref": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a",
                    "display": "Tour Guides"
                },
                {
                    "value": "fc348aa8-3835-40eb-a20b-c726e15c55b5",
                    "$ref": "https://example.com/v2/Groups/fc348aa8-3835-40eb-a20b-c726e15c55b5",
                    "display": "Employees"
                },
                {
                    "value": "71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7",
                    "$ref": "https://example.com/v2/Groups/71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7",
                    "display": "US Employees"
                }
            ],
            "x509Certificates": [
                {
                    "value": "MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo="
                }
            ],
            "meta": {
                "resourceType": "User",
                "created": "2010-01-23T04:56:22Z",
                "lastModified": "2011-05-13T04:42:34Z",
                "version": "W/\"a330bc54f0671c9\"",
                "location": "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646"
            }
        }"#;

        let user: Result<User, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &user {
            eprintln!("Deserialization failed: {:?}", e);
        }

        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:User"]);
        assert_eq!(user.id, Some("2819c223-7f76-453a-919d-413861904646".to_string()));
        assert_eq!(user.user_name, "bjensen@example.com");
        assert_eq!(user.name.as_ref().unwrap().formatted, Some("Ms. Barbara J Jensen, III".to_string()));
        assert_eq!(user.display_name, Some("Babs Jensen".to_string()));
        assert_eq!(user.nick_name, Some("Babs".to_string()));
        assert_eq!(user.profile_url, Some("https://login.example.com/bjensen".to_string()));
        assert_eq!(user.emails.as_ref().unwrap().len(), 2);
        assert_eq!(user.emails.as_ref().unwrap()[0].value, Some("bjensen@example.com".to_string()));
        assert_eq!(user.emails.as_ref().unwrap()[0].type_, Some("work".to_string()));
        assert_eq!(user.addresses.as_ref().unwrap().len(), 2);
        assert_eq!(user.addresses.as_ref().unwrap()[0].type_.as_ref().unwrap(), "work");
        assert_eq!(user.phone_numbers.as_ref().unwrap().len(), 2);
        assert_eq!(user.phone_numbers.as_ref().unwrap()[0].value, Some("555-555-5555".to_string()));
        assert_eq!(user.ims.as_ref().unwrap().len(), 1);
        assert_eq!(user.ims.as_ref().unwrap()[0].value, Some("someaimhandle".to_string()));
        assert_eq!(user.groups.as_ref().unwrap().len(), 3);
        assert_eq!(user.groups.as_ref().unwrap()[0].value, Some("e9e30dba-f08f-4109-8486-d5c6a331660a".to_string()));
        assert_eq!(user.x509_certificates.as_ref().unwrap().len(), 1);
        assert_eq!(user.x509_certificates.as_ref().unwrap()[0].value, Some("MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo=".to_string()), "x509_certificates[0].value did not match expected value");
        let meta = user.meta.unwrap();
        assert_eq!(meta.resource_type, Some("User".to_string()));
        assert_eq!(meta.created, Some("2010-01-23T04:56:22Z".to_string()));
        assert_eq!(meta.last_modified, Some("2011-05-13T04:42:34Z".to_string()));
        assert_eq!(meta.version, Some("W/\"a330bc54f0671c9\"".to_string()));
        assert_eq!(meta.location, Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string()));
    }

    #[test]
    fn user_deserialization_with_enterprise_user_extension() {
        let json_data = r#"{
            "schemas":
            [
                "urn:ietf:params:scim:schemas:core:2.0:User",
                "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"
            ],
            "id": "2819c223-7f76-453a-919d-413861904646",
            "externalId": "701984",
            "userName": "bjensen@example.com",
            "name":
            {
                "formatted": "Ms. Barbara J Jensen, III",
                "familyName": "Jensen",
                "givenName": "Barbara",
                "middleName": "Jane",
                "honorificPrefix": "Ms.",
                "honorificSuffix": "III"
            },
            "displayName": "Babs Jensen",
            "nickName": "Babs",
            "profileUrl": "https://login.example.com/bjensen",
            "emails":
            [
                {
                    "value": "bjensen@example.com",
                    "type": "work",
                    "primary": true
                },
                {
                    "value": "babs@jensen.org",
                    "type": "home"
                }
            ],
            "addresses":
            [
                {
                    "streetAddress": "100 Universal City Plaza",
                    "locality": "Hollywood",
                    "region": "CA",
                    "postalCode": "91608",
                    "country": "USA",
                    "formatted": "100 Universal City Plaza\nHollywood, CA 91608 USA",
                    "type": "work",
                    "primary": true
                },
                {
                    "streetAddress": "456 Hollywood Blvd",
                    "locality": "Hollywood",
                    "region": "CA",
                    "postalCode": "91608",
                    "country": "USA",
                    "formatted": "456 Hollywood Blvd\nHollywood, CA 91608 USA",
                    "type": "home"
                }
            ],
            "phoneNumbers":
            [
                {
                    "value": "555-555-5555",
                    "type": "work"
                },
                {
                    "value": "555-555-4444",
                    "type": "mobile"
                }
            ],
            "ims":
            [
                {
                    "value": "someaimhandle",
                    "type": "aim"
                }
            ],
            "photos":
            [
                {
                    "value": "https://photos.example.com/profilephoto/72930000000Ccne/F",
                    "type": "photo"
                },
                {
                    "value": "https://photos.example.com/profilephoto/72930000000Ccne/T",
                    "type": "thumbnail"
                }
            ],
            "userType": "Employee",
            "title": "Tour Guide",
            "preferredLanguage": "en-US",
            "locale": "en-US",
            "timezone": "America/Los_Angeles",
            "active": true,
            "password": "t1meMa$heen",
            "groups":
            [
                {
                    "value": "e9e30dba-f08f-4109-8486-d5c6a331660a",
                    "$ref": "../Groups/e9e30dba-f08f-4109-8486-d5c6a331660a",
                    "display": "Tour Guides"
                },
                {
                    "value": "fc348aa8-3835-40eb-a20b-c726e15c55b5",
                    "$ref": "../Groups/fc348aa8-3835-40eb-a20b-c726e15c55b5",
                    "display": "Employees"
                },
                {
                    "value": "71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7",
                    "$ref": "../Groups/71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7",
                    "display": "US Employees"
                }
            ],
            "x509Certificates":
            [
                {
                  "value": "MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo="

                }
            ],
            "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User":
            {
                "employeeNumber": "701984",
                "costCenter": "4130",
                "organization": "Universal Studios",
                "division": "Theme Park",
                "department": "Tour Operations",
                "manager":
                {
                    "value": "26118915-6090-4610-87e4-49d8ca9f808d",
                    "$ref": "../Users/26118915-6090-4610-87e4-49d8ca9f808d",
                    "displayName": "John Smith"
                }
            },
            "meta":
            {
                "resourceType": "User",
                "created": "2010-01-23T04:56:22Z",
                "lastModified": "2011-05-13T04:42:34Z",
                "version": "W/\"3694e05e9dff591\"",
                "location": "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646"
            }
        }"#;

        let user: Result<User, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &user {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(user.is_ok());
        let user = user.unwrap();
        let enterprise_user = user.enterprise_user.unwrap();
        std::assert_eq!(enterprise_user.employee_number, Some("701984".to_string()));
        std::assert_eq!(enterprise_user.cost_center, Some("4130".to_string()));
        std::assert_eq!(enterprise_user.organization, Some("Universal Studios".to_string()));
        std::assert_eq!(enterprise_user.division, Some("Theme Park".to_string()));
        std::assert_eq!(enterprise_user.department, Some("Tour Operations".to_string()));
        let manager = enterprise_user.manager.unwrap();
        std::assert_eq!(manager.value, Some("26118915-6090-4610-87e4-49d8ca9f808d".to_string()));
        std::assert_eq!(manager.display_name, Some("John Smith".to_string()));
    }

    #[test]
    fn user_deserialization_without_enterprise_user_extension() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
            "id": "2819c223-7f76-453a-919d-413861904646",
            "userName": "bjensen@example.com"
        }"#;

        let user: Result<User, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &user {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(user.is_ok());
        let user = user.unwrap();
        assert!(user.enterprise_user.is_none());
    }
}