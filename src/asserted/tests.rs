use super::Asserted;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Holder {
    #[serde(
        default = "Asserted::absent",
        skip_serializing_if = "Asserted::is_absent"
    )]
    title: Asserted<String>,
    #[serde(
        default = "Asserted::absent",
        skip_serializing_if = "Asserted::is_absent"
    )]
    roles: Asserted<Vec<String>>,
}

fn wire(body: serde_json::Value) -> Holder {
    serde_json::from_value(body).expect("deserializes")
}

#[test]
fn every_wire_form_survives_a_read_and_a_write() {
    for body in [
        json!({}),
        json!({"title": null, "roles": null}),
        json!({"title": "Tour Guide", "roles": []}),
        json!({"title": "Tour Guide", "roles": ["admin"]}),
    ] {
        let out = serde_json::to_value(wire(body.clone())).unwrap();
        assert_eq!(out, body, "{body}");
    }
}

#[test]
fn the_wire_forms_land_in_the_three_states() {
    assert!(wire(json!({})).title.is_absent());
    assert!(wire(json!({"title": null})).title.is_nulled());
    assert_eq!(
        wire(json!({"title": "x"})).title.as_option(),
        Some(&"x".to_string())
    );

    assert!(wire(json!({})).roles.is_absent());
    assert!(wire(json!({"roles": null})).roles.is_nulled());
    assert!(wire(json!({"roles": []})).roles.is_asserted());
    assert_eq!(wire(json!({"roles": ["a"]})).roles.len(), 1);
}

#[test]
fn a_proxy_can_normalise_a_clear_into_silence() {
    let mut holder = wire(json!({"title": null}));
    assert!(holder.title.is_nulled());
    holder.title = Asserted::absent();
    assert_eq!(serde_json::to_value(&holder).unwrap(), json!({}));
}

#[test]
fn a_proxy_can_turn_silence_into_a_clear() {
    let mut holder = wire(json!({}));
    holder.title = Asserted::nulled();
    assert_eq!(
        serde_json::to_value(&holder).unwrap(),
        json!({"title": null})
    );
}

#[test]
fn an_empty_array_and_a_null_are_both_asserted_and_both_read_empty() {
    for body in [json!({"roles": []}), json!({"roles": null})] {
        let holder = wire(body.clone());
        assert!(holder.roles.is_asserted(), "{body}");
        assert!(holder.roles.is_empty(), "{body}");
    }
}

#[test]
fn a_write_asserts_and_clear_keeps_the_assertion() {
    let mut roles: Asserted<Vec<String>> = Asserted::absent();
    roles.push("admin".to_string());
    assert!(roles.is_asserted());
    assert_eq!(roles.as_slice(), ["admin".to_string()]);

    roles = Asserted::set(Vec::new()); // RFC 7644 §3.5.1 names `[]` for a multi-valued attribute
    assert!(roles.is_asserted() && !roles.is_nulled() && roles.is_empty());

    roles = Asserted::absent();
    assert!(roles.is_absent());
}

#[test]
fn removal_from_an_unset_attribute_asserts_nothing() {
    let mut absent: Asserted<Vec<String>> = Asserted::absent();
    assert_eq!(absent.pop(), None);
    absent.retain(|_| false);
    assert!(absent.is_absent());

    let mut cleared: Asserted<Vec<String>> = Asserted::nulled();
    assert_eq!(cleared.pop(), None);
    cleared.retain(|_| false);
    assert!(cleared.is_nulled());
}

#[test]
fn a_read_cannot_tell_absent_from_nulled() {
    for attribute in [Asserted::<Vec<u8>>::absent(), Asserted::nulled()] {
        assert!(attribute.is_empty());
        assert_eq!(attribute.first(), None);
        assert_eq!(attribute.iter().count(), 0);
    }
    for attribute in [Asserted::<String>::absent(), Asserted::nulled()] {
        assert_eq!(attribute.as_option(), None);
        assert_eq!(attribute.into_option(), None);
    }
}

#[test]
fn equality_compares_the_state() {
    assert_ne!(Asserted::<String>::absent(), Asserted::<String>::nulled());
    assert_eq!(Asserted::<String>::absent(), Asserted::default());
    assert_eq!(Asserted::from("a"), Asserted::set("a"));
    assert_eq!(Asserted::from_iter(["a"]), Asserted::set(vec!["a"]));
}

#[test]
fn map_keeps_the_state() {
    assert!(Asserted::<u8>::absent().map(|v| v + 1).is_absent());
    assert!(Asserted::<u8>::nulled().map(|v| v + 1).is_nulled());
    assert_eq!(Asserted::set(1u8).map(|v| v + 1), Asserted::set(2));
}

#[test]
fn mutation_helpers_reach_the_values() {
    let mut roles = Asserted::set(vec!["a".to_string(), "b".to_string()]);
    roles[0] = "z".to_string(); // DerefMut, which cannot change assertion
    assert_eq!(roles.as_slice(), ["z".to_string(), "b".to_string()]);

    roles.extend(["c".to_string()]);
    assert_eq!(roles.len(), 3);
    assert_eq!((&roles).into_iter().count(), 3);
    assert_eq!(roles.clone().into_iter().count(), 3);

    let mut title = Asserted::set("a".to_string());
    if let Some(value) = title.as_mut() {
        value.push('!');
    }
    assert_eq!(title.as_option(), Some(&"a!".to_string()));
}

#[test]
fn extend_asserts_even_with_an_empty_iterator() {
    let mut roles: Asserted<Vec<String>> = Asserted::absent();
    roles.extend(std::iter::empty());
    assert!(roles.is_asserted() && roles.is_empty());
}

#[test]
fn clears_values_counts_both_forms_the_rfc_names() {
    // RFC 7644 §3.5.1 names `null` and `[]` for a multi-valued attribute.
    assert!(wire(json!({"roles": null})).roles.clears_values());
    assert!(wire(json!({"roles": []})).roles.clears_values());
    assert!(!wire(json!({"roles": ["a"]})).roles.clears_values());
    assert!(
        !wire(json!({})).roles.clears_values(),
        "silence clears nothing"
    );

    // A single-valued attribute has one form, so `is_nulled` is the answer.
    assert!(wire(json!({"title": null})).title.is_nulled());
    assert!(!wire(json!({"title": "x"})).title.is_nulled());
    assert!(!wire(json!({})).title.is_nulled());
}
