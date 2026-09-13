/// `Group::validate` and every branch of its `validate_context`.
#[test]
fn validate_and_validate_context() {
    let base = Group::<String> {
        schemas: vec![crate::schema_urns::GROUP.to_string()],
        id: None,
        external_id: None,
        display_name: "Tour Guides".to_string(),
        members: Multi::absent(),
        meta: None,
    };
    assert!(base.validate().is_ok());
    assert_eq!(
        Group {
            display_name: String::new(),
            ..base.clone()
        }
        .validate()
        .expect_err("displayName")
        .path(),
        "displayName"
    );
    assert_eq!(
        Group {
            schemas: vec![crate::schema_urns::USER.to_string()],
            ..base.clone()
        }
        .validate()
        .expect_err("wrong URN")
        .path(),
        "schemas"
    );

    let with_id = Group {
        id: Some("g1".to_string()),
        ..base.clone()
    };
    assert_eq!(
        with_id
            .validate_as(Context::CreateRequest)
            .expect_err("id on create")
            .path(),
        "id"
    );
    assert!(base.validate_as(Context::CreateRequest).is_ok());
    assert!(with_id.validate_as(Context::ReplaceRequest).is_ok());
    assert_eq!(
        base.validate_as(Context::Response)
            .expect_err("no id")
            .path(),
        "id"
    );
    assert!(with_id.validate_as(Context::Response).is_ok());
}

/// The hand-written `PartialEq` is case-insensitive and `Hash`
/// agrees with it, exercised through a real container.
#[test]
fn member_type_equality_and_hash_are_case_insensitive() {
    use std::collections::HashSet;
    assert_eq!(
        MemberType::Other("serviceaccount".into()),
        MemberType::Other("ServiceAccount".into())
    );
    assert_eq!(MemberType::from("USER".to_string()), MemberType::User);
    assert_eq!(
        MemberType::Other("User".into()),
        MemberType::User,
        "wire-identical values are equal"
    );
    let mut set = HashSet::new();
    set.insert(MemberType::Other("ServiceAccount".into()));
    assert!(set.contains(&MemberType::Other("serviceaccount".into())));
    set.insert(MemberType::User);
    assert!(set.contains(&MemberType::Other("USER".into())));
}

/// RFC 7643 §7 makes `canonicalValues` *suggestions*, so a provider may
/// send a `members.type` outside {User, Group}. Before 1.0 that failed the
/// whole Group payload; it now lands in `Other` and round-trips.
#[test]
fn unknown_member_type_round_trips_instead_of_failing() {
    let urn = crate::schema_urns::GROUP;
    let raw = format!(
        r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":[
                 {{"value":"a","type":"User"}},
                 {{"value":"b","type":"GROUP"}},
                 {{"value":"c","type":"ServiceAccount"}}]}}"#
    );
    let group: Group = serde_json::from_str(&raw).expect("unknown type must not fail the payload");
    assert_eq!(group.members[0].r#type, Some(MemberType::User));
    // §2.2: caseExact defaults to false, so "GROUP" is the Group label.
    assert_eq!(group.members[1].r#type, Some(MemberType::Group));
    assert_eq!(
        group.members[2].r#type,
        Some(MemberType::Other("ServiceAccount".to_string()))
    );

    let back = serde_json::to_value(&group).unwrap();
    let types: Vec<&str> = back["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["type"].as_str().unwrap())
        .collect();
    assert_eq!(types, ["User", "Group", "ServiceAccount"]);
}
/// RFC 7643 §2.5 equivalence for `Group.members`: absent, `null` and `[]`
/// all mean unassigned, and unassigned is omitted on the way out.
/// Guards the `deserialize_null_as_empty_vec` wiring, which
/// `#[serde(default)]` alone does not provide.
/// All three wire forms read alike, because RFC 7643 §2.5 makes them one
/// state in a resource. They do not write alike: RFC 7644 §3.5.1 makes an
/// absent member "not asserted by the client", so only the forms the client
/// actually sent go back out.
#[test]
fn members_read_alike_and_write_back_what_arrived() {
    let urn = crate::schema_urns::GROUP;
    for (label, raw, expected) in [
        (
            "absent",
            format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides"}}"#),
            None,
        ),
        (
            "null",
            format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":null}}"#),
            Some(serde_json::json!([])),
        ),
        (
            "empty",
            format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":[]}}"#),
            Some(serde_json::json!([])),
        ),
    ] {
        let group: Group = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("{label} form must deserialize: {e}"));
        assert!(group.members.is_empty(), "{label}: reads as empty");
        let out = serde_json::to_value(&group).unwrap();
        assert_eq!(
            out.get("members"),
            expected.as_ref(),
            "{label}: writes back"
        );
    }
}

use pretty_assertions::assert_eq;

use super::*;

#[test]
fn group_deserialization_succeeds_for_valid_full_json() {
    let json_data = r#"   {
             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
             "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
             "displayName": "Tour Guides",
             "members": [
               {
                 "value": "2819c223-7f76-453a-919d-413861904646",
                 "$ref":
           "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646",
                 "display": "Babs Jensen"
               },
               {
                 "value": "902c246b-6245-4190-8e05-00816be7344a",
                 "$ref":
           "https://example.com/v2/Users/902c246b-6245-4190-8e05-00816be7344a",
                 "display": "Mandy Pepperidge"
               }
             ],
             "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location":
           "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
             }
           }"#;

    let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

    if let Err(e) = &group {
        eprintln!("Deserialization failed: {:?}", e);
    }
    assert!(group.is_ok());
    let group = group.unwrap();
    assert_eq!(
        group.schemas,
        vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
    );
    assert_eq!(
        group.id,
        Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
    );
    assert_eq!(group.display_name, "Tour Guides");

    // Check members
    assert_eq!(group.members.len(), 2);
    assert_eq!(
        group.members[0].value,
        Some("2819c223-7f76-453a-919d-413861904646".to_string())
    );
    assert_eq!(group.members[0].display, Some("Babs Jensen".to_string()));
    assert_eq!(
        group.members[1].value,
        Some("902c246b-6245-4190-8e05-00816be7344a".to_string())
    );
    assert_eq!(
        group.members[1].display,
        Some("Mandy Pepperidge".to_string())
    );

    // Check meta
    let meta = group.meta.unwrap();
    assert_eq!(meta.resource_type, Some("Group".to_string()));
    assert_eq!(
        meta.created.as_ref().map(crate::ScimDateTime::as_str),
        Some("2010-01-23T04:56:22Z")
    );
    assert_eq!(
        meta.last_modified.as_ref().map(crate::ScimDateTime::as_str),
        Some("2011-05-13T04:42:34Z")
    );
    assert_eq!(meta.version, Some("W/\"3694e05e9dff592\"".to_string()));
    assert_eq!(
        meta.location,
        Some("https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a".to_string())
    );
}

