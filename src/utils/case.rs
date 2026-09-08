//! Case-insensitive attribute names, per RFC 7643 §2.1.
//!
//! "Attribute names are case insensitive and are often camel-cased." A
//! conformant peer may therefore send `"USERNAME"` or `"displayname"`, and a
//! `serde` derive matching exact strings drops the key and then fails on the
//! missing required field. This module rewrites every known attribute name,
//! sub-attribute name, protocol member (`Resources`, `Operations`) and schema
//! URN to its canonical spelling before deserialization.
//!
//! Three rules keep this from being lossy. A key with no case-insensitive
//! match is left exactly as it arrived. A subtree under an extension URN this
//! crate does not model (`urn:example:…`) is left byte-identical, because that
//! namespace is not governed by RFC 7643 §2.1 and its vendor may well be
//! case-sensitive. And two keys that fold to the same attribute — `userName`
//! and `USERNAME` in one object — are an error rather than a silent
//! last-write-wins, since §2.1 makes them the same attribute asserted twice
//! and nothing in either RFC says which value wins.
//!
//! The Java SCIM SDK does the same by enabling Jackson's
//! `ACCEPT_CASE_INSENSITIVE_PROPERTIES`; scim2-models lowercases every key in
//! a pre-validator citing §2.1. Nothing in this crate calls it implicitly:
//! the plain derives stay exact-match, and you reach for [`CaseInsensitive`]
//! or [`from_str`] / [`from_value`] where a peer's casing cannot be trusted.
//! Gated behind the `case-insensitive` feature (on by default).

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

/// Two keys in one object fold to the same attribute name.
///
/// RFC 7643 §2.1 makes `userName` and `USERNAME` the same attribute, so a
/// payload carrying both has asserted one attribute twice, possibly with
/// different values, and neither RFC defines a precedence. Resolving it
/// silently would let whichever spelling sorts later decide what a handler
/// sees, which is an attribute-smuggling vector at a request boundary; the
/// only safe answer is to refuse the body (RFC 7644 §3.12 `invalidSyntax`).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "attribute {canonical:?} is given more than once with differing case ({first:?} and {second:?}); RFC 7643 §2.1 makes these the same attribute"
)]
pub struct AmbiguousKey {
    /// The canonical spelling both keys fold to.
    pub canonical: String,
    /// The spelling seen first.
    pub first: String,
    /// The spelling seen second.
    pub second: String,
}

/// Every canonical wire name this crate models, generated from the embedded
/// RFC 7643 schemas plus the protocol messages. A test asserts it stays
/// complete against those schemas.
pub(crate) const WIRE_NAMES: &[&str] = &[
    "path",
    "op",
    "$ref",
    "active",
    "add",
    "addresses",
    "attributes",
    "authenticationSchemes",
    "bulk",
    "canonicalValues",
    "caseExact",
    "changePassword",
    "costCenter",
    "count",
    "country",
    "created",
    "department",
    "description",
    "detail",
    "display",
    "displayName",
    "division",
    "documentationUri",
    "emails",
    "employeeNumber",
    "endpoint",
    "enterpriseUser",
    "entitlements",
    "etag",
    "excludedAttributes",
    "externalId",
    "familyName",
    "filter",
    "formatted",
    "givenName",
    "groups",
    "honorificPrefix",
    "honorificSuffix",
    "id",
    "ims",
    "itemsPerPage",
    "lastModified",
    "locale",
    "locality",
    "location",
    "manager",
    "maxOperations",
    "maxPayloadSize",
    "maxResults",
    "members",
    "meta",
    "middleName",
    "multiValued",
    "mutability",
    "name",
    "nickName",
    "Operations",
    "organization",
    "password",
    "patch",
    "phoneNumbers",
    "photos",
    "postalCode",
    "preferredLanguage",
    "primary",
    "profileUrl",
    "ref",
    "referenceTypes",
    "region",
    "remove",
    "replace",
    "required",
    "Resources",
    "resourceType",
    "returned",
    "roles",
    "schema",
    "schemaExtensions",
    "schemas",
    "scimType",
    "sort",
    "sortBy",
    "sortOrder",
    "specUri",
    "startIndex",
    "status",
    "streetAddress",
    "subAttributes",
    "supported",
    "timezone",
    "title",
    "totalResults",
    "type",
    "uniqueness",
    "userName",
    "userType",
    "value",
    "version",
    "x509Certificates",
];

/// The schema URNs, because an extension appears as an object *key*
/// (`"urn:…:extension:enterprise:2.0:User": { … }`) and so is subject to the
/// same folding as any other attribute name.
const URNS: &[&str] = &[
    "urn:ietf:params:scim:schemas:core:2.0:User",
    "urn:ietf:params:scim:schemas:core:2.0:Group",
    "urn:ietf:params:scim:schemas:core:2.0:Schema",
    "urn:ietf:params:scim:schemas:core:2.0:ResourceType",
    "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
    "urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig",
    "urn:ietf:params:scim:api:messages:2.0:ListResponse",
    "urn:ietf:params:scim:api:messages:2.0:PatchOp",
    "urn:ietf:params:scim:api:messages:2.0:SearchRequest",
    "urn:ietf:params:scim:api:messages:2.0:Error",
    "urn:ietf:params:scim:api:messages:2.0:BulkRequest",
    "urn:ietf:params:scim:api:messages:2.0:BulkResponse",
];

