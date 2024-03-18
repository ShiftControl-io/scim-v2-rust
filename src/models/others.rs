use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    pub schemas: Vec<String>,
    pub attributes: Vec<String>,
    pub filter: String,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    pub count: i64,
}
impl Default for SearchRequest {
    fn default() -> Self {
        SearchRequest {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:SearchRequest".to_string()],
            attributes: vec![],
            filter: "".to_string(),
            start_index: 1,
            count: 100,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ListQuery {
    filter: String,
    #[serde(rename = "startIndex")]
    start_index: i64,
    count: Option<i64>,
}
impl Default for ListQuery {
    fn default() -> Self {
        ListQuery {
            filter: "".to_string(),
            start_index: 1,
            count: Some(100),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    items_per_page: i64,
    #[serde(rename = "totalResults")]
    total_results: i64,
    #[serde(rename = "startIndex")]
    start_index: i64,
    schemas: Vec<String>,
    #[serde(rename = "Resources")]
    resources: Vec<User>,
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
