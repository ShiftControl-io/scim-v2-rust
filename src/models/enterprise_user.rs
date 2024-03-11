use serde::{Deserialize, Serialize};
use crate::utils::error::SCIMError;

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

impl TryFrom<&str> for EnterpriseUser {
    type Error = SCIMError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(SCIMError::DeserializationError)
    }
}

impl EnterpriseUser {
    /// Validates an enterprise user.
    ///
    /// This function checks if the enterprise user has `employee_number`, `cost_center`, `organization`, `division`, `department`, and `manager`. If any of these fields are missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `enterprise_user` - A reference to an EnterpriseUser instance.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the enterprise user is valid.
    /// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
    ///
    /// # Example
    ///
    /// ```
    /// use scim_v2::models::enterprise_user::EnterpriseUser;
    ///
    /// let enterprise_user = EnterpriseUser {
    ///     // Initialize enterprise_user fields here...
    ///     // ...
    ///     ..Default::default()
    /// };
    ///
    /// match enterprise_user.validate() {
    ///     Ok(_) => println!("EnterpriseUser is valid."),
    ///     Err(e) => println!("EnterpriseUser is invalid: {}", e),
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), SCIMError> {
        if self.employee_number.is_none() {
            return Err(SCIMError::MissingRequiredField("employee_number".to_string()));
        }
        if self.cost_center.is_none() {
            return Err(SCIMError::MissingRequiredField("cost_center".to_string()));
        }
        if self.organization.is_none() {
            return Err(SCIMError::MissingRequiredField("organization".to_string()));
        }
        if self.division.is_none() {
            return Err(SCIMError::MissingRequiredField("division".to_string()));
        }
        if self.department.is_none() {
            return Err(SCIMError::MissingRequiredField("department".to_string()));
        }
        if self.manager.is_none() {
            return Err(SCIMError::MissingRequiredField("manager".to_string()));
        }
        Ok(())
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
