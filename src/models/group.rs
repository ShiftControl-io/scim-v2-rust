//Schema for group
use serde::{Deserialize, Serialize};
use crate::models::scim_schema::Meta;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub members: Option<Vec<Member>>,
    pub meta: Option<Meta>,
}
impl Default for Group {
    fn default() -> Self {
        Group {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
            id: "default_id".to_string(),
            display_name: "default_display_name".to_string(),
            members: None,
            meta: None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub display: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;


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
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
        assert_eq!(group.display_name, "Tour Guides");

        // Check members
        assert_eq!(group.members.as_ref().unwrap().len(), 2);
        assert_eq!(group.members.as_ref().unwrap()[0].value, Some("2819c223-7f76-453a-919d-413861904646".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[0].display, Some("Babs Jensen".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[1].value, Some("902c246b-6245-4190-8e05-00816be7344a".to_string()));
        assert_eq!(group.members.as_ref().unwrap()[1].display, Some("Mandy Pepperidge".to_string()));

        // Check meta
        let meta = group.meta.unwrap();
        assert_eq!(meta.resource_type, "Group");
        assert_eq!(meta.created, "2010-01-23T04:56:22Z");
        assert_eq!(meta.last_modified, "2011-05-13T04:42:34Z");
        assert_eq!(meta.version, "W/\"3694e05e9dff592\"");
        assert_eq!(meta.location, "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a");
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
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
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
        assert_eq!(group.schemas, vec!["urn:ietf:params:scim:schemas:core:2.0:Group"]);
        assert_eq!(group.id, "e9e30dba-f08f-4109-8486-d5c6a331660a");
        assert_eq!(group.display_name, "Tour Guides");
        assert!(group.members.is_none());
        assert!(group.meta.is_none());
    }
}