use serde::Deserializer;

/// Reduce a JSON value to its *assigned* attributes. Remove `null` and `[]`
/// recursively.
///
/// RFC 7643 §2.5 states: "Unassigned attributes, the null value, or an
/// empty array ... SHALL be considered to be equivalent in state". The RFC
/// also permits a server to omit these attributes: "MAY be omitted for
/// compactness". This crate emits `[]` instead of omitting the attribute.
/// RFC 7644 §3.5.1 gives `[]` an operational meaning in a request body. The
/// RFC's own sample payloads omit the attribute instead. A raw byte
/// comparison against those fixtures tests formatting, not fidelity. Each
/// round-trip assertion therefore reduces both sides first.
///
/// This function removes only `null` and `[]`. Every attribute that carries
/// real data survives on both sides. The round-trip assertions check this
/// survival.
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

/// Deserializes a multi-valued attribute. Collapses an explicit `null` to an
/// empty `Vec`.
///
/// RFC 7643 §2.5: "Unassigned attributes, the null value, or an empty array
/// (in the case of a multi-valued attribute) SHALL be considered to be
/// equivalent in state." `#[serde(default)]` alone covers only the *absent*
/// case. It rejects `"roles": null` with the error `invalid type: null,
/// expected a sequence`. Real providers do send this value. Pair this
/// function with `#[serde(default = "Vec::new")]` so all three wire forms
/// map to the same in-memory value.
// The only callers are `SearchRequest::attributes` and `excluded_attributes`,
// which the `filter` feature gates. Every resource attribute now uses
// `Multi<T>`, whose own `Deserialize` handles the three wire forms.
#[cfg(feature = "filter")]
pub(crate) fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    use serde::Deserialize;
    Ok(Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default())
}

/// Deserializes a boolean value from a JSON boolean, a string that
/// represents a boolean, or a JSON null.
///
/// Pair this deserializer with `#[serde(default)]` to handle a missing
/// field.
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
mod tests;
