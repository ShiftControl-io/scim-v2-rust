use super::*;

/// R2-C1: the table must be injective under case folding. Two entries that
/// fold to the same key collapse in the `HashMap`, and the loser was the
/// RFC's own spelling — `Resources` and `Operations` were being rewritten
/// to lowercase, silently emptying every `ListResponse` page and making a
/// PATCH body byte-identical to the RFC's example fail to parse.
#[test]
fn wire_name_table_is_injective_under_case_folding() {
    let mut seen = std::collections::HashMap::new();
    for n in WIRE_NAMES.iter().chain(URNS.iter()) {
        if let Some(prev) = seen.insert(n.to_ascii_lowercase(), *n) {
            panic!("{prev:?} and {n:?} fold to the same key");
        }
    }
    assert_eq!(table().len(), WIRE_NAMES.len() + URNS.len());
}

/// The two envelopes the collision hit, in every casing.
#[test]
fn protocol_envelope_members_keep_their_rfc_casing() {
    for spelling in ["Resources", "resources", "RESOURCES"] {
        let mut v = serde_json::json!({ spelling: [1] });
        canonicalize_keys(&mut v).unwrap();
        assert!(
            v.get("Resources").is_some(),
            "{spelling} must canonicalise to Resources"
        );
    }
    for spelling in ["Operations", "operations", "OPERATIONS"] {
        let mut v = serde_json::json!({ spelling: [1] });
        canonicalize_keys(&mut v).unwrap();
        assert!(
            v.get("Operations").is_some(),
            "{spelling} must canonicalise to Operations"
        );
    }
}

/// R2-H1: two spellings of one attribute in one object is an error, not a
/// silent last-write-wins whose winner depends on byte order.
#[test]
fn colliding_keys_are_rejected() {
    for raw in [
        r#"{"userName":"alice","username":"admin"}"#,
        r#"{"userName":"alice","USERNAME":"admin"}"#,
        r#"{"emails":[{"value":"a","VALUE":"b"}]}"#,
    ] {
        let mut v: Value = serde_json::from_str(raw).unwrap();
        let err = canonicalize_keys(&mut v).expect_err(raw);
        assert_ne!(err.first, err.second);
        assert!(
            from_str::<crate::models::user::User<String>>(raw).is_err(),
            "the deserializing entry points must surface the collision"
        );
    }
}

/// R2-M2: a subtree under an extension URN this crate does not model is
/// not RFC 7643 §2.1 territory and is left byte-identical. The extension
/// this crate does model is canonicalised, because that vocabulary is ours.
#[test]
fn foreign_extension_subtrees_are_untouched_but_known_ones_are_folded() {
    let mut v = serde_json::json!({
        "urn:example:2.0:Thing": {"Value": "x", "TITLE": "y", "MyOwn": "z"},
        "URN:IETF:PARAMS:SCIM:SCHEMAS:EXTENSION:ENTERPRISE:2.0:USER": {"EMPLOYEENUMBER": "7"}
    });
    canonicalize_keys(&mut v).unwrap();
    let foreign = &v["urn:example:2.0:Thing"];
    assert_eq!(foreign["Value"], "x");
    assert_eq!(foreign["TITLE"], "y");
    assert_eq!(foreign["MyOwn"], "z");
    let ours = &v["urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"];
    assert_eq!(ours["employeeNumber"], "7");
}

#[test]
fn canonicalizes_known_keys_and_leaves_unknown_ones() {
    let mut v = serde_json::json!({
        "SCHEMAS": ["URN:IETF:PARAMS:SCIM:SCHEMAS:CORE:2.0:USER"],
        "USERNAME": "bjensen",
        "Name": {"FAMILYNAME": "Jensen"},
        "emails": [{"VALUE": "a@example.com", "PRIMARY": true}],
        "x-vendor-extra": 1
    });
    canonicalize_keys(&mut v).unwrap();
    // Only *keys* are rewritten. The URNs inside the `schemas` array are
    // values, and values are never touched — RFC 8141 leaves the
    // case-sensitivity of a URN's namespace-specific string to the
    // namespace, and the crate compares URNs exactly, as the RFC prints them.
    assert_eq!(
        v["schemas"][0],
        "URN:IETF:PARAMS:SCIM:SCHEMAS:CORE:2.0:USER"
    );
    assert_eq!(v["userName"], "bjensen");
    assert_eq!(v["name"]["familyName"], "Jensen");
    assert_eq!(v["emails"][0]["value"], "a@example.com");
    assert_eq!(v["emails"][0]["primary"], true);
    assert_eq!(
        v["x-vendor-extra"], 1,
        "unknown keys must pass through untouched"
    );
}

/// The probe that motivated the module: a fully upper-cased User fails the
/// plain derive with `missing field userName` and parses here.
#[test]
fn an_upper_cased_user_parses() {
    let raw = r#"{"SCHEMAS":["urn:ietf:params:scim:schemas:core:2.0:User"],"USERNAME":"bjensen","DISPLAYNAME":"B"}"#;
    assert!(serde_json::from_str::<crate::models::user::User<String>>(raw).is_err());
    let u: crate::models::user::User<String> =
        from_str(raw).expect("§2.1: names are case-insensitive");
    assert_eq!(u.user_name, "bjensen");
    assert_eq!(u.display_name.as_deref(), Some("B"));
    let via_adapter: CaseInsensitive<crate::models::user::User<String>> =
        serde_json::from_str(raw).unwrap();
    assert_eq!(via_adapter.into_inner(), u);
}

/// The table must stay complete against the RFC schemas this crate ships.
#[cfg(feature = "schemas")]
#[test]
fn wire_names_cover_every_embedded_schema_attribute() {
    for raw in [
        crate::USER_SCHEMA,
        crate::GROUP_SCHEMA,
        crate::ENTERPRISE_USER_SCHEMA,
    ] {
        let schema: Value = serde_json::from_str(raw).unwrap();
        fn walk(attrs: &Value, missing: &mut Vec<String>) {
            for a in attrs.as_array().into_iter().flatten() {
                let name = a["name"].as_str().unwrap();
                if !WIRE_NAMES.contains(&name) {
                    missing.push(name.to_string());
                }
                walk(&a["subAttributes"], missing);
            }
        }
        let mut missing = Vec::new();
        walk(&schema["attributes"], &mut missing);
        assert!(missing.is_empty(), "WIRE_NAMES is missing: {missing:?}");
    }
}