fn table() -> &'static HashMap<String, &'static str> {
    static TABLE: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        WIRE_NAMES
            .iter()
            .chain(URNS.iter())
            .map(|n| (n.to_ascii_lowercase(), *n))
            .collect()
    })
}

/// Rewrite every object key in `value`, recursively, to its canonical
/// spelling where a case-insensitive match is known.
///
/// Keys with no known match are left exactly as they arrived, and values are
/// never touched — including the URN strings inside a `schemas` array, which
/// the crate compares exactly as RFC 7643 prints them. An object under an
/// extension URN this crate does not model is left byte-identical, subtree
/// included. Two keys in one object that fold to the same name are an
/// [`AmbiguousKey`] error rather than a silent overwrite.
pub fn canonicalize_keys(value: &mut Value) -> Result<(), AmbiguousKey> {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = std::mem::take(map).into_iter().collect();
            // canonical key -> the original spelling that claimed it
            let mut claimed: HashMap<String, String> = HashMap::with_capacity(entries.len());
            for (k, mut v) in entries {
                let lower = k.to_ascii_lowercase();
                let known = table().get(&lower).copied();
                // A URN key we do not model owns its subtree: leave it alone.
                let foreign_extension = lower.starts_with("urn:") && known.is_none();
                if !foreign_extension {
                    canonicalize_keys(&mut v)?;
                }
                let key = match known {
                    Some(canonical) => canonical.to_string(),
                    None => k.clone(),
                };
                if let Some(first) = claimed.insert(key.clone(), k.clone()) {
                    return Err(AmbiguousKey {
                        canonical: key,
                        first,
                        second: k,
                    });
                }
                map.insert(key, v);
            }
            Ok(())
        }
        Value::Array(items) => items.iter_mut().try_for_each(canonicalize_keys),
        _ => Ok(()),
    }
}

/// Deserialize `T` from JSON text, accepting attribute names in any case.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, serde_json::Error> {
    let mut v: Value = serde_json::from_str(s)?;
    canonicalize_keys(&mut v).map_err(serde::de::Error::custom)?;
    serde_json::from_value(v)
}

/// Deserialize `T` from a [`Value`], accepting attribute names in any case.
pub fn from_value<T: DeserializeOwned>(mut v: Value) -> Result<T, serde_json::Error> {
    canonicalize_keys(&mut v).map_err(serde::de::Error::custom)?;
    serde_json::from_value(v)
}

/// A `Deserialize` adapter that canonicalises attribute-name case before
/// deserializing `T`, so it composes with any `serde` front end:
/// `serde_json::from_str::<CaseInsensitive<User>>(..)?.into_inner()`.
///
/// It buffers the input as a [`Value`] first, which is the cost of doing this
/// without a derive; on a SCIM payload that is not measurable.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseInsensitive<T>(pub T);

impl<T> CaseInsensitive<T> {
    /// The deserialized value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'de, T: DeserializeOwned> serde::Deserialize<'de> for CaseInsensitive<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut v = Value::deserialize(deserializer)?;
        canonicalize_keys(&mut v).map_err(serde::de::Error::custom)?;
        serde_json::from_value(v)
            .map(CaseInsensitive)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// R2-C1: the table must be injective under case folding. Two entries that
    /// fold to the same key collapse in the `HashMap`, and the loser was the
    /// RFC's own spelling — `Resources` and `Operations` were being rewritten
    /// to lowercase, silently emptying every `ListResponse` page and making a
    /// PATCH body byte-identical to the RFC's example fail to parse.
    #[test]
    fn wire_name_table_is_injective_under_case_folding() {
        let mut seen = std::collections::HashMap::new();
        for n in WIRE_NAMES.iter().chain(URNS.iter()) {
            if let Some(prev) = seen.insert(n.to_ascii_lowercase(), *n) {
                panic!("{prev:?} and {n:?} fold to the same key");
            }
        }
        assert_eq!(table().len(), WIRE_NAMES.len() + URNS.len());
    }

    /// The two envelopes the collision hit, in every casing.
    #[test]
    fn protocol_envelope_members_keep_their_rfc_casing() {
        for spelling in ["Resources", "resources", "RESOURCES"] {
            let mut v = serde_json::json!({ spelling: [1] });
            canonicalize_keys(&mut v).unwrap();
            assert!(
                v.get("Resources").is_some(),
                "{spelling} must canonicalise to Resources"
            );
        }
        for spelling in ["Operations", "operations", "OPERATIONS"] {
            let mut v = serde_json::json!({ spelling: [1] });
            canonicalize_keys(&mut v).unwrap();
            assert!(
                v.get("Operations").is_some(),
                "{spelling} must canonicalise to Operations"
            );
        }
    }

