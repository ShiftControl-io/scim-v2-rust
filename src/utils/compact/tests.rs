use super::*;

/// Devin review on #49, ANALYSIS-1: the one protocol array a conformant
/// payload can legitimately hold empty is `Resources` on an empty page,
/// and RFC 7644 §3.4.2 permits omitting it, so the compact form still
/// deserializes and validates.
#[cfg(feature = "models")]
#[test]
fn compact_empty_page_omits_resources_and_still_validates() {
    use crate::Validate;
    use crate::models::{others::ListResponse, user::User};

    let page: ListResponse<User<String>> = ListResponse {
        schemas: vec![crate::schema_urns::LIST_RESPONSE.to_string()],
        total_results: 0,
        items_per_page: None,
        start_index: None,
        resources: vec![],
    };
    let json = serde_json::to_value(Compact(&page)).unwrap();
    assert!(json.get("Resources").is_none(), "{json}");
    assert_eq!(json["schemas"].as_array().map(Vec::len), Some(1));
    let back: ListResponse<User<String>> = serde_json::from_value(json).unwrap();
    assert_eq!(back.validate(), Ok(()));
    assert_eq!(back, page);
}

#[cfg(feature = "models")]
#[test]
fn compact_never_touches_non_empty_arrays() {
    use crate::models::user::{Email, User};

    let user = User::<String> {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        emails: vec![Email {
            value: Some("bjensen@example.com".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    let json = serde_json::to_value(Compact(&user)).unwrap();
    assert_eq!(json["schemas"].as_array().map(Vec::len), Some(1));
    assert_eq!(json["emails"].as_array().map(Vec::len), Some(1));
    assert!(json.get("roles").is_none());
}

#[test]
fn strips_empty_arrays_and_nulls_recursively() {
    let mut v = serde_json::json!({
        "a": [], "b": null, "c": [1], "d": {"e": [], "f": "x", "g": [{"h": []}]}
    });
    strip_unassigned(&mut v);
    assert_eq!(v, serde_json::json!({"c": [1], "d": {"f": "x", "g": [{}]}}));
}

/// R3-M3: every multi-valued attribute the plain form emits as `[]` is
/// omitted — all nine, not only `emails` — every other member is untouched,
/// and the compact form reads back equal to the original.
#[cfg(feature = "models")]
#[test]
fn compact_omits_every_empty_multi_valued_attribute_and_reads_back_equal() {
    use crate::models::user::User;

    let user = User::<String> {
        schemas: vec![crate::schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        display_name: Some("Barbara".to_string()),
        ..Default::default()
    };
    let plain = serde_json::to_value(&user).unwrap();
    let compact = serde_json::to_value(Compact(&user)).unwrap();
    let mut omitted = Vec::new();
    for (key, value) in plain.as_object().unwrap() {
        if value.as_array().is_some_and(Vec::is_empty) {
            assert!(compact.get(key).is_none(), "{key} should be omitted");
            omitted.push(key.as_str());
        } else {
            assert_eq!(compact.get(key), Some(value), "{key} must be untouched");
        }
    }
    omitted.sort_unstable();
    assert_eq!(
        omitted,
        [
            "addresses",
            "emails",
            "entitlements",
            "groups",
            "ims",
            "phoneNumbers",
            "photos",
            "roles",
            "x509Certificates",
        ]
    );
    let back: User<String> = serde_json::from_value(compact).unwrap();
    assert_eq!(back, user);
}
