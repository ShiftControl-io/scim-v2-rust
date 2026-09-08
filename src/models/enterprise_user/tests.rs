use super::*;
use crate::Validate;

/// Restored from `main`. Added as the fix for #48's review finding M-1,
/// and removed by this PR's rewrite of the module — which raised coverage
/// from 0% to 93% while deleting its only guard against the defect the
/// module had been fixed for. Verified by mutation: dropping
/// `skip_serializing_if` from `Manager.value` survives the extension
/// round-trip test, and fails this one.
#[test]
fn manager_omits_all_unset_fields_on_serialize() {
    assert_eq!(
        serde_json::to_value(Manager::default()).unwrap(),
        serde_json::json!({}),
        "a wholly unset Manager must serialize to {{}}, not to nulls"
    );
}

/// A partially populated `Manager` carries only what is assigned.
#[test]
fn manager_omits_the_unset_subset_on_serialize() {
    let manager = Manager {
        value: Some("26118915".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_value(&manager).unwrap();
    assert_eq!(json["value"], "26118915");
    let obj = json.as_object().unwrap();
    assert!(!obj.contains_key("$ref"), "unset $ref must be omitted");
    assert!(
        !obj.contains_key("displayName"),
        "unset displayName must be omitted"
    );
}

/// The read path keeps accepting an explicit `null` for each sub-attribute
/// (RFC 7643 §2.5) — a missing key, `null`, and `{}` all give `None`.
#[test]
fn manager_explicit_null_deserializes_to_none() {
    let m: Manager =
        serde_json::from_str(r#"{"value": null, "$ref": null, "displayName": null}"#).unwrap();
    assert_eq!(m.value, None);
    assert_eq!(m.r#ref, None);
    assert_eq!(m.display_name, None);

    let empty: Manager = serde_json::from_str("{}").unwrap();
    assert_eq!(empty.value, None);
}

/// Regression guard for the pre-1.0 bug. Every attribute in RFC 7643 §4.3
/// is `required: false` in the schema this crate embeds, so a wholly empty
/// extension is conformant. `validate` used to demand all six.
#[test]
fn an_empty_extension_is_valid() {
    assert!(EnterpriseUser::default().validate().is_ok());
}

/// A partially populated extension is the common real case — a provider
/// that tracks a department but no cost centre.
#[test]
fn a_partial_extension_is_valid() {
    let partial = EnterpriseUser {
        department: Some("Engineering".to_string()),
        ..Default::default()
    };
    assert!(partial.validate().is_ok());
}

/// RFC 7643 §4.3's own example, carrying every attribute including the
/// nested `manager`.
#[test]
fn full_extension_round_trips() {
    let raw = r#"{
            "employeeNumber": "701984",
            "costCenter": "4130",
            "organization": "Universal Studios",
            "division": "Theme Park",
            "department": "Tour Operations",
            "manager": {
                "value": "26118915-6090-4610-87e4-49d8ca9f808d",
                "$ref": "../Users/26118915-6090-4610-87e4-49d8ca9f808d",
                "displayName": "John Smith"
            }
        }"#;

    let eu: EnterpriseUser = serde_json::from_str(raw).expect("§4.3 example must deserialize");
    assert_eq!(eu.employee_number.as_deref(), Some("701984"));
    assert_eq!(eu.division.as_deref(), Some("Theme Park"));
    let manager = eu.manager.as_ref().expect("manager must deserialize");
    assert_eq!(manager.display_name.as_deref(), Some("John Smith"));
    assert!(eu.validate().is_ok());

    let back: serde_json::Value = serde_json::to_value(&eu).unwrap();
    let original: serde_json::Value = serde_json::from_str(raw).unwrap();
    assert_eq!(back, original, "no §4.3 attribute may be dropped");
}

/// An unassigned extension serializes to an empty object rather than one
/// padded with nulls, per RFC 7643 §2.5.
#[test]
fn default_serializes_to_an_empty_object() {
    assert_eq!(
        serde_json::to_value(EnterpriseUser::default()).unwrap(),
        serde_json::json!({})
    );
}
