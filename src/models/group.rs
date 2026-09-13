//Schema for group
use crate::models::scim_schema::Meta;
use crate::multi::Multi;
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
    #[serde(default = "Multi::absent", skip_serializing_if = "Multi::is_absent")]
    pub members: Multi<Member<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// This enum models the `members.type` sub-attribute from RFC 7643 §4.2.1.
///
/// This enum carries the `#[non_exhaustive]` attribute and the
/// [`Other`](MemberType::Other) catch-all variant. §7 defines
/// `canonicalValues` as "a collection of **suggested** canonical values
/// that **MAY** be used". The Group schema describes this sub-attribute as
/// "the type of resource, e.g., 'User' or 'Group'". A provider may
/// therefore legitimately send another label. Before version 1.0, an
/// unrecognized label failed deserialization of the entire enclosing
/// payload. Unknown labels now land in `Other` and round-trip unchanged.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "String", into = "String")]
pub enum MemberType {
    User,
    Group,
    /// This variant holds a label outside the RFC's suggested set. The
    /// crate keeps the label's text exactly as received.
    Other(String),
}

impl MemberType {
    /// This method returns the canonical wire label. The crate also uses
    /// this label as the comparison key.
    fn label(&self) -> &str {
        match self {
            MemberType::User => "User",
            MemberType::Group => "Group",
            MemberType::Other(s) => s,
        }
    }
}

/// This implementation compares values without regard to case. The schema
/// sets `caseExact: false` for this sub-attribute, so a case-insensitive
/// comparison matches the schema. The [`From<String>`] implementation
/// already folds case for the two suggested labels. This implementation
/// extends that same folding to equality comparison.
///
/// A derived `PartialEq` would compare `Other` byte for byte.
/// `Other("serviceaccount")` would then not equal
/// `Other("ServiceAccount")`. Worse, `Other("User")` would not equal
/// `MemberType::User`, even though both serialize to `"User"`. A
/// hand-constructed value could therefore fail `== Some(MemberType::User)`
/// even though it is wire-identical to a value that matches.
impl PartialEq for MemberType {
    fn eq(&self, other: &Self) -> bool {
        self.label().eq_ignore_ascii_case(other.label())
    }
}

impl Eq for MemberType {}

/// This implementation stays consistent with [`PartialEq`]. Equal values
/// must hash equally. The hash key is therefore the lowercased label.
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

/// This impl requires `T: Display` for the same reason as
/// [`User`](crate::models::user::User). Validation checks a response's `id`
/// against RFC 7643 §3.1's "non-empty" requirement.
impl<T: std::fmt::Display> Validate for Group<T> {
    /// RFC 7643 §4.2 marks `displayName` REQUIRED. RFC 7643 §3 marks
    /// `schemas` REQUIRED on every resource.
    fn validate(&self) -> Result<(), ValidationError> {
        require_schema_urn(&self.schemas, crate::schema_urns::GROUP)?;
        if self.display_name.is_empty() {
            return Err(ValidationError::missing_required("displayName"));
        }
        Ok(())
    }

    /// RFC 7643 §3.1 marks `id` REQUIRED on a response. RFC 7643 §3.1
    /// forbids `id` on a create request.
    fn validate_context(&self, ctx: Context) -> Result<(), ValidationError> {
        match ctx {
            Context::CreateRequest if self.id.is_some() => Err(ValidationError::invalid_value(
                "id",
                "MUST NOT be specified by the client on create (RFC 7643 §3.1)",
            )),
            Context::Response if self.id.as_ref().is_none_or(|id| id.to_string().is_empty()) => {
                Err(ValidationError::missing_required("id"))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;
