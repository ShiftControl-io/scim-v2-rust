use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::scim_schema::Schema;
use crate::models::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub schemas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(rename = "excludedAttributes", skip_serializing_if = "Option::is_none")]
    excluded_attributes: Option<Vec<String>>,
    pub filter: String,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    pub count: i64,
}

impl Default for SearchRequest {
    fn default() -> Self {
        SearchRequest {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:SearchRequest".to_string()],
            attributes: None,
            excluded_attributes: None,
            filter: "".to_string(),
            start_index: 1,
            count: 100,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "excludedAttributes", skip_serializing_if = "Option::is_none")]
    pub excluded_attributes: Option<String>,
}

impl Default for ListQuery {
    fn default() -> Self {
        ListQuery {
            filter: Some("".to_string()),
            start_index: Some(1),
            count: Some(100),
            attributes: Some("".to_string()),
            excluded_attributes: Some("".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Resource {
    User(Box<User>),
    Schema(Box<Schema>),
    Group(Box<Group>),
    ResourceType(Box<ResourceType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i64,
    #[serde(rename = "totalResults")]
    pub total_results: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    pub schemas: Vec<String>,
    #[serde(rename = "Resources")]
    pub resources: Vec<Resource>,
}

impl Default for ListResponse {
    fn default() -> Self {
        ListResponse {
            items_per_page: 0,
            total_results: 0,
            start_index: 1,
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:ListResponse".to_string()],
            resources: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PatchOp {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<PatchOperations>,
}

impl Default for PatchOp {
    fn default() -> Self {
        PatchOp {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".to_string()],
            operations: vec![PatchOperations::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchOperations {
    pub op: String,
    pub value: HashMap<String, Value>,
}

impl Default for PatchOperations {
    fn default() -> Self {
        PatchOperations {
            op: "".to_string(),
            value: HashMap::new(),
        }
    }
}



