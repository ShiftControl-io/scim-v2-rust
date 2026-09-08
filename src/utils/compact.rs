//! Compact serialization: omit unassigned multi-valued attributes.
//!
//! By default this crate serializes an empty multi-valued attribute as `[]`,
//! because RFC 7644 §3.5.1 gives `[]` a meaning omission lacks — "an empty
//! array `[]` for a multi-valued attribute, to clear all values" — and these
//! models are request bodies as well as representations. RFC 7643 §2.5 only
//! *permits* omitting an unassigned attribute "for compactness".
//!
//! A server serializing a response it knows is not a request may prefer the
//! compact form. That choice belongs to the caller, so it is a wrapper type
//! rather than a Cargo feature: Cargo unifies features across the dependency
//! graph, so a feature would let any transitive crate change what every other
//! consumer puts on the wire.

use serde::Serialize;
use serde_json::Value;

/// Remove every `null` and every empty array from `value`, recursively.
///
/// Both are "unassigned" under RFC 7643 §2.5. The crate's serializers never
/// emit `null` for an unset `Option`, so in practice this removes `[]`.
pub fn strip_unassigned(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null() && v.as_array().is_none_or(|a| !a.is_empty()));
            map.values_mut().for_each(strip_unassigned);
        }
        Value::Array(items) => items.iter_mut().for_each(strip_unassigned),
        _ => {}
    }
}

/// Serialize `T` with unassigned multi-valued attributes omitted.
///
/// The stripping is tree-wide and not field-aware: every empty array goes,
/// including protocol arrays such as a `ListResponse`'s `Resources` and a
/// `PatchOp`'s `Operations`, and arrays inside extension payloads this crate
/// does not model. That is deliberate. RFC 7643 §2.5 applies to every
/// multi-valued attribute, extensions included, and no conformant payload is
/// altered by it: `Resources` is "REQUIRED if totalResults is non-zero"
/// (RFC 7644 §3.4.2), so an empty page may omit it; `schemas` and
/// `Operations` are REQUIRED non-empty, so an empty one was already invalid
/// and is merely invalid in its other form. Non-empty arrays are never
/// touched.
///
/// ```
/// # #[cfg(feature = "models")] {
/// use scim_v2::{models::user::User, utils::compact::Compact};
///
/// let user = User::<String> {
///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
///     user_name: "bjensen".to_string(),
///     ..Default::default()
/// };
/// // Default form keeps a clear-all expressible:
/// assert!(serde_json::to_string(&user).unwrap().contains(r#""emails":[]"#));
/// // Compact form omits it:
/// assert!(!serde_json::to_string(&Compact(&user)).unwrap().contains("emails"));
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Compact<'a, T: Serialize>(pub &'a T);

impl<T: Serialize> Serialize for Compact<'_, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut v = serde_json::to_value(self.0).map_err(serde::ser::Error::custom)?;
        strip_unassigned(&mut v);
        v.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Devin review on #49, ANALYSIS-1: the one protocol array a conformant
    /// payload can legitimately hold empty is `Resources` on an empty page,
    /// and RFC 7644 §3.4.2 permits omitting it, so the compact form still
    /// deserializes and validates.
    #[cfg(feature = "models")]
    #[test]
    fn compact_empty_page_omits_resources_and_still_validates() {
        use crate::Validate;
        use crate::models::{others::ListResponse, user::User};

        let page: ListResponse<User<String>> = ListResponse {
            schemas: vec![crate::schema_urns::LIST_RESPONSE.to_string()],
            total_results: 0,
            items_per_page: None,
            start_index: None,
            resources: vec![],
        };
        let json = serde_json::to_value(Compact(&page)).unwrap();
        assert!(json.get("Resources").is_none(), "{json}");
        assert_eq!(json["schemas"].as_array().map(Vec::len), Some(1));
        let back: ListResponse<User<String>> = serde_json::from_value(json).unwrap();
        assert_eq!(back.validate(), Ok(()));
        assert_eq!(back, page);
    }

    #[cfg(feature = "models")]
    #[test]
    fn compact_never_touches_non_empty_arrays() {
        use crate::models::user::{Email, User};

        let user = User::<String> {
            schemas: vec![crate::schema_urns::USER.to_string()],
            user_name: "bjensen".to_string(),
            emails: vec![Email {
                value: Some("bjensen@example.com".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_value(Compact(&user)).unwrap();
        assert_eq!(json["schemas"].as_array().map(Vec::len), Some(1));
        assert_eq!(json["emails"].as_array().map(Vec::len), Some(1));
        assert!(json.get("roles").is_none());
    }

    #[test]
    fn strips_empty_arrays_and_nulls_recursively() {
        let mut v = serde_json::json!({
            "a": [], "b": null, "c": [1], "d": {"e": [], "f": "x", "g": [{"h": []}]}
        });
        strip_unassigned(&mut v);
        assert_eq!(v, serde_json::json!({"c": [1], "d": {"f": "x", "g": [{}]}}));
    }
}
