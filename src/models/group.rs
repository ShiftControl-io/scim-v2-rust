//Schema for group
use crate::models::scim_schema::Meta;
use crate::utils::validation::{Context, Validate, ValidationError, require_schema_urn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Group<T = String> {
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub display_name: String,
    #[serde(
        default = "Vec::new",
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec"
    )]
    pub members: Vec<Member<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// RFC 7643 §4.2.1 `members.type`.
///
/// `#[non_exhaustive]`, with an [`Other`](MemberType::Other) catch-all.
/// §7 defines `canonicalValues` as "a collection of **suggested** canonical
/// values that **MAY** be used", and the Group schema describes this
/// sub-attribute as "the type of resource, e.g., 'User' or 'Group'". So a
/// provider may legitimately send another label, and before 1.0 that failed
/// deserialization of the entire enclosing payload. Unknown labels now land in
/// `Other` and round-trip unchanged.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "String", into = "String")]
pub enum MemberType {
    User,
    Group,
    /// A label outside the RFC's suggested set, preserved verbatim.
    Other(String),
}

impl MemberType {
    /// The canonical wire label, which is also the comparison key.
    fn label(&self) -> &str {
        match self {
            MemberType::User => "User",
            MemberType::Group => "Group",
            MemberType::Other(s) => s,
        }
    }
}

/// Case-insensitive, matching the schema's `caseExact: false` for this
/// sub-attribute and the folding [`From<String>`] already applies to the two
/// suggested labels.
///
/// The derive would have compared `Other` byte-exact, so
/// `Other("serviceaccount") != Other("ServiceAccount")` and, worse,
/// `Other("User") != MemberType::User` even though both serialize to `"User"`
/// — a hand-constructed value could fail `== Some(MemberType::User)` while
/// being wire-identical to one that matches.
impl PartialEq for MemberType {
    fn eq(&self, other: &Self) -> bool {
        self.label().eq_ignore_ascii_case(other.label())
    }
}

impl Eq for MemberType {}

/// Consistent with [`PartialEq`]: equal values must hash equally, so the key
/// is the lowercased label.
impl std::hash::Hash for MemberType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label().to_ascii_lowercase().hash(state);
    }
}

impl From<String> for MemberType {
    fn from(s: String) -> Self {
        // §2.2: `caseExact` defaults to false, so the comparison is
        // case-insensitive; the original spelling survives in `Other`.
        if s.eq_ignore_ascii_case("user") {
            MemberType::User
        } else if s.eq_ignore_ascii_case("group") {
            MemberType::Group
        } else {
            MemberType::Other(s)
        }
    }
}

impl From<MemberType> for String {
    fn from(m: MemberType) -> Self {
        match m {
            MemberType::User => "User".to_string(),
            MemberType::Group => "Group".to_string(),
            MemberType::Other(s) => s,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Member<T = String> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MemberType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl<T> Validate for Group<T> {
    /// RFC 7643 §4.2 marks `displayName` REQUIRED; §3 marks `schemas` REQUIRED
    /// on every resource.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, crate::schema_urns::GROUP)?;
        if self.display_name.is_empty() {
            return Err(ValidationError::missing_required("displayName"));
        }
        Ok(())
    }

    /// RFC 7643 §3.1: `id` REQUIRED on a response, forbidden on a create.
    fn validate_context(&self, ctx: Context) -> Result<(), ValidationError> {
        match ctx {
            Context::CreateRequest if self.id.is_some() => Err(ValidationError::invalid_value(
                "id",
                "MUST NOT be specified by the client on create (RFC 7643 §3.1)",
            )),
            Context::Response if self.id.is_none() => Err(ValidationError::missing_required("id")),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;
