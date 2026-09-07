//! Exercises the public API from outside the crate.
//!
//! Rust does not enforce field privacy within the defining crate, so a unit
//! test cannot tell whether a field is `pub`. Deleting `pub` from
//! `SearchRequest::excluded_attributes` — the shipped bug 1.0 fixes — left the
//! whole in-crate suite green while the field became unusable to every
//! consumer. These tests live in `tests/`, which is a separate crate, so they
//! fail on that mutation.

#![cfg(all(feature = "models", feature = "filter"))]

use scim_v2::filter::Filter;
use scim_v2::models::errors::ScimType;
use scim_v2::models::others::{ListResponse, Resource, SearchRequest};
use scim_v2::models::user::User;
use scim_v2::{Validate, schema_urns};

/// Every field of `SearchRequest` is reachable and reaches the wire.
/// `excluded_attributes` in particular was never `pub` before 1.0.
#[test]
fn search_request_fields_are_publicly_constructible() {
    let request = SearchRequest::<Filter> {
        schemas: vec![schema_urns::SEARCH_REQUEST.to_string()],
        attributes: vec!["userName".to_string()],
        excluded_attributes: vec!["password".to_string()],
        filter: Some(r#"userName eq "bjensen""#.parse().expect("valid filter")),
        sort_by: None,
        sort_order: None,
        start_index: Some(1),
        count: Some(10),
    };

    let json = serde_json::to_value(&request).expect("SearchRequest must serialize");
    assert_eq!(json["attributes"], serde_json::json!(["userName"]));
    assert_eq!(
        json["excludedAttributes"],
        serde_json::json!(["password"]),
        "excludedAttributes must reach the wire, which needs the field to be pub"
    );
    assert_eq!(json["startIndex"], 1);
    assert_eq!(json["count"], 10);
}

/// The typed `ListResponse` is usable from outside, including `Validate`,
/// which has to be imported as a trait.
#[test]
fn typed_list_response_is_usable_downstream() {
    let body = format!(
        r#"{{"schemas":["{}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
             "Resources":[{{"schemas":["{}"],"userName":"bjensen"}}]}}"#,
        schema_urns::LIST_RESPONSE,
        schema_urns::USER
    );

    let list: ListResponse<User<String>> =
        serde_json::from_str(&body).expect("typed ListResponse must deserialize downstream");
    list.validate().expect("a complete page is conformant");
    assert_eq!(list.resources[0].user_name, "bjensen");
}

/// The heterogeneous form and the sealed trait's method are both public.
#[test]
fn heterogeneous_list_response_is_usable_downstream() {
    use scim_v2::models::others::ScimResource;

    let body = format!(
        r#"{{"schemas":["{}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
             "Resources":[{{"schemas":["{}"],"userName":"bjensen"}}]}}"#,
        schema_urns::LIST_RESPONSE,
        schema_urns::USER
    );

    let list: ListResponse<Resource<String>> =
        serde_json::from_str(&body).expect("heterogeneous ListResponse must deserialize");
    assert_eq!(list.resources[0].schema_urn(), schema_urns::USER);
}

/// `ValidationError`'s accessors and the §3.12 body builder are public, and
/// the error names the wire path rather than the Rust field name.
#[test]
fn validation_error_surface_is_usable_downstream() {
    let user = User::<String> {
        schemas: vec![schema_urns::USER.to_string()],
        user_name: String::new(),
        ..Default::default()
    };

    let err = user.validate().expect_err("a blank userName is invalid");
    assert_eq!(err.path(), "userName");
    assert_eq!(err.scim_type_str(), "invalidValue");

    let body = err.to_http_error(400);
    assert_eq!(body.status, "400", "§3.12 renders status as a JSON string");
    assert_eq!(body.scim_type, Some(ScimType::InvalidValue));
}

/// RFC 7644 §3.5.1's clear-all idiom, from outside the crate: an emptied
/// multi-valued attribute has to reach the wire as `[]`, because omission
/// means "not asserted" and the server may then keep or default the values.
#[cfg(not(feature = "compact-multi-valued"))]
#[test]
fn a_cleared_multi_valued_attribute_reaches_the_wire_downstream() {
    let user = User::<String> {
        schemas: vec![schema_urns::USER.to_string()],
        user_name: "bjensen".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_value(&user).expect("User must serialize");
    assert_eq!(
        json["roles"],
        serde_json::json!([]),
        "an unassigned multi-valued attribute must serialize as [] per RFC 7644 §3.5.1"
    );
}
