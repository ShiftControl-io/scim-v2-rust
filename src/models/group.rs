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
        deserialize_with = "crate::utils::serde::deserialize_null_as_empty_vec",
        skip_serializing_if = "crate::utils::serde::skip_multi_valued"
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
mod tests {

    /// RFC 7643 §7 makes `canonicalValues` *suggestions*, so a provider may
    /// send a `members.type` outside {User, Group}. Before 1.0 that failed the
    /// whole Group payload; it now lands in `Other` and round-trips.
    #[test]
    fn unknown_member_type_round_trips_instead_of_failing() {
        let urn = crate::schema_urns::GROUP;
        let raw = format!(
            r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":[
                 {{"value":"a","type":"User"}},
                 {{"value":"b","type":"GROUP"}},
                 {{"value":"c","type":"ServiceAccount"}}]}}"#
        );
        let group: Group =
            serde_json::from_str(&raw).expect("unknown type must not fail the payload");
        assert_eq!(group.members[0].r#type, Some(MemberType::User));
        // §2.2: caseExact defaults to false, so "GROUP" is the Group label.
        assert_eq!(group.members[1].r#type, Some(MemberType::Group));
        assert_eq!(
            group.members[2].r#type,
            Some(MemberType::Other("ServiceAccount".to_string()))
        );

        let back = serde_json::to_value(&group).unwrap();
        let types: Vec<&str> = back["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m["type"].as_str().unwrap())
            .collect();
        assert_eq!(types, ["User", "Group", "ServiceAccount"]);
    }
    /// RFC 7643 §2.5 equivalence for `Group.members`: absent, `null` and `[]`
    /// all mean unassigned, and unassigned is omitted on the way out.
    /// Guards the `deserialize_null_as_empty_vec` wiring, which
    /// `#[serde(default)]` alone does not provide.
    #[cfg(not(feature = "compact-multi-valued"))]
    #[test]
    fn members_treat_absent_null_and_empty_alike() {
        let urn = crate::schema_urns::GROUP;
        for (label, raw) in [
            (
                "absent",
                format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides"}}"#),
            ),
            (
                "null",
                format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":null}}"#),
            ),
            (
                "empty",
                format!(r#"{{"schemas":["{urn}"],"displayName":"Tour Guides","members":[]}}"#),
            ),
        ] {
            let group: Group = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("{label} form must deserialize: {e}"));
            assert!(group.members.is_empty(), "{label}: members");
            let out = serde_json::to_value(&group).unwrap();
            assert_eq!(
                out["members"],
                serde_json::json!([]),
                "{label}: unassigned members serialize as [], keeping a §3.5.1 clear-all expressible"
            );
        }
    }

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn group_deserialization_succeeds_for_valid_full_json() {
        let json_data = r#"   {
             "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
             "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
             "displayName": "Tour Guides",
             "members": [
               {
                 "value": "2819c223-7f76-453a-919d-413861904646",
                 "$ref":
           "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646",
                 "display": "Babs Jensen"
               },
               {
                 "value": "902c246b-6245-4190-8e05-00816be7344a",
                 "$ref":
           "https://example.com/v2/Users/902c246b-6245-4190-8e05-00816be7344a",
                 "display": "Mandy Pepperidge"
               }
             ],
             "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location":
           "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
             }
           }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &group {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(
            group.schemas,
            vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
        );
        assert_eq!(
            group.id,
            Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
        );
        assert_eq!(group.display_name, "Tour Guides");

        // Check members
        assert_eq!(group.members.len(), 2);
        assert_eq!(
            group.members[0].value,
            Some("2819c223-7f76-453a-919d-413861904646".to_string())
        );
        assert_eq!(group.members[0].display, Some("Babs Jensen".to_string()));
        assert_eq!(
            group.members[1].value,
            Some("902c246b-6245-4190-8e05-00816be7344a".to_string())
        );
        assert_eq!(
            group.members[1].display,
            Some("Mandy Pepperidge".to_string())
        );

        // Check meta
        let meta = group.meta.unwrap();
        assert_eq!(meta.resource_type, Some("Group".to_string()));
        assert_eq!(meta.created, Some("2010-01-23T04:56:22Z".to_string()));
        assert_eq!(meta.last_modified, Some("2011-05-13T04:42:34Z".to_string()));
        assert_eq!(meta.version, Some("W/\"3694e05e9dff592\"".to_string()));
        assert_eq!(
            meta.location,
            Some("https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a".to_string())
        );
    }

    #[test]
    fn group_deserialization_succeeds_for_valid_json() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides",
            "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }
        }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        if let Err(e) = &group {
            eprintln!("Deserialization failed: {:?}", e);
        }
        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(
            group.schemas,
            vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
        );
        assert_eq!(
            group.id,
            Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
        );
        assert_eq!(group.display_name, "Tour Guides");
    }

    #[test]
    fn group_deserialization_fails_for_invalid_json() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": 12345
        },
        "meta": {
               "resourceType": "Group",
               "created": "2010-01-23T04:56:22Z",
               "lastModified": "2011-05-13T04:42:34Z",
               "version": "W\/\"3694e05e9dff592\"",
               "location": "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a"
            }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        assert!(group.is_err());
    }

    #[test]
    fn group_deserialization_handles_missing_optional_fields() {
        let json_data = r#"{
            "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
            "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
            "displayName": "Tour Guides"
        }"#;

        let group: Result<Group, serde_json::Error> = serde_json::from_str(json_data);

        assert!(group.is_ok());
        let group = group.unwrap();
        assert_eq!(
            group.schemas,
            vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]
        );
        assert_eq!(
            group.id,
            Some("e9e30dba-f08f-4109-8486-d5c6a331660a".into())
        );
        assert_eq!(group.display_name, "Tour Guides");
        assert!(group.members.is_empty());
        assert!(group.meta.is_none());
    }
}
