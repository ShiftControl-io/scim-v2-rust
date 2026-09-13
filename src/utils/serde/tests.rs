use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct WithDefault {
    #[serde(default, deserialize_with = "deserialize_optional_lenient_bool")]
    primary: Option<bool>,
}

#[derive(Deserialize)]
struct WithoutDefault {
    #[serde(deserialize_with = "deserialize_optional_lenient_bool")]
    #[allow(dead_code)]
    primary: Option<bool>,
}

#[test]
fn accepts_a_real_bool() {
    let parsed: WithDefault = serde_json::from_str(r#"{"primary": true}"#).unwrap();
    assert_eq!(parsed.primary, Some(true));

    let parsed: WithDefault = serde_json::from_str(r#"{"primary": false}"#).unwrap();
    assert_eq!(parsed.primary, Some(false));
}

#[test]
fn accepts_a_stringified_bool_case_insensitively() {
    let parsed: WithDefault = serde_json::from_str(r#"{"primary": "True"}"#).unwrap();
    assert_eq!(parsed.primary, Some(true));

    let parsed: WithDefault = serde_json::from_str(r#"{"primary": "false"}"#).unwrap();
    assert_eq!(parsed.primary, Some(false));
}

#[test]
fn rejects_a_non_boolean_string() {
    let result: Result<WithDefault, _> = serde_json::from_str(r#"{"primary": "maybe"}"#);
    assert!(result.is_err());
}

#[test]
fn explicit_null_deserializes_to_none() {
    let parsed: WithDefault = serde_json::from_str(r#"{"primary": null}"#).unwrap();
    assert_eq!(parsed.primary, None);
}

#[test]
fn omitted_key_deserializes_to_none_when_paired_with_serde_default() {
    let parsed: WithDefault = serde_json::from_str(r#"{}"#).unwrap();
    assert_eq!(parsed.primary, None);
}

#[test]
fn omitted_key_fails_without_serde_default() {
    let result: Result<WithoutDefault, _> = serde_json::from_str(r#"{}"#);
    assert!(
        result.is_err(),
        "deserialize_with alone does not default an omitted key to None; \
             #[serde(default)] is required at the call site"
    );
}
