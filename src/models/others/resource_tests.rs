use super::*;
use crate::models::user::User;
use crate::schema_urns;
use crate::utils::validation::ValidationErrorKind;

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

/// `ListResponse::validate` reaches into
/// each resource, so a resource without `schemas` fails the page under its
/// index rather than slipping through an envelope-only check.
#[test]
fn typed_list_response_validate_reports_a_resource_missing_schemas() {
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
    let err = list
        .validate()
        .expect_err("a resource without schemas fails the page");
    assert_eq!(err.path(), "Resources[0].schemas");
    assert_eq!(err.kind(), &ValidationErrorKind::MissingRequiredAttribute);
}

/// The page carries each resource's own rules
/// and, through `validate_as`, the direction — a `Response` page needs an
/// `id` on every entry.
#[test]
fn list_response_validation_reaches_into_each_resource() {
    let user = |name: &str, id: Option<&str>| User::<String> {
        schemas: vec![schema_urns::USER.to_string()],
        user_name: name.to_string(),
        id: id.map(str::to_string),
        ..Default::default()
    };
    let page = |resources: Vec<User<String>>| ListResponse::<User<String>> {
        schemas: vec![schema_urns::LIST_RESPONSE.to_string()],
        total_results: resources.len() as i64,
        start_index: Some(1),
        items_per_page: Some(resources.len() as i64),
        resources,
    };
    let err = page(vec![user("a", Some("1")), user("", Some("2"))])
        .validate()
        .expect_err("second resource has no userName");
    assert_eq!(err.path(), "Resources[1].userName");

    let ok = page(vec![user("a", Some("1")), user("b", None)]);
    assert_eq!(ok.validate(), Ok(()), "direction-agnostic: id not required");
    let err = ok
        .validate_as(Context::Response)
        .expect_err("a response needs ids");
    assert_eq!(err.path(), "Resources[1].id");
    assert_eq!(
        page(vec![user("a", Some("1"))]).validate_as(Context::Response),
        Ok(())
    );
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

/// Every resource type reports the `schemas` it carries and the URN of its
/// type, which is what the typed page compares.
#[test]
fn scim_resource_reports_declared_schemas_and_its_own_urn() {
    let user = User::<String> {
        schemas: vec![schema_urns::USER.to_string(), "urn:x".to_string()],
        user_name: "u".to_string(),
        ..Default::default()
    };
    assert_eq!(user.declared_schemas(), [schema_urns::USER, "urn:x"]);
    assert_eq!(user.schema_urn(), schema_urns::USER);

    let group: Group<String> = serde_json::from_value(serde_json::json!({
        "schemas": [schema_urns::GROUP], "id": "1", "displayName": "g"
    }))
    .unwrap();
    assert_eq!(group.declared_schemas(), [schema_urns::GROUP]);
    assert_eq!(group.schema_urn(), schema_urns::GROUP);

    let schema: Schema = serde_json::from_value(serde_json::json!({
        "schemas": [schema_urns::SCHEMA], "id": "urn:x", "name": "X", "description": "d", "attributes": [], "meta": {}
    }))
    .unwrap();
    assert_eq!(schema.declared_schemas(), [schema_urns::SCHEMA]);
    assert_eq!(schema.schema_urn(), schema_urns::SCHEMA);
    let bare: Schema = serde_json::from_value(serde_json::json!({
        "id": "urn:x", "name": "X", "description": "d", "attributes": [], "meta": {}
    }))
    .unwrap();
    assert!(bare.declared_schemas().is_empty());

    let rt: ResourceType = serde_json::from_value(serde_json::json!({
        "schemas": [schema_urns::RESOURCE_TYPE], "id": "User", "name": "User", "endpoint": "/Users", "schema": schema_urns::USER
    }))
    .unwrap();
    assert_eq!(rt.declared_schemas(), [schema_urns::RESOURCE_TYPE]);
    assert_eq!(rt.schema_urn(), schema_urns::RESOURCE_TYPE);

    let wrapped = Resource::Group(Box::new(group.clone()));
    assert_eq!(wrapped.declared_schemas(), [schema_urns::GROUP]);
    assert_eq!(wrapped.schema_urn(), schema_urns::GROUP);
}

/// A declared-schema mismatch is reported as such for every resource type,
/// not left to fall through to the resource's own `validate`.
#[test]
fn list_response_reports_a_declared_schema_mismatch_for_every_resource_type() {
    fn page<R: ScimResource + serde::de::DeserializeOwned>(
        resource: serde_json::Value,
    ) -> ListResponse<R> {
        serde_json::from_value(serde_json::json!({
            "schemas": [schema_urns::LIST_RESPONSE], "totalResults": 1, "startIndex": 1, "itemsPerPage": 1,
            "Resources": [resource]
        }))
        .unwrap()
    }
    fn mismatch(err: ValidationError) {
        assert_eq!(err.path(), "Resources[0].schemas");
        assert!(err.to_string().contains("does not declare"), "{err}");
    }
    let user = |urn: &str| serde_json::json!({"schemas": [urn], "id": "1", "userName": "u"});
    let group = |urn: &str| serde_json::json!({"schemas": [urn], "id": "1", "displayName": "g"});
    let schema = |urn: &str| serde_json::json!({"schemas": [urn], "id": "urn:x", "name": "X", "description": "d", "attributes": [], "meta": {}});
    let rt = |urn: &str| serde_json::json!({"schemas": [urn], "id": "User", "name": "User", "endpoint": "/Users", "schema": schema_urns::USER});

    assert_eq!(
        page::<User<String>>(user(schema_urns::USER)).validate(),
        Ok(())
    );
    mismatch(
        page::<User<String>>(user(schema_urns::GROUP))
            .validate()
            .unwrap_err(),
    );
    assert_eq!(
        page::<Group<String>>(group(schema_urns::GROUP)).validate(),
        Ok(())
    );
    mismatch(
        page::<Group<String>>(group(schema_urns::USER))
            .validate()
            .unwrap_err(),
    );
    assert_eq!(
        page::<Schema>(schema(schema_urns::SCHEMA)).validate(),
        Ok(())
    );
    mismatch(
        page::<Schema>(schema(schema_urns::USER))
            .validate()
            .unwrap_err(),
    );
    assert_eq!(
        page::<ResourceType>(rt(schema_urns::RESOURCE_TYPE)).validate(),
        Ok(())
    );
    mismatch(
        page::<ResourceType>(rt(schema_urns::SCHEMA))
            .validate()
            .unwrap_err(),
    );
    assert_eq!(
        page::<Resource<String>>(group(schema_urns::GROUP)).validate(),
        Ok(())
    );
}

/// `Resource` delegates both halves of `Validate` to the wrapped resource.
#[test]
fn resource_delegates_validation_to_the_wrapped_resource() {
    let no_name = Resource::User(Box::new(User::<String> {
        schemas: vec![schema_urns::USER.to_string()],
        user_name: String::new(),
        ..Default::default()
    }));
    assert_eq!(no_name.validate().unwrap_err().path(), "userName");
    let no_id: Group<String> = serde_json::from_value(serde_json::json!({
        "schemas": [schema_urns::GROUP], "displayName": "g"
    }))
    .unwrap();
    let no_id = Resource::Group(Box::new(no_id));
    assert_eq!(no_id.validate(), Ok(()));
    assert_eq!(
        no_id
            .validate_context(Context::Response)
            .unwrap_err()
            .path(),
        "id"
    );
    assert_eq!(no_id.validate_context(Context::CreateRequest), Ok(()));
}
