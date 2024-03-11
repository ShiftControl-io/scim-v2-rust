use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub created: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub version: String,
    pub location: String,
}
