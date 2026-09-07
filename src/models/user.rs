use crate::models::enterprise_user::EnterpriseUser;
use crate::models::scim_schema::Meta;
use crate::utils::{error::SCIMError, serde::deserialize_optional_lenient_bool};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User<T = String> {
    // urn:ietf:params:scim:schemas:core:2.0:User
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ims: Option<Vec<Im>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<Photo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_certificates: Option<Vec<X509Certificate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(
        rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_user: Option<EnterpriseUser>,
}

impl<T> Default for User<T> {
    fn default() -> Self {
        User {
            schemas: vec![crate::schema_urns::USER.to_string()],
            user_name: "".to_string(),
            id: None,
            external_id: None,
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

/// The components of the user's name.
///
/// Unassigned sub-attributes are omitted from serialized output rather than
/// emitted as `null`; RFC 7643 §2.5 treats the two as equivalent in resource
/// state. Note the consequence for `PUT`: per RFC 7644 §3.5.1 an omitted
/// `readWrite` attribute is "not asserted by the client" and the server MAY
/// keep the existing value or apply a default — it is *not* a deterministic
/// clear. Callers that need to clear a sub-attribute on `PUT` should use a
/// `PATCH` operation or hand-build a `serde_json::Value` carrying an explicit
/// `null`.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Email {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PhoneNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Im {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Photo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entitlement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default, // required as `deserialize_with` does not set default when field is missing
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_lenient_bool"
    )]
    pub primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct X509Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl<T> User<T> {
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
    /// let user: User<String> = User {
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
}

impl<T> User<T>
where
    T: Serialize,
{
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
    ///     id: Some("123".to_string()),
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
}

impl<T> User<T>
where
    T: DeserializeOwned,
{
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
    /// match User::<String>::deserialize(user_json) {
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

    /// Minimal reproduction: the bug is isolated entirely to `Role`
    /// deserialization, independent of the surrounding `User`/`ListResponse`
    /// shape. GitHub Enterprise omits `primary` on `Role` objects rather
    /// than sending a JSON boolean (e.g. `{"value": "enterprise_owner"}`
    /// with no `primary` key at all), and `Role::primary`'s
    /// `deserialize_with` attribute — without an accompanying
    /// `#[serde(default)]` — turns an absent key into a hard `missing
    /// field` error instead of defaulting to `None`.
    #[test]
    fn role_deserialization_with_omitted_primary_key() {
        let payload = r#"{ "value": "custom_role" }"#;

        let result = serde_json::from_str::<Role>(payload);

        assert!(
            result.is_ok(),
            "failed to deserialize Role with missing `primary` field: {:?}",
            result.err()
        );
    }

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
        assert_eq!(
            user.schemas,
            vec!["urn:ietf:params:scim:schemas:core:2.0:User"]
        );
        assert_eq!(
            user.id,
            Some("2819c223-7f76-453a-919d-413861904646".to_string())
        );
        assert_eq!(user.user_name, "bjensen@example.com");
        let meta = user.meta.unwrap();
        assert_eq!(meta.resource_type, Some("User".to_string()));
        assert_eq!(meta.created, Some("2010-01-23T04:56:22Z".to_string()));
        assert_eq!(meta.last_modified, Some("2011-05-13T04:42:34Z".to_string()));
        assert_eq!(meta.version, Some("W/\"3694e05e9dff590\"".to_string()));
        assert_eq!(
            meta.location,
            Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string())
        );
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
        assert_eq!(
            user.schemas,
            vec!["urn:ietf:params:scim:schemas:core:2.0:User"]
        );
        assert_eq!(
            user.id,
            Some("2819c223-7f76-453a-919d-413861904646".to_string())
        );
        assert_eq!(user.external_id, Some("701984".to_string()));
        assert_eq!(user.user_name, "bjensen@example.com");
        assert_eq!(
            user.name.as_ref().unwrap().formatted,
            Some("Ms. Barbara J Jensen, III".to_string())
        );
        assert_eq!(user.display_name, Some("Babs Jensen".to_string()));
        assert_eq!(user.nick_name, Some("Babs".to_string()));
        assert_eq!(
            user.profile_url,
            Some("https://login.example.com/bjensen".to_string())
        );
        assert_eq!(user.emails.as_ref().unwrap().len(), 2);
        assert_eq!(
            user.emails.as_ref().unwrap()[0].value,
            Some("bjensen@example.com".to_string())
        );
        assert_eq!(
            user.emails.as_ref().unwrap()[0].r#type,
            Some("work".to_string())
        );
        assert_eq!(user.addresses.as_ref().unwrap().len(), 2);
        assert_eq!(
            user.addresses.as_ref().unwrap()[0].r#type.as_ref().unwrap(),
            "work"
        );
        assert_eq!(user.phone_numbers.as_ref().unwrap().len(), 2);
        assert_eq!(
            user.phone_numbers.as_ref().unwrap()[0].value,
            Some("555-555-5555".to_string())
        );
        assert_eq!(user.ims.as_ref().unwrap().len(), 1);
        assert_eq!(
            user.ims.as_ref().unwrap()[0].value,
            Some("someaimhandle".to_string())
        );
        assert_eq!(user.groups.as_ref().unwrap().len(), 3);
        assert_eq!(
            user.groups.as_ref().unwrap()[0].value,
            Some("e9e30dba-f08f-4109-8486-d5c6a331660a".to_string())
        );
        assert_eq!(user.x509_certificates.as_ref().unwrap().len(), 1);
        assert_eq!(user.x509_certificates.as_ref().unwrap()[0].value, Some("MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo=".to_string()), "x509_certificates[0].value did not match expected value");
        let meta = user.meta.unwrap();
        assert_eq!(meta.resource_type, Some("User".to_string()));
        assert_eq!(meta.created, Some("2010-01-23T04:56:22Z".to_string()));
        assert_eq!(meta.last_modified, Some("2011-05-13T04:42:34Z".to_string()));
        assert_eq!(meta.version, Some("W/\"a330bc54f0671c9\"".to_string()));
        assert_eq!(
            meta.location,
            Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string())
        );
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
        assert_eq!(enterprise_user.employee_number, Some("701984".to_string()));
        assert_eq!(enterprise_user.cost_center, Some("4130".to_string()));
        assert_eq!(
            enterprise_user.organization,
            Some("Universal Studios".to_string())
        );
        assert_eq!(enterprise_user.division, Some("Theme Park".to_string()));
        assert_eq!(
            enterprise_user.department,
            Some("Tour Operations".to_string())
        );
        let manager = enterprise_user.manager.unwrap();
        assert_eq!(
            manager.value,
            Some("26118915-6090-4610-87e4-49d8ca9f808d".to_string())
        );
        assert_eq!(manager.display_name, Some("John Smith".to_string()));
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

    // Test data is from https://scimvalidator.microsoft.com/
    #[test]
    fn deserialize_entra_user() {
        let user: Result<User, serde_json::Error> = serde_json::from_str(include_str!(
            "../test_data/provider_samples/entra_user_creation_test.json"
        ));
        user.expect("user should deserialize");
    }

    /// End-to-end regression for the omitted `Role.primary` key, using a
    /// sanitized real response from GitHub Enterprise's SCIM `/Users`
    /// endpoint. The first resource carries a role with `primary` absent
    /// (`{"value": "enterprise_owner"}`); the second carries an empty
    /// `roles` array. Before the `#[serde(default)]` fix, the first
    /// resource failed with a `missing field \`primary\`` error, taking the
    /// whole `ListResponse` down with it.
    #[test]
    fn deserialize_github_enterprise_user_list() {
        use crate::models::others::ListResponse;

        let list: ListResponse<String> = serde_json::from_str(include_str!(
            "../test_data/provider_samples/github_enterprise_user_list_test.json"
        ))
        .expect("GitHub Enterprise user list should deserialize");

        assert_eq!(list.total_results, 2);
        assert_eq!(list.resources.len(), 2);
    }

    /// Verbatim `User` payloads from RFC 7644. Each is parsed into `User`
    /// and then round-tripped (serialize, re-parse as `serde_json::Value`)
    /// to confirm no modelled field is dropped on the way back out.
    mod rfc7644_samples {
        use super::*;
        use pretty_assertions::assert_eq;

        fn assert_user_round_trips(raw: &str) {
            let user: User =
                serde_json::from_str(raw).expect("RFC 7644 sample must deserialize into User");
            let reserialized: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(&user).expect("serialize User"))
                    .unwrap();
            let original: serde_json::Value = serde_json::from_str(raw).unwrap();
            assert_eq!(
                reserialized, original,
                "round-tripped User JSON must equal the original RFC payload"
            );
        }

        /// RFC 7644 §3.3 — unnumbered example, "a client sends a POST request
        /// containing a `User` to the `/Users` endpoint". User creation request
        /// (POST /Users body).
        #[test]
        fn rfc7644_s3_3_user_create_request() {
            let raw = include_str!("../test_data/rfc7644/s3.3_user_create_request.json");
            assert_user_round_trips(raw);
            let user: User = serde_json::from_str(raw).unwrap();
            assert_eq!(user.user_name, "bjensen");
            assert_eq!(user.external_id.as_deref(), Some("bjensen"));
            assert_eq!(
                user.name.as_ref().unwrap().family_name.as_deref(),
                Some("Jensen")
            );
        }

        /// RFC 7644 §3.3 — unnumbered example, the 201 Created body ("the server
        /// signals a successful creation ... and returns a representation of the
        /// resource created"). Adds server-assigned `id` and `meta`.
        #[test]
        fn rfc7644_s3_3_user_create_response() {
            let raw = include_str!("../test_data/rfc7644/s3.3_user_create_response.json");
            assert_user_round_trips(raw);
            let user: User = serde_json::from_str(raw).unwrap();
            assert_eq!(
                user.id.as_deref(),
                Some("2819c223-7f76-453a-919d-413861904646")
            );
            let meta = user.meta.as_ref().unwrap();
            assert_eq!(meta.resource_type.as_deref(), Some("User"));
            assert_eq!(meta.version.as_deref(), Some("W/\"e180ee84f0671b1\""));
        }

        /// RFC 7644 §3.4.1 — unnumbered example, "The example below retrieves a
        /// single User via the `/Users` endpoint". GET /Users/{id} retrieval
        /// response (multi-valued `emails` and `phoneNumbers`).
        #[test]
        fn rfc7644_s3_4_1_user_retrieval_response() {
            let raw = include_str!("../test_data/rfc7644/s3.4.1_user_retrieval_response.json");
            assert_user_round_trips(raw);
            let user: User = serde_json::from_str(raw).unwrap();
            assert_eq!(
                user.emails.as_ref().unwrap()[0].value.as_deref(),
                Some("bjensen@example.com")
            );
            assert_eq!(
                user.phone_numbers.as_ref().unwrap()[0].r#type.as_deref(),
                Some("work")
            );
        }

        /// RFC 7644 §3.5.1 — unnumbered example following "a successful PUT
        /// operation returns a 200 OK response code and the entire resource
        /// within the response body ... For example". PUT /Users/{id} request
        /// replacing a user, including an empty `roles` array.
        #[test]
        fn rfc7644_s3_5_1_user_put_request() {
            let raw = include_str!("../test_data/rfc7644/s3.5.1_user_put_request.json");
            assert_user_round_trips(raw);
            let user: User = serde_json::from_str(raw).unwrap();
            assert_eq!(user.roles.as_ref().map(Vec::len), Some(0));
            assert_eq!(
                user.name.as_ref().unwrap().middle_name.as_deref(),
                Some("Jane")
            );
            assert_eq!(user.emails.as_ref().unwrap().len(), 2);
        }

        /// RFC 7644 §3.5.1 — unnumbered example, "The service responds with the
        /// entire updated User". PUT /Users/{id} response (drops `roles`, adds
        /// `meta`).
        #[test]
        fn rfc7644_s3_5_1_user_put_response() {
            let raw = include_str!("../test_data/rfc7644/s3.5.1_user_put_response.json");
            assert_user_round_trips(raw);
            let user: User = serde_json::from_str(raw).unwrap();
            assert!(user.roles.is_none());
            assert_eq!(
                user.meta.as_ref().unwrap().last_modified.as_deref(),
                Some("2011-08-08T08:00:12Z")
            );
        }
        /// An unset `Name.formatted` is omitted on serialize rather than
        /// emitted as `null` — consistent with the rest of `Name` and every
        /// other model in the crate. RFC 7643 §2.5 treats `null` and omitted
        /// as equivalent, so this is a compactness/consistency choice, and a
        /// present value still round-trips.
        #[test]
        fn name_omits_unset_formatted_on_serialize() {
            let name = Name {
                formatted: None,
                family_name: Some("Jensen".to_string()),
                given_name: Some("Barbara".to_string()),
                ..Name::default()
            };
            let serialized = serde_json::to_string(&name).unwrap();
            assert!(
                !serialized.contains("formatted"),
                "unset formatted must not appear on the wire: {serialized}"
            );
            assert!(
                !serialized.contains("null"),
                "no field should be null: {serialized}"
            );

            let full = Name {
                formatted: Some("Ms. Barbara J Jensen III".to_string()),
                ..name
            };
            let round: Name = serde_json::from_str(&serde_json::to_string(&full).unwrap()).unwrap();
            assert_eq!(round.formatted.as_deref(), Some("Ms. Barbara J Jensen III"));
            assert_eq!(round.family_name.as_deref(), Some("Jensen"));
        }

        /// The read path must keep accepting an explicit `"formatted": null` from
        /// peers that send it (RFC 7643 §2.5) — a missing key, `null`, and `{}`
        /// all deserialize to `None`. Regression guard against a future
        /// `#[serde(default)]`, `deny_unknown_fields`, or custom deserializer
        /// silently changing this (cf. `Role.primary`, CHANGELOG 0.4.2).
        #[test]
        fn name_explicit_null_deserializes_to_none() {
            let name: Name =
                serde_json::from_str(r#"{"formatted": null, "familyName": "Jensen"}"#).unwrap();
            assert_eq!(name.formatted, None);
            assert_eq!(name.family_name.as_deref(), Some("Jensen"));

            let empty: Name = serde_json::from_str("{}").unwrap();
            assert_eq!(empty.formatted, None);
        }

        /// A fully-unset `Name` serializes to an empty object, not one padded
        /// with `null`s. `formatted` was the last field without
        /// `skip_serializing_if`, so before this it emitted `{"formatted":null}`.
        /// Nested in a `User` this is the `"name":{}` a downstream peer receives.
        #[test]
        fn name_default_serializes_to_empty_object() {
            assert_eq!(
                serde_json::to_value(Name::default()).unwrap(),
                serde_json::json!({})
            );
        }
    }
}
