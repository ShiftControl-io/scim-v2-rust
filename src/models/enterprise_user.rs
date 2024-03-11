use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct EnterpriseUser {
    #[serde(rename = "employeeNumber")]
    pub employee_number: Option<String>,
    #[serde(rename = "costCenter")]
    pub cost_center: Option<String>,
    pub organization: Option<String>,
    pub division: Option<String>,
    pub department: Option<String>,
    pub manager: Option<Manager>,
}
impl Default for EnterpriseUser {
    fn default() -> Self {
        EnterpriseUser {
            employee_number: None,
            cost_center: None,
            organization: None,
            division: None,
            department: None,
            manager: None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Manager {
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}
