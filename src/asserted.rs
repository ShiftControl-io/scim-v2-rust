//! An attribute, and whether the wire asserted it.
//!
//! RFC 7643 §2.5 makes three wire forms equivalent in a resource: an absent
//! member, `null`, and `[]` for a multi-valued attribute. "Unassigned
//! attributes, the null value, or an empty array (in the case of a
//! multi-valued attribute) SHALL be considered to be equivalent in "state"."
//!
//! A request is not a resource. RFC 7644 §3.5.1 says an omitted readWrite
//! attribute "MAY be assumed to be not asserted by the client", and the
//! service provider "MAY assume that any existing values are to be cleared,
//! or the service provider MAY assign a default value". The same section
//! gives the client the deterministic form: clients "MAY specify "null" for a
//! single-valued attribute, or an empty array "[]" for a multi-valued
//! attribute, to clear all values".
//!
//! So a client says one of three things about an attribute. It says nothing.
//! It says clear the value. It says use this value. An `Option<T>` holds two
//! of the three, so a server or a proxy cannot tell silence from an
//! instruction to clear.
//!
//! [`Asserted`] holds all three, for a single-valued attribute and a
//! multi-valued one alike. The question is the same in both cases, so one
//! type answers it.
//!
//! ```
//! # #[cfg(feature = "models")] {
//! use scim_v2::Asserted;
//!
//! let silence: Asserted<String> = Asserted::absent();
//! let clear: Asserted<String> = Asserted::nulled();
//! let value = Asserted::set("Tour Guide".to_string());
//!
//! assert!(!silence.is_asserted()); // the client said nothing
//! assert!(clear.is_asserted()); // the client said something
//! assert_eq!(value.as_option(), Some(&"Tour Guide".to_string()));
//! # }
//! ```

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// What a wire message said about one attribute.
///
/// Three states, which are the three things a client can say under RFC 7644
/// §3.5.1: [`Absent`](Self::Absent) for a member that was not there,
/// [`Nulled`](Self::Nulled) for a `null` or an `[]`, and
/// [`Set`](Self::Set) for a value.
///
/// Put it on a field with the same two attributes every time, whatever the
/// field holds:
///
/// ```ignore
/// #[serde(default = "Asserted::absent", skip_serializing_if = "Asserted::is_absent")]
/// pub title: Asserted<String>,
///
/// #[serde(default = "Asserted::absent", skip_serializing_if = "Asserted::is_absent")]
/// pub emails: Asserted<Vec<Email>>,
/// ```
///
/// Each state writes back the form it came from. `Absent` leaves the member
/// off the wire. `Nulled` writes `null`. `Set` writes the value, and an
/// `Asserted::set(Vec::new())` therefore writes `[]`. A message that arrives
/// and leaves unchanged is unchanged, and a caller who prefers a different
/// form says so: read a `null` and send nothing by assigning
/// [`Asserted::Absent`](Self::Absent).
///
/// A multi-valued attribute also derefs to `[T]`, so `len`, `iter`, `first`,
/// indexing and `for value in &attribute` read as they would on a `Vec`.
///
/// ```
/// # #[cfg(feature = "models")] {
/// use scim_v2::Asserted;
///
/// let mut roles: Asserted<Vec<String>> = Asserted::absent();
/// assert!(roles.is_empty() && !roles.is_asserted());
///
/// roles.push("admin".to_string()); // a write asserts the attribute
/// assert_eq!(&roles[0], "admin");
///
/// roles = Asserted::set(Vec::new()); // RFC 7644 §3.5.1 "to clear all values"
/// assert!(roles.clears_values());
///
/// roles = Asserted::absent(); // say nothing at all
/// assert!(!roles.clears_values());
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Asserted<T> {
    /// The member was not on the wire. RFC 7644 §3.5.1: "not asserted by the
    /// client".
    Absent,
    /// The member was `null`, or `[]` for a multi-valued attribute. RFC 7644
    /// §3.5.1: the client asked "to clear all values".
    Nulled,
    /// The member carried a value.
    Set(T),
}

impl<T> Asserted<T> {
    /// The member was not on the wire.
    pub const fn absent() -> Self {
        Self::Absent
    }

    /// The member arrived as `null`. RFC 7644 §3.5.1 makes this the client's
    /// way to clear a single-valued attribute.
    pub const fn nulled() -> Self {
        Self::Nulled
    }

    /// The client sent this value.
    pub const fn set(value: T) -> Self {
        Self::Set(value)
    }

