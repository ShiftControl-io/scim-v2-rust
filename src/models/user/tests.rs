// Import everything from the outer module
use pretty_assertions::assert_eq;
use test_case::test_case;

use super::*;

fn base_user() -> User<String> {
    User {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        ..Default::default()
    }
}

/// R2-M4: every branch of `validate_context`, as unit tests rather than
/// doctests, including the `password` branch that had no coverage at all.
#[test]
fn validate_context_enforces_direction_rules() {
    // CreateRequest: id MUST NOT be present (RFC 7643 §3.1).
    let with_id = User {
        id: Some("7".to_string()),
        ..base_user()
    };
    assert_eq!(
        with_id
            .validate_as(Context::CreateRequest)
            .expect_err("id on create")
            .path(),
        "id"
    );
    assert!(base_user().validate_as(Context::CreateRequest).is_ok());

    // ReplaceRequest tolerates id (RFC 7644 §3.5.1 ignores readOnly; its
    // PUT example carries one) and a password.
    let replace = User {
        password: Some("s3cret".to_string()),
        ..with_id.clone()
    };
    assert!(replace.validate_as(Context::ReplaceRequest).is_ok());

    // Response: id REQUIRED, password (returned: never) forbidden.
    assert_eq!(
        base_user()
            .validate_as(Context::Response)
            .expect_err("no id")
            .path(),
        "id"
    );
    assert_eq!(
        replace
            .validate_as(Context::Response)
            .expect_err("password")
            .path(),
        "password"
    );
    assert!(with_id.validate_as(Context::Response).is_ok());
}

/// `Valid` and `Strict` as unit tests: the wrapper is obtainable only by
/// passing, dereferences, and hands the value back.
#[test]
fn valid_and_strict_wrappers() {
    use crate::utils::validation::{CreateRequest, Strict, Valid};

    let valid = Valid::new(base_user(), Context::CreateRequest).expect("conformant");
    assert_eq!(valid.context(), Context::CreateRequest);
    assert_eq!(valid.user_name, "bjensen");
    let inner = valid.into_inner();
    assert!(
        Valid::new(inner, Context::Response).is_err(),
        "a response needs an id"
    );

    // R3-L4: a proven value goes back on the wire as the plain value, and
    // `Strict` dereferences through `Valid` to it.
    let valid = Valid::new(base_user(), Context::CreateRequest).expect("conformant");
    assert_eq!(
        serde_json::to_value(&valid).unwrap(),
        serde_json::to_value(base_user()).unwrap()
    );
    let body = serde_json::to_string(&base_user()).unwrap();
    let strict: Strict<User<String>, CreateRequest> = serde_json::from_str(&body).unwrap();
    assert_eq!(strict.user_name, "bjensen");
    assert_eq!(strict.context(), Context::CreateRequest);
    assert_eq!(
        serde_json::to_value(&strict).unwrap(),
        serde_json::to_value(base_user()).unwrap()
    );
    assert_eq!(strict.into_valid().user_name, "bjensen");

    let with_id = serde_json::to_string(&User {
        id: Some("7".to_string()),
        ..base_user()
    })
    .unwrap();
    let err = serde_json::from_str::<Strict<User<String>, CreateRequest>>(&with_id).unwrap_err();
    assert!(err.to_string().contains("id"), "{err}");
}

