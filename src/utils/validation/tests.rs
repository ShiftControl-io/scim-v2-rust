use super::*;

#[test]
fn missing_required_reports_the_wire_path() {
    let err = ValidationError::missing_required("userName");
    assert_eq!(err.path(), "userName");
    assert_eq!(err.kind(), &ValidationErrorKind::MissingRequiredAttribute);
    assert_eq!(
        err.to_string(),
        "userName: required attribute is missing or empty"
    );
}

/// A nested attribute is reported in dotted wire notation, the same
/// notation a filter or PATCH path uses, so a server can hand it straight
/// back to the client.
#[test]
fn invalid_value_carries_its_detail_and_a_dotted_path() {
    let err = ValidationError::invalid_value("name.familyName", "must not be blank");
    assert_eq!(err.path(), "name.familyName");
    assert_eq!(
        err.kind(),
        &ValidationErrorKind::InvalidValue("must not be blank".to_string())
    );
    assert_eq!(err.to_string(), "name.familyName: must not be blank");
}

/// RFC 7644 §3.12: `invalidValue` covers "a required value was missing, or
/// the value specified was not compatible with the operation or attribute
/// type", which is both of the current kinds.
#[test]
fn both_kinds_map_to_invalid_value() {
    assert_eq!(
        ValidationError::missing_required("schemas").scim_type_str(),
        "invalidValue"
    );
    assert_eq!(
        ValidationError::invalid_value("x", "y").scim_type_str(),
        "invalidValue"
    );
}

/// The RFC 7644 §3.12 body a server returns. `status` is a JSON *string*
/// per §3.12, not a number.
#[cfg(feature = "models")]
#[test]
fn to_http_error_builds_the_rfc_error_body() {
    let err = ValidationError::missing_required("userName");
    let body = err.to_http_error();

    assert_eq!(body.schemas, vec![crate::schema_urns::ERROR]);
    assert_eq!(body.scim_type, Some(ScimType::InvalidValue));
    assert_eq!(
        body.detail.as_deref(),
        Some("userName: required attribute is missing or empty")
    );
    assert_eq!(body.status, "400");

    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(json["scimType"], "invalidValue");
    assert_eq!(json["status"], "400", "status must serialize as a string");
}

/// RFC 7643 §3: non-empty, unique, and naming the required URN. Extra
/// URNs are allowed (an unmodelled extension), a duplicate of any URN is
/// not — including a duplicate of the required one, which is the case a
/// membership-only check waves through.
#[test]
fn require_schema_urn_enforces_non_empty_unique_and_present() {
    let req = "urn:ietf:params:scim:schemas:core:2.0:User";
    let ext = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User";
    let ok =
        |v: &[&str]| require_schema_urn(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>(), req);

    assert_eq!(ok(&[req]), Ok(()));
    assert_eq!(ok(&[req, ext]), Ok(()));
    assert_eq!(ok(&[ext, req]), Ok(()));

    assert_eq!(ok(&[]), Err(ValidationError::missing_required("schemas")));
    let err = ok(&[ext]).unwrap_err();
    assert_eq!(err.path(), "schemas");
    assert!(err.to_string().contains("must include"), "{err}");

    for dup in [&[req, req][..], &[req, ext, ext], &[ext, req, ext]] {
        let err = ok(dup).unwrap_err();
        assert_eq!(err.path(), "schemas");
        assert!(err.to_string().contains("duplicate value"), "{err}");
        assert_eq!(err.scim_type_str(), "invalidValue");
    }
}

/// The duplicate rule reaches every type through the shared helper; this
/// pins that `Strict` cannot certify a payload RFC 7643 §3 forbids.
#[cfg(feature = "models")]
#[test]
fn strict_rejects_duplicate_schema_urns() {
    use crate::models::user::User;
    let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User","urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"}"#;
    let err = serde_json::from_str::<Strict<User<String>, CreateRequest>>(body).unwrap_err();
    assert!(err.to_string().contains("duplicate value"), "{err}");
    // The lenient model still reads it, and reports the same failure on demand.
    let user: User<String> = serde_json::from_str(body).unwrap();
    assert_eq!(user.validate().unwrap_err().path(), "schemas");
}

/// `ValidationError` is comparable, which is what lets tests assert on a
/// whole error rather than on its rendered string.
#[test]
fn errors_compare_by_path_and_kind() {
    assert_eq!(
        ValidationError::missing_required("a"),
        ValidationError::missing_required("a")
    );
    assert_ne!(
        ValidationError::missing_required("a"),
        ValidationError::missing_required("b")
    );
    assert_ne!(
        ValidationError::missing_required("a"),
        ValidationError::invalid_value("a", "detail")
    );
}

/// R3-I1: the labels consumers put in their log lines, one per variant.
#[test]
fn context_as_str_names_each_variant() {
    assert_eq!(Context::CreateRequest.as_str(), "create request");
    assert_eq!(Context::ReplaceRequest.as_str(), "replace request");
    assert_eq!(Context::Response.as_str(), "response");
}

/// Devin round 2, ANALYSIS-2: relocating an error under a container keeps
/// its kind and detail; only the path gains the prefix.
#[test]
fn under_prefixes_the_path_and_keeps_kind_and_detail() {
    let err = ValidationError::invalid_value("type", "must be one of oauth2, httpbasic")
        .under("authenticationSchemes[1]");
    assert_eq!(err.path(), "authenticationSchemes[1].type");
    assert_eq!(
        err.kind(),
        &ValidationErrorKind::InvalidValue("must be one of oauth2, httpbasic".to_string())
    );
    let err = ValidationError::missing_required("userName").under("Resources[3]");
    assert_eq!(err.path(), "Resources[3].userName");
    assert_eq!(err.kind(), &ValidationErrorKind::MissingRequiredAttribute);
}
