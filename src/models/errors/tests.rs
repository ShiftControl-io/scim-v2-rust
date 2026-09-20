/// The ten §3.12 keywords round-trip as typed variants; an unknown
/// keyword survives as `Other` rather than failing the error body.
#[test]
fn scim_type_round_trips_every_keyword_and_preserves_unknowns() {
    for (wire, variant) in [
        ("invalidFilter", ScimType::InvalidFilter),
        ("tooMany", ScimType::TooMany),
        ("uniqueness", ScimType::Uniqueness),
        ("mutability", ScimType::Mutability),
        ("invalidSyntax", ScimType::InvalidSyntax),
        ("invalidPath", ScimType::InvalidPath),
        ("noTarget", ScimType::NoTarget),
        ("invalidValue", ScimType::InvalidValue),
        ("invalidVers", ScimType::InvalidVers),
        ("sensitive", ScimType::Sensitive),
    ] {
        let parsed: ScimType = serde_json::from_str(&format!("\"{wire}\"")).unwrap();
        assert_eq!(parsed, variant, "{wire}");
        assert_eq!(
            serde_json::to_string(&variant).unwrap(),
            format!("\"{wire}\"")
        );
    }
    let unknown: ScimType = serde_json::from_str("\"vendorSpecific\"").unwrap();
    assert_eq!(unknown, ScimType::Other("vendorSpecific".to_string()));
    assert_eq!(unknown.as_str(), "vendorSpecific");
}

/// RFC 7644 §3.12: the Error URN and a numeric status rendered as a string.
#[test]
fn validate_checks_urn_and_status() {
    let good = ScimHttpError {
        status: "400".to_string(),
        ..Default::default()
    };
    assert!(good.validate().is_ok());
    let bad_status = ScimHttpError {
        status: "400 Bad Request".to_string(),
        ..Default::default()
    };
    assert_eq!(
        bad_status.validate().expect_err("not a code").path(),
        "status"
    );
    let bad_urn = ScimHttpError {
        schemas: vec![schema_urns::USER.to_string()],
        status: "400".to_string(),
        ..Default::default()
    };
    assert_eq!(bad_urn.validate().expect_err("wrong URN").path(), "schemas");
}
use serde_json::json;

use super::*;

#[test]
fn scim_http_error_default_creates_expected_error() {
    let error = ScimHttpError::default();
    assert_eq!(
        error.schemas,
        vec!["urn:ietf:params:scim:api:messages:2.0:Error".to_string()]
    );
    assert_eq!(error.scim_type, None);
    assert_eq!(error.detail, None);
    assert_eq!(error.status, "".to_string());
}

#[test]
fn scim_http_error_deserialize_from_valid_json() {
    let json = json!({
        "schemas": ["urn:ietf:params:scim:api:messages:2.0:Error"],
        "scimType": "invalidValue",
        "detail": "Invalid email address",
        "status": "400"
    });

    let error: ScimHttpError = serde_json::from_value(json).unwrap();
    assert_eq!(
        error.schemas,
        vec!["urn:ietf:params:scim:api:messages:2.0:Error".to_string()]
    );
    assert_eq!(error.scim_type, Some(ScimType::InvalidValue));
    assert_eq!(error.detail, Some("Invalid email address".to_string()));
    assert_eq!(error.status, "400".to_string());
}

#[test]
fn scim_http_error_deserialize_from_invalid_json() {
    let json = json!({
    "status": "400"
     });

    let error: Result<ScimHttpError, _> = serde_json::from_value(json);
    assert!(error.is_err());
}

/// RFC 7644 §3.6 — unnumbered example, the 404 body from "Client's attempt
/// to retrieve the previously deleted User". The error body returned for a
/// DELETE of a resource that does not exist. Verbatim from the RFC;
/// exercises an `Error` payload that carries `detail` and `status` but no
/// `scimType`.
#[test]
fn rfc7644_s3_6_error_response_round_trips() {
    let raw = include_str!("../../test_data/rfc7644/s3.6_error_response.json");
    let err: ScimHttpError =
        serde_json::from_str(raw).expect("RFC 7644 §3.6 error body must deserialize");
    assert_eq!(err.status, "404");
    assert_eq!(
        err.detail.as_deref(),
        Some("Resource 2819c223-7f76-453a-919d-413861904646 not found")
    );
    assert_eq!(err.scim_type, None);

    let reserialized: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&err).unwrap()).unwrap();
    let original: serde_json::Value = serde_json::from_str(raw).unwrap();
    assert_eq!(reserialized, original);
}

#[test]
fn scim_http_error_serialize_to_json() {
    let error = ScimHttpError {
        schemas: vec![schema_urns::ERROR.to_string()],
        scim_type: Some(ScimType::InvalidValue),
        detail: Some("Invalid email address".to_string()),
        status: "400".to_string(),
    };

    let json = serde_json::to_value(&error).unwrap();
    assert_eq!(
        json,
        json!({
            "schemas": ["urn:ietf:params:scim:api:messages:2.0:Error"],
            "scimType": "invalidValue",
            "detail": "Invalid email address",
            "status": "400"
        })
    );
}

/// RFC 7644 §3.12: `status` is "the HTTP status code … expressed as a JSON
/// string", so a numeric string outside 100–599 is not one.
#[test]
fn validate_rejects_a_status_outside_the_http_range() {
    let error = |status: &str| ScimHttpError {
        schemas: vec![schema_urns::ERROR.to_string()],
        scim_type: None,
        detail: None,
        status: status.to_string(),
    };
    assert_eq!(error("400").validate(), Ok(()));
    assert_eq!(error("599").validate(), Ok(()));
    for bad in ["99", "600", "999", "0", "-400", "4xx", ""] {
        assert_eq!(
            error(bad).validate().unwrap_err().path(),
            "status",
            "{bad:?}"
        );
    }
}

/// `ScimType` prints its wire keyword, which is what the error body carries.
#[test]
fn scim_type_displays_its_wire_keyword() {
    assert_eq!(ScimType::InvalidValue.to_string(), "invalidValue");
    assert_eq!(ScimType::TooMany.to_string(), "tooMany");
    assert_eq!(
        ScimType::Other("vendorSpecific".to_string()).to_string(),
        "vendorSpecific"
    );
    assert_eq!(
        serde_json::to_string(&ScimType::InvalidFilter).unwrap(),
        "\"invalidFilter\""
    );
}