#[test]
fn group_deserialization_succeeds_for_valid_json() {
    let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides",
            "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }
        }"#;

    let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

    if let Err(e) = &group {
        eprintln!("Deserialization failed: {:?}", e);
    }
    assert!(group.is_ok());
    let group = group.unwrap();
    assert_eq!(
        group.schemas,
        vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
    );
    assert_eq!(
        group.id,
        Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
    );
    assert_eq!(group.display_name, "Tour Guides");
}

#[test]
fn group_deserialization_fails_for_invalid_json() {
    let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": 12345
        },
        "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }"#;

    let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

    assert!(group.is_err());
}

#[test]
fn group_deserialization_handles_missing_optional_fields() {
    let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides"
        }"#;

    let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

    assert!(group.is_ok());
    let group = group.unwrap();
    assert_eq!(
        group.schemas,
        vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
    );
    assert_eq!(
        group.id,
        Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
    );
    assert_eq!(group.display_name, "Tour Guides");
    assert!(group.members.is_empty());
    assert!(group.meta.is_none());
}

/// RFC 7643 §3.1's "non-empty id" on a Group response.
#[test]
fn response_rejects_an_empty_id() {
    let group = |id: Option<&str>| -> Group<String> {
        let id = id.map_or(String::new(), |i| format!(r#","id":"{i}""#));
        serde_json::from_str(&format!(
            r#"{{"schemas":["{}"],"displayName":"Tour Guides"{id}}}"#,
            crate::schema_urns::GROUP
        ))
        .unwrap()
    };
    assert_eq!(
        group(Some(""))
            .validate_as(Context::Response)
            .unwrap_err()
            .path(),
        "id"
    );
    assert_eq!(
        group(None)
            .validate_as(Context::Response)
            .unwrap_err()
            .path(),
        "id"
    );
    assert_eq!(
        group(Some("e9e30dba")).validate_as(Context::Response),
        Ok(())
    );
}

/// Equality and hashing distinguish different members while folding case;
/// the label is what goes on the wire.
#[test]
fn member_type_distinguishes_different_values_and_hashes_them_apart() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    fn h(m: &MemberType) -> u64 {
        let mut hasher = DefaultHasher::new();
        m.hash(&mut hasher);
        hasher.finish()
    }
    assert_ne!(MemberType::User, MemberType::Group);
    assert_ne!(
        MemberType::Other("a".to_string()),
        MemberType::Other("b".to_string())
    );
    assert_ne!(
        MemberType::Other("ServiceAccount".to_string()),
        MemberType::User
    );
    assert_ne!(h(&MemberType::User), h(&MemberType::Group));
    assert_ne!(
        h(&MemberType::Other("a".to_string())),
        h(&MemberType::Other("b".to_string()))
    );
    assert_eq!(
        h(&MemberType::Other("user".to_string())),
        h(&MemberType::User)
    );
    assert_eq!(MemberType::User.label(), "User");
    assert_eq!(MemberType::Group.label(), "Group");
    assert_eq!(
        MemberType::Other("ServiceAccount".to_string()).label(),
        "ServiceAccount"
    );
    assert_eq!(
        serde_json::to_string(&MemberType::Group).unwrap(),
        "\"Group\""
    );
    assert_eq!(
        serde_json::to_string(&MemberType::Other("ServiceAccount".to_string())).unwrap(),
        "\"ServiceAccount\""
    );
}
