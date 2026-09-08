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
mod tests;