    /// Whether the wire carried the member at all.
    ///
    /// A `PUT` handler applies the attribute when this is `true`, and leaves
    /// the stored value alone when it is `false`.
    pub const fn is_asserted(&self) -> bool {
        !self.is_absent()
    }

    /// The negation of [`is_asserted`](Self::is_asserted), for
    /// `skip_serializing_if`.
    pub const fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }

    /// Whether the member arrived as `null`.
    ///
    /// For a single-valued attribute this is the whole RFC 7644 §3.5.1 clear
    /// question, because clients "MAY specify "null" for a single-valued
    /// attribute ... to clear all values". A multi-valued attribute has a
    /// second form, so ask
    /// [`clears_values`](Asserted::clears_values) there instead.
    pub const fn is_nulled(&self) -> bool {
        matches!(self, Self::Nulled)
    }

    /// The value, if the client sent one. `Absent` and `Nulled` both give
    /// `None`, which RFC 7643 §2.5 makes correct for a resource.
    pub const fn as_option(&self) -> Option<&T> {
        match self {
            Self::Set(value) => Some(value),
            _ => None,
        }
    }

    /// The value for mutation, if the client sent one.
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Set(value) => Some(value),
            _ => None,
        }
    }

    /// The value, dropping the difference between absent and cleared.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Set(value) => Some(value),
            _ => None,
        }
    }

    /// Apply a function to the value, if there is one, and keep the state.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Asserted<U> {
        match self {
            Self::Set(value) => Asserted::Set(f(value)),
            Self::Nulled => Asserted::Nulled,
            Self::Absent => Asserted::Absent,
        }
    }
}

impl<T> Asserted<Vec<T>> {
    /// Whether the client asked to clear all values, counting both forms
    /// RFC 7644 §3.5.1 names: a `null`, and an empty array.
    pub fn clears_values(&self) -> bool {
        match self {
            Self::Nulled => true,
            Self::Set(values) => values.is_empty(),
            Self::Absent => false,
        }
    }

    /// The values. Absent and nulled both give `&[]`.
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Set(values) => values,
            _ => &[],
        }
    }

    /// The values for mutation. Asserts the attribute when it was not set.
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        if !matches!(self, Self::Set(_)) {
            *self = Self::Set(Vec::new());
        }
        match self {
            Self::Set(values) => values,
            _ => unreachable!("just set"),
        }
    }

    /// Append a value. Asserts the attribute.
    pub fn push(&mut self, value: T) {
        self.as_mut_vec().push(value);
    }

    /// Remove and return the last value. An attribute that holds no values
    /// keeps its state, because a removal that removes nothing asserts
    /// nothing.
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Set(values) => values.pop(),
            _ => None,
        }
    }

    /// Keep the values that satisfy `f`. Keeps the state, as
    /// [`pop`](Self::pop) does.
    pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
        if let Self::Set(values) = self {
            values.retain(f);
        }
    }
}

impl<T> Default for Asserted<T> {
    /// Absent. A model built with `..Default::default()` asserts nothing.
    fn default() -> Self {
        Self::Absent
    }
}

impl<T> Deref for Asserted<Vec<T>> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Asserted<Vec<T>> {
    /// The values for in-place mutation. A slice cannot gain or lose an
    /// element, so this cannot change whether the attribute is asserted.
    fn deref_mut(&mut self) -> &mut [T] {
        match self {
            Self::Set(values) => values,
            _ => &mut [],
        }
    }
}

impl<T> Extend<T> for Asserted<Vec<T>> {
    /// Asserts the attribute, even when the iterator is empty.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.as_mut_vec().extend(iter);
    }
}

impl<T> FromIterator<T> for Asserted<Vec<T>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Set(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for Asserted<Vec<T>> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Set(values) => values.into_iter(),
            _ => Vec::new().into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Asserted<Vec<T>> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T> From<T> for Asserted<T> {
    /// A value the caller supplies is a value the caller asserts.
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

impl<T: Serialize> Serialize for Asserted<T> {
    /// `Set` writes its value and `Nulled` writes `null`. `Absent` also
    /// writes `null` here, because the field's `skip_serializing_if` is what
    /// removes the member, and a serializer must write something.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Set(value) => value.serialize(serializer),
            _ => serializer.serialize_none(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Asserted<T> {
    /// Anything that reaches here was on the wire, so the result is never
    /// [`Absent`](Self::Absent). A `null` gives [`Nulled`](Self::Nulled).
    /// An absent member never reaches here: the field's `default` supplies
    /// [`Asserted::absent`] instead.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(value) => Self::Set(value),
            None => Self::Nulled,
        })
    }
}

#[cfg(test)]
mod tests;
