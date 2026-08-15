use serde::Deserializer;

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
