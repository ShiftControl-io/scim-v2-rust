use super::Multi;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Holder {
    #[serde(default, skip_serializing_if = "Multi::is_absent")]
    values: Multi<String>,
}

fn wire(json: serde_json::Value) -> Holder {
    serde_json::from_value(json).expect("deserializes")
}

#[test]
fn the_three_wire_forms_land_in_two_states() {
    assert!(wire(json!({})).values.is_absent());
    assert!(wire(json!({"values": null})).values.is_asserted());
    assert!(wire(json!({"values": []})).values.is_asserted());
    assert!(wire(json!({"values": ["a"]})).values.is_asserted());
}

#[test]
fn a_read_followed_by_a_write_does_not_change_the_message() {
    for body in [
        json!({}),
        json!({"values": []}),
        json!({"values": ["a", "b"]}),
    ] {
        let round_tripped = serde_json::to_value(wire(body.clone())).unwrap();
        assert_eq!(round_tripped, body, "{body}");
    }
}

#[test]
fn null_comes_back_as_an_empty_array() {
    // RFC 7644 §3.5.1 makes `null` and `[]` one instruction, so the
    // instruction survives even though the spelling does not.
    let out = serde_json::to_value(wire(json!({"values": null}))).unwrap();
    assert_eq!(out, json!({"values": []}));
}

#[test]
fn an_absent_attribute_stays_off_the_wire() {
    let out = serde_json::to_value(wire(json!({}))).unwrap();
    assert_eq!(out, json!({}));
}

#[test]
fn a_cleared_attribute_stays_on_the_wire() {
    let mut holder = wire(json!({"values": ["a"]}));
    holder.values.clear();
    assert_eq!(
        serde_json::to_value(&holder).unwrap(),
        json!({"values": []})
    );
}

#[test]
fn unset_takes_an_attribute_back_off_the_wire() {
    let mut holder = wire(json!({"values": ["a"]}));
    holder.values.unset();
    assert!(holder.values.is_absent());
    assert_eq!(serde_json::to_value(&holder).unwrap(), json!({}));
}

#[test]
fn a_write_asserts_an_absent_attribute() {
    let mut values = Multi::absent();
    assert!(values.is_absent());
    values.push("a".to_string());
    assert!(values.is_asserted());
    assert_eq!(values.as_slice(), ["a".to_string()]);
}

#[test]
fn as_mut_vec_asserts_an_absent_attribute() {
    let mut values: Multi<String> = Multi::absent();
    values.as_mut_vec().push("a".to_string());
    assert!(values.is_asserted());
}

#[test]
fn a_read_cannot_tell_absent_from_cleared() {
    let absent: Multi<String> = Multi::absent();
    let cleared: Multi<String> = Multi::cleared();
    for values in [&absent, &cleared] {
        assert!(values.is_empty());
        assert_eq!(values.len(), 0);
        assert_eq!(values.first(), None);
        assert_eq!(values.iter().count(), 0);
    }
}

#[test]
fn equality_compares_the_state() {
    assert_ne!(Multi::<String>::absent(), Multi::<String>::cleared());
    assert_eq!(Multi::<String>::absent(), Multi::default());
    assert_eq!(Multi::from(vec!["a"]), Multi::from_iter(["a"]));
}

#[test]
fn conversions_assert_the_attribute() {
    assert!(Multi::from(vec!["a"]).is_asserted());
    assert!(Multi::<&str>::from(Vec::new()).is_asserted()); // an empty Vec is a clear
    assert!(Multi::<&str>::from(None).is_absent());
    assert!(Multi::from(Some(vec!["a"])).is_asserted());
}

#[test]
fn iteration_works_by_value_and_by_reference() {
    let values = Multi::from(vec!["a".to_string(), "b".to_string()]);
    assert_eq!((&values).into_iter().count(), 2);
    assert_eq!(values.clone().into_iter().count(), 2);
    let mut seen = Vec::new();
    for value in &values {
        seen.push(value.clone());
    }
    assert_eq!(seen, ["a".to_string(), "b".to_string()]);
}

#[test]
fn into_vec_and_into_inner_expose_both_views() {
    assert_eq!(Multi::<String>::absent().into_vec(), Vec::<String>::new());
    assert_eq!(Multi::<String>::absent().into_inner(), None);
    assert_eq!(Multi::<String>::cleared().into_inner(), Some(Vec::new()));
}

#[test]
fn retain_filters_without_asserting_an_absent_attribute() {
    let mut values = Multi::from(vec!["a".to_string(), "bb".to_string()]);
    values.retain(|v| v.len() == 1);
    assert_eq!(values.as_slice(), ["a".to_string()]);

    let mut absent: Multi<String> = Multi::absent();
    absent.retain(|_| false);
    assert!(absent.is_absent(), "removing nothing asserts nothing");
}

#[test]
fn pop_removes_without_asserting_an_absent_attribute() {
    let mut values = Multi::from(vec!["a".to_string()]);
    assert_eq!(values.pop(), Some("a".to_string()));
    assert!(values.is_asserted() && values.is_empty());

    let mut absent: Multi<String> = Multi::absent();
    assert_eq!(absent.pop(), None);
    assert!(absent.is_absent(), "removing nothing asserts nothing");
}

#[test]
fn extend_asserts_even_with_an_empty_iterator() {
    let mut values: Multi<String> = Multi::absent();
    values.extend(["a".to_string()]);
    assert_eq!(values.as_slice(), ["a".to_string()]);

    let mut empty: Multi<String> = Multi::absent();
    empty.extend(std::iter::empty());
    assert!(
        empty.is_asserted(),
        "a write asserts, even a write of nothing"
    );
    assert!(empty.is_empty());
}

#[test]
fn as_ref_and_deref_mut_expose_the_values() {
    let mut values = Multi::from(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(AsRef::<[String]>::as_ref(&values), values.as_slice());
    assert_eq!(AsRef::<[String]>::as_ref(&values).len(), 2);

    values[0] = "z".to_string(); // DerefMut, which cannot change assertion
    assert_eq!(values.as_slice(), ["z".to_string(), "b".to_string()]);
    assert!(values.is_asserted());
}