/// R2-L4: RFC 7643 §2.4's at-most-one-primary rule on every attribute that
/// carries `primary`, not only `emails`.
#[test_case("emails" ; "emails")]
#[test_case("phoneNumbers" ; "phone_numbers")]
#[test_case("ims" ; "ims")]
#[test_case("photos" ; "photos")]
#[test_case("addresses" ; "addresses")]
#[test_case("entitlements" ; "entitlements")]
#[test_case("roles" ; "roles")]
#[test_case("x509Certificates" ; "x509_certificates")]
fn validate_rejects_a_second_primary_on(attr: &str) {
    let urn = crate::schema_urns::USER;
    let two = r#"[{"value":"a","primary":true},{"value":"b","primary":true}]"#;
    let raw = format!(r#"{{"schemas":["{urn}"],"userName":"u","{attr}":{two}}}"#);
    let user: User<String> = serde_json::from_str(&raw).unwrap();
    let err = user.validate().expect_err("two primaries");
    assert_eq!(err.path(), attr);
    let one = raw.replace(r#"{"value":"b","primary":true}"#, r#"{"value":"b"}"#);
    let user: User<String> = serde_json::from_str(&one).unwrap();
    assert!(user.validate().is_ok());
}

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
    assert_eq!(user.emails.len(), 2);
    assert_eq!(
        user.emails[0].value,
        Some("bjensen@example.com".to_string())
    );
    assert_eq!(user.emails[0].r#type, Some("work".to_string()));
    assert_eq!(user.addresses.len(), 2);
    assert_eq!(user.addresses[0].r#type.as_ref().unwrap(), "work");
    assert_eq!(user.phone_numbers.len(), 2);
    assert_eq!(
        user.phone_numbers[0].value,
        Some("555-555-5555".to_string())
    );
    assert_eq!(user.ims.len(), 1);
    assert_eq!(user.ims[0].value, Some("someaimhandle".to_string()));
    assert_eq!(user.groups.len(), 3);
    assert_eq!(
        user.groups[0].value,
        Some("e9e30dba-f08f-4109-8486-d5c6a331660a".to_string())
    );
    assert_eq!(user.x509_certificates.len(), 1);
    assert_eq!(user.x509_certificates[0].value, Some("MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo=".to_string()), "x509_certificates[0].value did not match expected value");
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
//
// Asserts fidelity, not just parseability — it previously called only
// `.expect(...)`, so the fixture proved nothing about whether any modelled
// attribute survived, including the `addresses[].primary` this release
// added.
//
// Deliberately not `assert_user_round_trips`. This fixture carries
// `roles[0].primary` as the JSON *string* `"True"`, and the lenient
// deserializer normalises it to the boolean `true`, so re-serialising
// cannot reproduce the original value — by design, and the reason that
// deserializer exists. That normalisation is itself the most useful thing
// this fixture pins: it is the only real provider payload in the suite
// that exercises a stringified boolean end to end.
#[test]
fn deserialize_entra_user() {
    let raw = include_str!("../../test_data/provider_samples/entra_user_creation_test.json");
    assert!(
        raw.contains(r#""primary": "True""#),
        "fixture must still carry the stringified boolean this test is about"
    );

    let user: User = serde_json::from_str(raw).expect("Entra payload must deserialize");
    assert_eq!(user.user_name, "isaias@bode.ca");
    assert_eq!(user.active, Some(true));

    // The stringified boolean, normalised.
    assert_eq!(user.roles.len(), 1);
    assert_eq!(user.roles[0].primary, Some(true));

    // Every modelled multi-valued attribute the fixture populates.
    assert_eq!(user.addresses.len(), 1);
    assert_eq!(user.addresses[0].r#type.as_deref(), Some("work"));
    assert_eq!(user.addresses[0].country.as_deref(), Some("Gibraltar"));
    assert_eq!(user.emails.len(), 1);
    assert_eq!(user.phone_numbers.len(), 3);

    // The enterprise extension, from its URN key.
    let ext = user
        .enterprise_user
        .as_ref()
        .expect("the extension must deserialize from its URN key");
    assert_eq!(ext.employee_number.as_deref(), Some("LLQMUPKSPYGA"));
    assert!(ext.manager.is_some());

    // Everything except that one normalisation does survive, which is the
    // round-trip claim narrowed to what is actually true here.
    let reserialized: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&user).expect("serialize")).unwrap();
    let mut original: serde_json::Value = serde_json::from_str(raw).unwrap();
    original["roles"][0]["primary"] = serde_json::json!(true);
    assert_eq!(
        crate::utils::serde::drop_unassigned(reserialized),
        crate::utils::serde::drop_unassigned(original),
        "apart from the stringified-boolean normalisation, no assigned attribute may be lost"
    );
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
    use crate::models::others::{ListResponse, Resource};

    let list: ListResponse<Resource<String>> = serde_json::from_str(include_str!(
        "../../test_data/provider_samples/github_enterprise_user_list_test.json"
    ))
    .expect("GitHub Enterprise user list should deserialize");

    assert_eq!(list.total_results, 2);
    assert_eq!(list.resources.len(), 2);
}

/// RFC 7643 §2.4: "The primary attribute value true MUST appear no more
/// than once." Checked per multi-valued attribute, reported by wire name.
#[test]
fn validate_rejects_a_second_primary() {
    let two = User::<String> {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        emails: vec![
            Email {
                value: Some("a@example.com".to_string()),
                primary: Some(true),
                ..Default::default()
            },
            Email {
                value: Some("b@example.com".to_string()),
                primary: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let err = two.validate().expect_err("two primaries");
    assert_eq!(err.path(), "emails");

    let one = User::<String> {
        emails: vec![
            Email {
                value: Some("a@example.com".to_string()),
                primary: Some(true),
                ..Default::default()
            },
            Email {
                value: Some("b@example.com".to_string()),
                primary: Some(false),
                ..Default::default()
            },
        ],
        ..two
    };
    assert!(
        one.validate().is_ok(),
        "one primary plus an explicit false is fine"
    );
}

/// RFC 7643 §3: `schemas` "MUST only contain values defined as schema and
/// schemaExtensions for the resource's defined resourceType". A User must
/// carry the User URN; the enterprise extension URN alongside it is fine.
#[test]
fn validate_requires_the_user_urn_in_schemas() {
    let wrong = User::<String> {
        schemas: vec![crate::schema_urns::GROUP.to_string()],
        user_name: "bjensen".to_string(),
        ..Default::default()
    };
    assert_eq!(
        wrong.validate().expect_err("Group URN on a User").path(),
        "schemas"
    );

    let with_extension = User::<String> {
        schemas: vec![
            crate::schema_urns::USER.to_string(),
            crate::schema_urns::ENTERPRISE_USER.to_string(),
        ],
        user_name: "bjensen".to_string(),
        ..Default::default()
    };
    assert!(with_extension.validate().is_ok());
}

/// `active` tolerates the stringified booleans some providers send, the
/// same way every `primary` does.
#[test]
fn active_accepts_a_stringified_boolean() {
    let urn = crate::schema_urns::USER;
    for (raw, want) in [
        (
            format!(r#"{{"schemas":["{urn}"],"userName":"a","active":true}}"#),
            Some(true),
        ),
        (
            format!(r#"{{"schemas":["{urn}"],"userName":"a","active":"True"}}"#),
            Some(true),
        ),
        (
            format!(r#"{{"schemas":["{urn}"],"userName":"a","active":"false"}}"#),
            Some(false),
        ),
        (format!(r#"{{"schemas":["{urn}"],"userName":"a"}}"#), None),
    ] {
        let u: User = serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{raw}: {e}"));
        assert_eq!(u.active, want, "{raw}");
    }
}

/// RFC 7643 §2.4's common sub-attributes on `Address`. `primary` is
/// covered by the JumpCloud fixture, but `value` and `display` appear in
/// no fixture, so deleting either left the suite green while
/// re-introducing the silent round-trip drop this release fixed. Asserted
/// here rather than by editing a fixture, since the fixtures are verbatim
/// RFC payloads and sanitized provider responses.
#[test]
fn address_common_sub_attributes_round_trip() {
    let raw = r#"{
            "formatted": "100 Universal City Plaza\nHollywood, CA 91608 USA",
            "streetAddress": "100 Universal City Plaza",
            "locality": "Hollywood",
            "region": "CA",
            "postalCode": "91608",
            "country": "USA",
            "type": "work",
            "value": "100 Universal City Plaza, Hollywood, CA 91608 USA",
            "display": "Work address",
            "primary": true
        }"#;

    let address: Address = serde_json::from_str(raw).expect("address must deserialize");
    assert_eq!(
        address.value.as_deref(),
        Some("100 Universal City Plaza, Hollywood, CA 91608 USA")
    );
    assert_eq!(address.display.as_deref(), Some("Work address"));
    assert_eq!(address.primary, Some(true));

    let back: serde_json::Value = serde_json::to_value(&address).unwrap();
    let original: serde_json::Value = serde_json::from_str(raw).unwrap();
    assert_eq!(back, original, "no Address sub-attribute may be dropped");
}

/// §2.4 defines `primary` as a boolean, but providers stringify it. The
/// lenient deserializer covers every carrier; this pins it through the
/// real model types rather than through a synthetic struct.
#[test]
fn primary_accepts_a_stringified_boolean_on_every_carrier() {
    macro_rules! assert_lenient {
        ($ty:ty, $label:literal) => {{
            for raw in [
                r#"{"primary": true}"#,
                r#"{"primary": "true"}"#,
                r#"{"primary": "True"}"#,
            ] {
                let v: $ty =
                    serde_json::from_str(raw).unwrap_or_else(|e| panic!("{}: {raw}: {e}", $label));
                assert_eq!(v.primary, Some(true), "{}: {raw}", $label);
            }
            let absent: $ty = serde_json::from_str("{}").unwrap();
            assert_eq!(absent.primary, None, "{}: absent", $label);
        }};
    }
    assert_lenient!(Email, "Email");
    assert_lenient!(Address, "Address");
    assert_lenient!(PhoneNumber, "PhoneNumber");
    assert_lenient!(Im, "Im");
    assert_lenient!(Photo, "Photo");
    assert_lenient!(Entitlement, "Entitlement");
    assert_lenient!(Role, "Role");
    assert_lenient!(X509Certificate, "X509Certificate");
}

/// RFC 7643 §2.5 makes an absent attribute, an explicit `null`, and an
/// empty array equivalent in state. Multi-valued attributes are modelled
/// as `Vec<T>` rather than `Option<Vec<T>>` precisely so those three wire
/// forms cannot produce three distinct in-memory states.
#[test]
fn multi_valued_attributes_treat_absent_null_and_empty_alike() {
    let urn = crate::schema_urns::USER;
    let absent = format!(r#"{{"schemas":["{urn}"],"userName":"bjensen"}}"#);
    let null =
        format!(r#"{{"schemas":["{urn}"],"userName":"bjensen","roles":null,"emails":null}}"#);
    let empty = format!(r#"{{"schemas":["{urn}"],"userName":"bjensen","roles":[],"emails":[]}}"#);

    for (label, raw) in [("absent", &absent), ("null", &null), ("empty", &empty)] {
        let user: User = serde_json::from_str(raw)
            .unwrap_or_else(|e| panic!("{label} form must deserialize: {e}"));
        assert!(user.roles.is_empty(), "{label}: roles");
        assert!(user.emails.is_empty(), "{label}: emails");
    }
}

/// An unassigned multi-valued attribute serializes as `[]`, not as `null`
/// and not by omission.
///
/// RFC 7643 §2.5 permits omitting it ("MAY be omitted for compactness"),
/// but RFC 7644 §3.5.1 gives `[]` operational meaning that omission does
/// not have: "Clients that want to override a server's defaults MAY
/// specify `null` for a single-valued attribute, or an empty array `[]`
/// for a multi-valued attribute, to clear all values", while an omitted
/// attribute is merely "not asserted by the client" and the server MAY
/// clear it *or* substitute a default. Since these models serve as request
/// bodies as well as representations, emitting `[]` is what keeps a
/// conformant clear-all expressible.
#[test]
fn unassigned_multi_valued_attributes_serialize_as_empty_arrays() {
    let user = User::<String> {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_value(&user).unwrap();
    let obj = json.as_object().unwrap();
    for attr in [
        "emails",
        "addresses",
        "phoneNumbers",
        "ims",
        "photos",
        "groups",
        "entitlements",
        "roles",
        "x509Certificates",
    ] {
        assert_eq!(
            obj.get(attr),
            Some(&serde_json::json!([])),
            "{attr} must serialize as [] so a clear-all PUT is expressible"
        );
    }
    assert!(
        !json.to_string().contains("null"),
        "no attribute may be null: {json}"
    );
}

/// The RFC 7644 §3.5.1 clear-all idiom, end to end: a client that empties a
/// multi-valued attribute produces a body carrying `[]` for it, which is
/// the form the RFC says clears existing values on the server.
#[test]
fn a_cleared_attribute_reaches_the_wire_as_an_empty_array() {
    let user = User::<String> {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        roles: Vec::new(),
        emails: vec![Email {
            value: Some("bjensen@example.com".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    let json = serde_json::to_value(&user).unwrap();
    assert_eq!(
        json["roles"],
        serde_json::json!([]),
        "cleared roles must be []"
    );
    assert_eq!(
        json["emails"].as_array().unwrap().len(),
        1,
        "assigned values survive"
    );
}

/// Verbatim `User` payloads from RFC 7644. Each is parsed into `User`
/// and then round-tripped (serialize, re-parse as `serde_json::Value`)
/// to confirm no modelled field is dropped on the way back out.
mod rfc7644_samples {
    use super::*;
    use pretty_assertions::assert_eq;

    use crate::utils::serde::drop_unassigned;

    fn assert_user_round_trips(raw: &str) {
        let user: User =
            serde_json::from_str(raw).expect("RFC 7644 sample must deserialize into User");
        let reserialized: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&user).expect("serialize User")).unwrap();
        let original: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert_eq!(
            drop_unassigned(reserialized),
            drop_unassigned(original),
            "round-tripped User must carry every assigned attribute of the original RFC payload"
        );
    }

    /// JumpCloud sends `"emails": null` on a minimal PUT. RFC 7643 §2.5
    /// makes that equivalent to unassigned, and this is the payload that
    /// makes `deserialize_null_as_empty_vec` load-bearing rather than
    /// theoretical: with `#[serde(default)]` alone, modelling `emails` as
    /// `Vec<Email>` would reject this real provider response outright.
    #[test]
    fn jumpcloud_put_user_minimal_accepts_explicit_null_emails() {
        let raw = include_str!("../../test_data/provider_samples/jumpcloud_put_user_minimal.json");
        assert!(
            raw.contains(r#""emails": null"#),
            "fixture must carry the null"
        );

        let user: User = serde_json::from_str(raw).expect("JumpCloud minimal PUT must parse");
        assert!(user.emails.is_empty());
        assert_eq!(user.user_name, "testuser@example.io");
        assert_eq!(user.active, Some(true));

        // `[]` on the way back out, never `null` and never omitted:
        // §2.5 makes the state equivalent, and `[]` keeps a §3.5.1
        // clear-all expressible.
        let back = serde_json::to_value(&user).unwrap();
        assert_eq!(back["emails"], serde_json::json!([]));
    }

    /// The full JumpCloud PUT carries empty arrays for `phoneNumbers` and
    /// `addresses`, the enterprise extension under its URN key, and
    /// `photos` with a bare `value`. All three shapes have to survive.
    #[test]
    fn jumpcloud_put_user_full_round_trips() {
        let raw = include_str!("../../test_data/provider_samples/jumpcloud_put_user_full.json");
        assert_user_round_trips(raw);

        let user: User = serde_json::from_str(raw).unwrap();
        assert!(user.phone_numbers.is_empty(), "explicit [] is unassigned");
        assert!(user.addresses.is_empty(), "explicit [] is unassigned");
        assert_eq!(user.photos.len(), 1);
        let enterprise = user
            .enterprise_user
            .as_ref()
            .expect("the enterprise extension must deserialize from its URN key");
        assert!(
            enterprise.validate().is_ok(),
            "RFC 7643 §4.3 defines no required attribute on the extension"
        );
    }

    /// The create request declares both the core and enterprise URNs in
    /// `schemas` while carrying no extension body, which is legal and must
    /// not be mistaken for a malformed payload.
    #[test]
    fn jumpcloud_create_user_round_trips() {
        let raw = include_str!("../../test_data/provider_samples/jumpcloud_create_user.json");
        assert_user_round_trips(raw);

        let user: User = serde_json::from_str(raw).unwrap();
        assert!(
            user.schemas
                .contains(&crate::schema_urns::ENTERPRISE_USER.to_string())
        );
        assert_eq!(user.emails.len(), 1);
        assert_eq!(user.addresses.len(), 1);
        assert!(user.validate().is_ok());
    }

    /// RFC 7644 §3.3 — unnumbered example, "a client sends a POST request
    /// containing a `User` to the `/Users` endpoint". User creation request
    /// (POST /Users body).
    #[test]
    fn rfc7644_s3_3_user_create_request() {
        let raw = include_str!("../../test_data/rfc7644/s3.3_user_create_request.json");
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
        let raw = include_str!("../../test_data/rfc7644/s3.3_user_create_response.json");
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
        let raw = include_str!("../../test_data/rfc7644/s3.4.1_user_retrieval_response.json");
        assert_user_round_trips(raw);
        let user: User = serde_json::from_str(raw).unwrap();
        assert_eq!(user.emails[0].value.as_deref(), Some("bjensen@example.com"));
        assert_eq!(user.phone_numbers[0].r#type.as_deref(), Some("work"));
    }

    /// RFC 7644 §3.5.1 — unnumbered example following "a successful PUT
    /// operation returns a 200 OK response code and the entire resource
    /// within the response body ... For example". PUT /Users/{id} request
    /// replacing a user, including an empty `roles` array.
    #[test]
    fn rfc7644_s3_5_1_user_put_request() {
        let raw = include_str!("../../test_data/rfc7644/s3.5.1_user_put_request.json");
        assert_user_round_trips(raw);
        let user: User = serde_json::from_str(raw).unwrap();
        assert!(user.roles.is_empty());
        assert_eq!(
            user.name.as_ref().unwrap().middle_name.as_deref(),
            Some("Jane")
        );
        assert_eq!(user.emails.len(), 2);
    }

    /// RFC 7644 §3.5.1 — unnumbered example, "The service responds with the
    /// entire updated User". PUT /Users/{id} response (drops `roles`, adds
    /// `meta`).
    #[test]
    fn rfc7644_s3_5_1_user_put_response() {
        let raw = include_str!("../../test_data/rfc7644/s3.5.1_user_put_response.json");
        assert_user_round_trips(raw);
        let user: User = serde_json::from_str(raw).unwrap();
        assert!(user.roles.is_empty());
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