    /// R2-H1: two spellings of one attribute in one object is an error, not a
    /// silent last-write-wins whose winner depends on byte order.
    #[test]
    fn colliding_keys_are_rejected() {
        for raw in [
            r#"{"userName":"alice","username":"admin"}"#,
            r#"{"userName":"alice","USERNAME":"admin"}"#,
            r#"{"emails":[{"value":"a","VALUE":"b"}]}"#,
        ] {
            let mut v: Value = serde_json::from_str(raw).unwrap();
            let err = canonicalize_keys(&mut v).expect_err(raw);
            assert_ne!(err.first, err.second);
            assert!(
                from_str::<crate::models::user::User<String>>(raw).is_err(),
                "the deserializing entry points must surface the collision"
            );
        }
    }

    /// R2-M2: a subtree under an extension URN this crate does not model is
    /// not RFC 7643 §2.1 territory and is left byte-identical. The extension
    /// this crate does model is canonicalised, because that vocabulary is ours.
    #[test]
    fn foreign_extension_subtrees_are_untouched_but_known_ones_are_folded() {
        let mut v = serde_json::json!({
            "urn:example:2.0:Thing": {"Value": "x", "TITLE": "y", "MyOwn": "z"},
            "URN:IETF:PARAMS:SCIM:SCHEMAS:EXTENSION:ENTERPRISE:2.0:USER": {"EMPLOYEENUMBER": "7"}
        });
        canonicalize_keys(&mut v).unwrap();
        let foreign = &v["urn:example:2.0:Thing"];
        assert_eq!(foreign["Value"], "x");
        assert_eq!(foreign["TITLE"], "y");
        assert_eq!(foreign["MyOwn"], "z");
        let ours = &v["urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"];
        assert_eq!(ours["employeeNumber"], "7");
    }

    #[test]
    fn canonicalizes_known_keys_and_leaves_unknown_ones() {
        let mut v = serde_json::json!({
            "SCHEMAS": ["URN:IETF:PARAMS:SCIM:SCHEMAS:CORE:2.0:USER"],
            "USERNAME": "bjensen",
            "Name": {"FAMILYNAME": "Jensen"},
            "emails": [{"VALUE": "a@example.com", "PRIMARY": true}],
            "x-vendor-extra": 1
        });
        canonicalize_keys(&mut v).unwrap();
        // Only *keys* are rewritten. The URNs inside the `schemas` array are
        // values, and values are never touched — RFC 8141 leaves the
        // case-sensitivity of a URN's namespace-specific string to the
        // namespace, and the crate compares URNs exactly, as the RFC prints them.
        assert_eq!(
            v["schemas"][0],
            "URN:IETF:PARAMS:SCIM:SCHEMAS:CORE:2.0:USER"
        );
        assert_eq!(v["userName"], "bjensen");
        assert_eq!(v["name"]["familyName"], "Jensen");
        assert_eq!(v["emails"][0]["value"], "a@example.com");
        assert_eq!(v["emails"][0]["primary"], true);
        assert_eq!(
            v["x-vendor-extra"], 1,
            "unknown keys must pass through untouched"
        );
    }

    /// The probe that motivated the module: a fully upper-cased User fails the
    /// plain derive with `missing field userName` and parses here.
    #[test]
    fn an_upper_cased_user_parses() {
        let raw = r#"{"SCHEMAS":["urn:ietf:params:scim:schemas:core:2.0:User"],"USERNAME":"bjensen","DISPLAYNAME":"B"}"#;
        assert!(serde_json::from_str::<crate::models::user::User<String>>(raw).is_err());
        let u: crate::models::user::User<String> =
            from_str(raw).expect("§2.1: names are case-insensitive");
        assert_eq!(u.user_name, "bjensen");
        assert_eq!(u.display_name.as_deref(), Some("B"));
        let via_adapter: CaseInsensitive<crate::models::user::User<String>> =
            serde_json::from_str(raw).unwrap();
        assert_eq!(via_adapter.into_inner(), u);
    }

    /// The table must stay complete against the RFC schemas this crate ships.
    #[cfg(feature = "schemas")]
    #[test]
    fn wire_names_cover_every_embedded_schema_attribute() {
        for raw in [
            crate::USER_SCHEMA,
            crate::GROUP_SCHEMA,
            crate::ENTERPRISE_USER_SCHEMA,
        ] {
            let schema: Value = serde_json::from_str(raw).unwrap();
            fn walk(attrs: &Value, missing: &mut Vec<String>) {
                for a in attrs.as_array().into_iter().flatten() {
                    let name = a["name"].as_str().unwrap();
                    if !WIRE_NAMES.contains(&name) {
                        missing.push(name.to_string());
                    }
                    walk(&a["subAttributes"], missing);
                }
            }
            let mut missing = Vec::new();
            walk(&schema["attributes"], &mut missing);
            assert!(missing.is_empty(), "WIRE_NAMES is missing: {missing:?}");
        }
    }
}
