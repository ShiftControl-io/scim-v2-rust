use serde::Deserializer;

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
