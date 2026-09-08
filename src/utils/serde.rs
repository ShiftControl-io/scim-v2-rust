use serde::{Deserialize, Deserializer};

/// Reduce a JSON value to its *assigned* attributes, recursively, dropping
/// `null` and `[]`.
///
/// RFC 7643 §2.5: "Unassigned attributes, the null value, or an empty array
/// ... SHALL be considered to be equivalent in state", and such attributes
/// "MAY be omitted for compactness". This crate emits `[]` rather than
/// omitting, because RFC 7644 §3.5.1 gives `[]` operational meaning on a
/// request body, while the RFC's own sample payloads omit. Comparing raw bytes
/// against those fixtures would therefore test a formatting choice rather than
/// fidelity, so round-trip assertions reduce both sides first.
///
/// Only `null` and `[]` are removed. Every attribute carrying real data
/// survives on both sides, which is what those assertions are for.
#[cfg(test)]
pub(crate) fn drop_unassigned(v: serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    match v {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, val)| !val.is_null() && val.as_array() != Some(&vec![]))
                .map(|(k, val)| (k, drop_unassigned(val)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(drop_unassigned).collect()),
        other => other,
    }
}

/// Deserializes a multi-valued attribute, collapsing an explicit `null` to an
/// empty `Vec`.
///
/// RFC 7643 §2.5: "Unassigned attributes, the null value, or an empty array
/// (in the case of a multi-valued attribute) SHALL be considered to be
/// equivalent in state." `#[serde(default)]` alone covers only the *absent*
/// case and rejects `"roles": null` with `invalid type: null, expected a
/// sequence`, which real providers do send. Pair this with
/// `#[serde(default = "Vec::new")]` so all three wire forms land on the same
/// in-memory value.
pub(crate) fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default())
}

/// Deserializes to a boolean from either a boolean, string representation of boolean, or null.
///
/// Note: Must be paired with `#[serde(default)]` in order to handle missing fields.
pub(crate) fn deserialize_optional_lenient_bool<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    struct OptionalLenientBoolVisitor;

    impl<'de> de::Visitor<'de> for OptionalLenientBoolVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean, a string representing a boolean, or null")
        }

        fn visit_bool<E: de::Error>(self, v: bool) -> Result<Option<bool>, E> {
            Ok(Some(v))
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Option<bool>, E> {
            // RFC 7643 §2.3.2 defines a boolean as the JSON literal; accepting
            // the string form is an accommodation of providers that send
            // `"True"`. Always on: it only widens what is accepted, and a
            // Cargo feature would be flippable by any crate in the graph.
            if v.eq_ignore_ascii_case("true") {
                Ok(Some(true))
            } else if v.eq_ignore_ascii_case("false") {
                Ok(Some(false))
            } else {
                Err(E::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        fn visit_none<E: de::Error>(self) -> Result<Option<bool>, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Option<bool>, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionalLenientBoolVisitor)
}

#[cfg(test)]
mod tests {
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
}
