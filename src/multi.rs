//! A multi-valued attribute that remembers whether the wire asserted it.
//!
//! RFC 7643 §2.5 makes three wire forms equivalent in a resource: an absent
//! member, `null`, and `[]`. "Unassigned attributes, the null value, or an
//! empty array (in the case of a multi-valued attribute) SHALL be considered
//! to be equivalent in "state"."
//!
//! A request is different. RFC 7644 §3.5.1 says an omitted readWrite
//! attribute "MAY be assumed to be not asserted by the client", and the
//! service provider "MAY assume that any existing values are to be cleared,
//! or the service provider MAY assign a default value". The same section
//! gives the client the other choice: "Clients that want to override a
//! server's defaults MAY specify "null" for a single-valued attribute, or an
//! empty array "[]" for a multi-valued attribute, to clear all values."
//!
//! So a request has two instructions, not three. An absent member asserts
//! nothing. A `null` and an `[]` both clear the values. A `Vec<T>` holds only
//! one of the two, which makes a resource that was read and then written
//! different from the one that arrived.
//!
//! [`Multi`] holds both. It derefs to `[T]`, so a reader that does not care
//! writes the same code as before.
//!
//! ```
//! # #[cfg(feature = "models")] {
//! use scim_v2::Multi;
//!
//! let absent: Multi<String> = Multi::absent();
//! let cleared: Multi<String> = Multi::cleared();
//!
//! assert!(absent.is_empty() && cleared.is_empty()); // both read as empty
//! assert!(!absent.is_asserted()); // but the wire said different things
//! assert!(cleared.is_asserted());
//! # }
//! ```

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A SCIM multi-valued attribute.
///
/// Three states, which are the two RFC 7644 §3.5.1 instructions plus the
/// values themselves: **absent** (the member was not on the wire, so the
/// client asserted nothing), **cleared** (the member was `null` or `[]`, so
/// the client asked to clear all values), and **set** (the member carried
/// values).
///
/// Put it on a model field with `default` and `skip_serializing_if`, and the
/// three wire forms survive a read followed by a write:
///
/// ```ignore
/// #[serde(default, skip_serializing_if = "Multi::is_absent")]
/// pub emails: Multi<Email>,
/// ```
///
/// [`Deref`] gives every read a `Vec` has: `len`, `is_empty`, `iter`,
/// `first`, indexing, and `for value in &multi`. A read therefore treats
/// absent and cleared the same, which RFC 7643 §2.5 says is correct for a
/// resource. Ask [`is_asserted`](Self::is_asserted) when the difference
/// matters, which is a `PUT` handler and little else.
///
/// `PartialEq` compares the state, so `Multi::absent() != Multi::cleared()`.
///
/// ```
/// use scim_v2::Multi;
///
/// let mut roles: Multi<String> = Multi::absent();
/// assert!(!roles.is_asserted());
///
/// roles.push("admin".to_string()); // a write asserts the attribute
/// assert!(roles.is_asserted());
/// assert_eq!(roles.len(), 1);
/// assert_eq!(&roles[0], "admin");
///
/// roles.clear(); // still asserted: this is "clear all values"
/// assert!(roles.is_asserted());
/// assert!(roles.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Multi<T>(Option<Vec<T>>);

impl<T> Multi<T> {
    /// The member was not on the wire. The client asserted nothing.
    pub const fn absent() -> Self {
        Self(None)
    }

    /// The member was on the wire and empty. The client asked to clear all
    /// values.
    pub const fn cleared() -> Self {
        Self(Some(Vec::new()))
    }

    /// Whether the wire carried the member at all.
    ///
    /// RFC 7644 §3.5.1: an omitted readWrite attribute "MAY be assumed to be
    /// not asserted by the client". A `PUT` handler applies the attribute
    /// when this is `true` and leaves the stored value alone when it is
    /// `false`.
    pub const fn is_asserted(&self) -> bool {
        self.0.is_some()
    }

    /// The negation of [`is_asserted`](Self::is_asserted), for
    /// `skip_serializing_if`.
    pub const fn is_absent(&self) -> bool {
        self.0.is_none()
    }

    /// The values, as a slice. Absent and cleared both give `&[]`.
    pub fn as_slice(&self) -> &[T] {
        self.0.as_deref().unwrap_or_default()
    }

    /// The values, dropping the distinction between absent and cleared.
    pub fn into_vec(self) -> Vec<T> {
        self.0.unwrap_or_default()
    }

    /// The state itself: `None` for absent, `Some` for asserted.
    pub fn into_inner(self) -> Option<Vec<T>> {
        self.0
    }

    /// The values for mutation. Asserts the attribute if it was absent.
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        self.0.get_or_insert_with(Vec::new)
    }

    /// Append a value. Asserts the attribute.
    pub fn push(&mut self, value: T) {
        self.as_mut_vec().push(value);
    }

    /// Remove every value and keep the attribute asserted, which is the
    /// RFC 7644 §3.5.1 "clear all values" instruction.
    ///
    /// Use [`unset`](Self::unset) to stop asserting the attribute instead.
    pub fn clear(&mut self) {
        self.0 = Some(Vec::new());
    }

    /// Stop asserting the attribute. The member leaves the wire.
    pub fn unset(&mut self) {
        self.0 = None;
    }

    /// Remove and return the last value. An absent attribute gives `None` and
    /// stays absent, because a removal that removes nothing asserts nothing.
    pub fn pop(&mut self) -> Option<T> {
        self.0.as_mut().and_then(Vec::pop)
    }

    /// Keep the values that satisfy `f`. An absent attribute stays absent,
    /// for the same reason as [`pop`](Self::pop).
    pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
        if let Some(values) = self.0.as_mut() {
            values.retain(f);
        }
    }
}

impl<T> Default for Multi<T> {
    /// Absent. A model built with `..Default::default()` asserts nothing.
    fn default() -> Self {
        Self::absent()
    }
}

impl<T> Deref for Multi<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Multi<T> {
    /// The values for in-place mutation. An element cannot be added or
    /// removed through a slice, so this cannot change whether the attribute
    /// is asserted.
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.as_deref_mut().unwrap_or_default()
    }
}

impl<T> Extend<T> for Multi<T> {
    /// Asserts the attribute, even when the iterator is empty.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.as_mut_vec().extend(iter);
    }
}

impl<T> AsRef<[T]> for Multi<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> From<Vec<T>> for Multi<T> {
    /// Asserted, including an empty `Vec`, which is the clear-all form.
    fn from(values: Vec<T>) -> Self {
        Self(Some(values))
    }
}

impl<T> From<Option<Vec<T>>> for Multi<T> {
    fn from(values: Option<Vec<T>>) -> Self {
        Self(values)
    }
}

impl<T> FromIterator<T> for Multi<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Some(iter.into_iter().collect()))
    }
}

impl<T> IntoIterator for Multi<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Multi<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T: Serialize> Serialize for Multi<T> {
    /// Always an array. An absent attribute leaves the wire through the
    /// field's `skip_serializing_if`, so a value that reaches here is written
    /// as `[]` rather than as `null`: RFC 7643 §2.5 makes the two equivalent,
    /// and this crate never emits `null`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Multi<T> {
    /// Anything that reaches here was on the wire, so the result is always
    /// asserted. `null` and `[]` both give the cleared state, which RFC 7644
    /// §3.5.1 makes one instruction. An absent member never reaches here: the
    /// field's `default` gives [`Multi::absent`] instead.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Some(
            Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default(),
        )))
    }
}

#[cfg(test)]
mod tests;
