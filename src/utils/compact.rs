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

mod sealed {
    pub trait Sealed {}
}

/// The types [`Compact`] accepts: the representations a server or client
/// emits, never a request whose empty arrays carry RFC 7644 §3.5.1 clear-all
/// meaning. Sealed — the set is this crate's to define — and implemented for
/// `User`, `Group`, `EnterpriseUser`, `Schema`, `ResourceType`,
/// `ServiceProviderConfig`, `Resource` and `ListResponse`. `PatchOp` and
/// `SearchRequest` are deliberately absent; see [`Compact`].
pub trait Compactable: Serialize + sealed::Sealed {}

#[cfg(feature = "models")]
mod impls {
    use super::{Compactable, sealed::Sealed};
    use crate::models::enterprise_user::EnterpriseUser;
    use crate::models::group::Group;
    use crate::models::others::{ListResponse, Resource, ScimResource};
    use crate::models::resource_types::ResourceType;
    use crate::models::scim_schema::Schema;
    use crate::models::service_provider_config::ServiceProviderConfig;
    use crate::models::user::User;
    use serde::Serialize;

    impl<T: Serialize> Sealed for User<T> {}
    impl<T: Serialize> Compactable for User<T> {}
    impl<T: Serialize> Sealed for Group<T> {}
    impl<T: Serialize> Compactable for Group<T> {}
    impl Sealed for EnterpriseUser {}
    impl Compactable for EnterpriseUser {}
    impl Sealed for Schema {}
    impl Compactable for Schema {}
    impl Sealed for ResourceType {}
    impl Compactable for ResourceType {}
    impl Sealed for ServiceProviderConfig {}
    impl Compactable for ServiceProviderConfig {}
    impl<T: Serialize> Sealed for Resource<T> {}
    impl<T: Serialize> Compactable for Resource<T> {}
    impl<R: ScimResource + Serialize> Sealed for ListResponse<R> {}
    impl<R: ScimResource + Serialize> Compactable for ListResponse<R> {}
}

/// Remove every `null` and every empty array from `value`, recursively.
///
/// Both are "unassigned" under RFC 7643 §2.5. The crate's serializers never
/// emit `null` for an unset `Option`, so in practice this removes `[]`.
/// Crate-private: applied to a request body it would delete an RFC 7644
/// §3.5.1 clear-all, which is what the [`Compactable`] bound on [`Compact`]
/// exists to prevent.
pub(crate) fn strip_unassigned(value: &mut Value) {
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
/// including protocol arrays such as a `ListResponse`'s `Resources`, and
/// arrays inside extension payloads this crate does not model. RFC 7643 §2.5
/// applies to every multi-valued attribute, extensions included, and for the
/// types this accepts no conformant *representation* is altered by it:
/// `Resources` is "REQUIRED if totalResults is non-zero" (RFC 7644 §3.4.2),
/// so an empty page may omit it; `schemas` is REQUIRED non-empty, so an
/// empty one was already invalid. Non-empty arrays are never touched.
///
/// # Not for request bodies
///
/// RFC 7644 §3.5.1 gives `[]` in a request the meaning "clear all values",
/// which stripping would silently delete: `{"op":"replace","path":"emails",
/// "value":[]}` would go out as `{"op":"replace","path":"emails"}`, and an
/// `add` carrying `{"members":[],"userName":"x"}` would lose the clear and
/// keep the set. The [`Compactable`] bound therefore excludes `PatchOp` and
/// `SearchRequest` outright — `Compact(&patch_op)` does not compile — and the
/// resource types it does accept are meant for the representations a server
/// *returns*. Do not compact a resource you are about to send as a `PUT`
/// body, for the same reason.
///
/// ```compile_fail
/// use scim_v2::{models::others::PatchOp, utils::compact::Compact};
///
/// fn emit(op: &PatchOp) -> String {
///     serde_json::to_string(&Compact(op)).unwrap()
/// }
/// ```
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
pub struct Compact<'a, T: Compactable>(pub &'a T);

impl<T: Compactable> Serialize for Compact<'_, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut v = serde_json::to_value(self.0).map_err(serde::ser::Error::custom)?;
        strip_unassigned(&mut v);
        v.serialize(serializer)
    }
}

#[cfg(test)]
mod tests;
