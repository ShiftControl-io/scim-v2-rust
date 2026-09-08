use super::*;
use crate::models::user::User;
use crate::schema_urns;

/// The headline of the 1.0 `ListResponse` change: a homogeneous page
/// deserializes into the concrete resource, so a caller that already knows
/// `GET /Users` returns users does not match through a four-way enum to
/// prove it.
#[test]
fn typed_list_response_deserializes_without_matching_an_enum() {
    let schema = schema_urns::LIST_RESPONSE;
    let user = schema_urns::USER;
    let body = format!(
        r#"{{"schemas":["{schema}"],"totalResults":2,"startIndex":1,"itemsPerPage":2,
                 "Resources":[
                   {{"schemas":["{user}"],"userName":"bjensen"}},
                   {{"schemas":["{user}"],"userName":"jsmith"}}]}}"#
    );

    let list: ListResponse<User<String>> =
        serde_json::from_str(&body).expect("typed ListResponse must deserialize");
    assert!(list.validate().is_ok());
    let names: Vec<&str> = list
        .resources
        .iter()
        .map(|u| u.user_name.as_str())
        .collect();
    assert_eq!(names, ["bjensen", "jsmith"]);
}

/// L-2: the typed path chooses `R` at the call site, so before this the
/// payload's declared `schemas` was never compared against the type it was
/// parsed as. A Group payload carrying a `userName` deserialized cleanly
/// into `ListResponse<User<String>>` and both validators returned `Ok`.
#[test]
fn typed_list_response_validate_rejects_a_schema_mismatch() {
    let schema = schema_urns::LIST_RESPONSE;
    let group = schema_urns::GROUP;
    let body = format!(
        r#"{{"schemas":["{schema}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
                 "Resources":[{{"schemas":["{group}"],"userName":"admin","displayName":"Tour Guides"}}]}}"#
    );

    // It still deserializes — `R` decides the shape, and every attribute
    // `User` requires is present.
    let list: ListResponse<User<String>> = serde_json::from_str(&body).expect("shape matches User");
    assert_eq!(list.resources[0].user_name, "admin");

    let err = list
        .validate()
        .expect_err("a Group URN in a ListResponse<User> is a mismatch");
    assert_eq!(err.path(), "Resources[0].schemas");
    assert_eq!(err.scim_type_str(), "invalidValue");
}

/// `ListResponse::validate` judges the envelope and, when a resource
/// declares `schemas`, the discriminator. An absent `schemas` is the
/// resource's own `Validate`'s business — `User::validate` rejects this one
/// — so it is not reported as a `ListResponse` failure.
#[test]
fn typed_list_response_validate_leaves_an_absent_discriminator_to_the_resource() {
    let list: ListResponse<User<String>> = ListResponse {
        schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
        total_results: 1,
        start_index: Some(1),
        items_per_page: Some(1),
        resources: vec![User::<String> {
            schemas: Vec::new(),
            user_name: "bjensen".to_string(),
            ..Default::default()
        }],
    };
    assert!(
        list.validate().is_ok(),
        "the envelope check does not judge an omitted `schemas`"
    );
    assert_eq!(list.resources[0].validate().unwrap_err().path(), "schemas");
}

/// The `schemas` array may carry URNs this crate does not model — a
/// custom extension, say — alongside one it does. Dispatch follows the
/// recognised URN rather than failing on the unrecognised one.
#[test]
fn resource_dispatch_ignores_an_unknown_urn_beside_a_known_one() {
    let body = format!(
        r#"{{"schemas":["urn:example:custom:Thing","{}"],"userName":"bjensen"}}"#,
        schema_urns::USER
    );
    let r: Resource<String> = serde_json::from_str(&body).expect("must dispatch on the User URN");
    assert!(matches!(r, Resource::User(_)));
}

/// The heterogeneous form still works, which RFC 7644 §3.4.3 needs for a
/// query against the root `/.search` endpoint.
#[test]
fn heterogeneous_list_response_still_dispatches_per_resource() {
    let schema = schema_urns::LIST_RESPONSE;
    let user = schema_urns::USER;
    let group = schema_urns::GROUP;
    let body = format!(
        r#"{{"schemas":["{schema}"],"totalResults":2,"startIndex":1,"itemsPerPage":2,
                 "Resources":[
                   {{"schemas":["{user}"],"userName":"bjensen"}},
                   {{"schemas":["{group}"],"displayName":"Tour Guides"}}]}}"#
    );

    let list: ListResponse<Resource<String>> =
        serde_json::from_str(&body).expect("heterogeneous ListResponse must deserialize");
    let urns: Vec<&str> = list
        .resources
        .iter()
        .map(ScimResource::schema_urn)
        .collect();
    assert_eq!(urns, [schema_urns::USER, schema_urns::GROUP]);
}

/// A typed `ListResponse` rejects a payload whose resources are a
/// different type, rather than silently accepting it. The `schemas` URN in
/// the body does not select the type here — `R` does — so the mismatch has
/// to surface as a deserialization failure.
#[test]
fn typed_list_response_rejects_a_mismatched_resource() {
    let schema = schema_urns::LIST_RESPONSE;
    let group = schema_urns::GROUP;
    let body = format!(
        r#"{{"schemas":["{schema}"],"totalResults":1,"startIndex":1,"itemsPerPage":1,
                 "Resources":[{{"schemas":["{group}"],"displayName":"Tour Guides"}}]}}"#
    );

    // `Group` has no `userName`, which `User` requires.
    let parsed: Result<ListResponse<User<String>>, _> = serde_json::from_str(&body);
    assert!(
        parsed.is_err(),
        "a Group payload must not deserialize as ListResponse<User>"
    );
}
