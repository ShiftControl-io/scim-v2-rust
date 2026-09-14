//! This module makes attribute names case-insensitive, per RFC 7643 §2.1.
//!
//! RFC 7643 §2.1 states: "Attribute names are case insensitive and are
//! often camel-cased". A conformant peer may therefore send `"USERNAME"`
//! or `"displayname"`. A `serde` derive that matches exact strings drops
//! such a key. The derive then fails on the missing required field. This
//! module rewrites every known attribute name, sub-attribute name,
//! protocol member (`Resources`, `Operations`) and schema URN to its
//! canonical spelling before deserialization.
//!
//! Three rules prevent data loss here. This module leaves a key with no
//! case-insensitive match exactly as it arrived. This module leaves a
//! subtree byte-identical under an extension URN that this crate does not
//! model (`urn:example:…`). RFC 7643 §2.1 does not govern that namespace.
//! Its vendor may well be case-sensitive. Two keys that fold to the same
//! attribute are an error, not a silent last-write-wins. For example,
//! `userName` and `USERNAME` in one object fold to the same attribute.
//! RFC 7643 §2.1 makes them the same attribute asserted twice. Neither
//! RFC says which value wins.
//!
//! The Java SCIM SDK does the same. It enables Jackson's
//! `ACCEPT_CASE_INSENSITIVE_PROPERTIES`. The scim2-models library
//! lowercases every key in a pre-validator that cites §2.1.
//!
//! # Choosing it
//!
//! Nothing here happens implicitly. The derives stay exact-match. You
//! choose the behavior at each call site, as this example shows:
//!
//! ```
//! # #[cfg(feature = "models")] {
//! use scim_v2::models::user::User;
//!
//! let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"USERNAME":"bjensen"}"#;
//!
//! // Exact-match, the serde default: `USERNAME` is not `userName`.
//! assert!(serde_json::from_str::<User>(body).is_err());
//!
//! // Canonicalised first, per §2.1.
//! let user: User = scim_v2::case_insensitive::from_str(body).unwrap();
//! assert_eq!(user.user_name, "bjensen");
//! # }
//! ```
//!
//! [`CaseInsensitive<T>`] is the primitive type. Use it directly in a
//! function signature. A `serde` front end, such as an `axum` extractor,
//! can name it directly. This crate re-exports it as
//! [`scim_v2::CaseInsensitive`]. [`from_str()`] and [`from_value()`] are
//! shorthand functions. They wrap and then unwrap [`CaseInsensitive<T>`]
//! for you. Use them for the common case where you already have the
//! bytes.
//!
//! [`CaseInsensitive<T>`]: CaseInsensitive
//! [`scim_v2::CaseInsensitive`]: crate::CaseInsensitive

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

/// Two keys in one object fold to the same attribute name.
///
/// RFC 7643 §2.1 makes `userName` and `USERNAME` the same attribute. A
/// payload that carries both has asserted one attribute twice, possibly
/// with different values. Neither RFC defines a precedence. A silent
/// resolution would let whichever spelling sorts later decide what a
/// handler sees. This behavior would be an attribute-smuggling vector at
/// a request boundary. This crate therefore refuses the body instead
/// (RFC 7644 §3.12 `invalidSyntax`).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "attribute {canonical:?} is given more than once with differing case ({first:?} and {second:?}); RFC 7643 §2.1 makes these the same attribute"
)]
pub struct AmbiguousKey {
    /// The canonical spelling both keys fold to.
    pub canonical: String,
    /// The spelling this crate saw first.
    pub first: String,
    /// The spelling this crate saw second.
    pub second: String,
}

/// This constant lists every canonical wire name that this crate models.
/// This crate derives the list from the embedded RFC 7643 schemas plus
/// the protocol messages. A test asserts that the list stays complete
/// against those schemas.
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

/// This constant lists the schema URNs. An extension appears as an
/// object *key* (`"urn:…:extension:enterprise:2.0:User": { … }`). This
/// module therefore folds a URN the same way it folds any other
/// attribute name.
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
/// This function leaves a key with no known match exactly as it arrived.
/// This function never touches a value. This includes the URN strings
/// inside a `schemas` array. This crate compares those URN strings
/// exactly as RFC 7643 prints them. This function leaves an object
/// byte-identical, subtree included, under an extension URN that this
/// crate does not model. Two keys in one object that fold to the same
/// name are an [`AmbiguousKey`] error, not a silent overwrite.
///
/// On `Err`, `value` still holds exactly what the caller passed in. The
/// collision check runs over the whole tree before this function
/// rewrites any key. A caller that logs, echoes or retries the rejected
/// body therefore sees both spellings. Such a caller never sees a
/// half-rebuilt object that asserts only the attacker's spelling.
pub fn canonicalize_keys(value: &mut Value) -> Result<(), AmbiguousKey> {
    check_collisions(value)?;
    rewrite_keys(value);
    Ok(())
}

/// This function is phase one. It is read-only. This function finds the
/// first pair of keys in one object that fold to the same canonical name.
/// This search covers the whole tree that the rewrite would visit.
fn check_collisions(value: &Value) -> Result<(), AmbiguousKey> {
    match value {
        Value::Object(map) => {
            // canonical key -> the original spelling that claimed it
            let mut claimed: HashMap<String, &str> = HashMap::with_capacity(map.len());
            for (k, v) in map {
                let lower = k.to_ascii_lowercase();
                let known = table().get(&lower).copied();
                let canonical = known.unwrap_or(k.as_str());
                if let Some(first) = claimed.insert(canonical.to_string(), k.as_str()) {
                    return Err(AmbiguousKey {
                        canonical: canonical.to_string(),
                        first: first.to_string(),
                        second: k.clone(),
                    });
                }
                // A URN key we do not model owns its subtree: leave it alone.
                if !(lower.starts_with("urn:") && known.is_none()) {
                    check_collisions(v)?;
                }
            }
            Ok(())
        }
        Value::Array(items) => items.iter().try_for_each(check_collisions),
        _ => Ok(()),
    }
}

/// This function is phase two, the rewrite. It cannot fail once phase
/// one has passed.
fn rewrite_keys(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = std::mem::take(map).into_iter().collect();
            for (k, mut v) in entries {
                let lower = k.to_ascii_lowercase();
                let known = table().get(&lower).copied();
                if !(lower.starts_with("urn:") && known.is_none()) {
                    rewrite_keys(&mut v);
                }
                map.insert(known.map_or(k, str::to_string), v);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_keys),
        _ => {}
    }
}

/// Deserialize `T` from JSON text. Accept an attribute name in any case.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, serde_json::Error> {
    let mut v: Value = serde_json::from_str(s)?;
    canonicalize_keys(&mut v).map_err(serde::de::Error::custom)?;
    serde_json::from_value(v)
}

/// Deserialize `T` from a [`Value`]. Accept an attribute name in any case.
pub fn from_value<T: DeserializeOwned>(mut v: Value) -> Result<T, serde_json::Error> {
    canonicalize_keys(&mut v).map_err(serde::de::Error::custom)?;
    serde_json::from_value(v)
}

/// This struct is a `Deserialize` adapter. It canonicalizes
/// attribute-name case before it deserializes `T`. It therefore composes
/// with any `serde` front end, for example
/// `serde_json::from_str::<CaseInsensitive<User>>(..)?.into_inner()`.
///
/// It buffers the input as a [`Value`] first. This buffering is the cost
/// of the adapter. The adapter uses no derive macro. On a SCIM payload,
/// this cost is not measurable.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseInsensitive<T>(pub T);

impl<T> CaseInsensitive<T> {
    /// This method returns the deserialized value.
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
