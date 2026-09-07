//Schema for group
use crate::models::scim_schema::Meta;
use crate::utils::validation::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<Member<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MemberType {
    #[serde(rename = "User", alias = "USER", alias = "user")]
    User,
    #[serde(rename = "Group", alias = "GROUP", alias = "group")]
    Group,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
        if self.schemas.is_empty() {
            return Err(ValidationError::missing_required("schemas"));
        }
        if self.display_name.is_empty() {
            return Err(ValidationError::missing_required("displayName"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// RFC 7643 §2.5 equivalence for `Group.members`: absent, `null` and `[]`
    /// all mean unassigned, and unassigned is omitted on the way out.
    /// Guards the `deserialize_null_as_empty_vec` wiring, which
    /// `#[serde(default)]` alone does not provide.
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
            assert!(
                !out.as_object().unwrap().contains_key("members"),
                "{label}: unassigned members must be omitted"
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
