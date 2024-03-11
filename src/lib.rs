//! # SCIM v2
//!
//! `scim_v2` is a crate that provides utilities for working with the System for Cross-domain Identity Management (SCIM) version 2.0 protocol.
//!
//! This crate provides the following functionalities:
//! - Models for various SCIM resources such as `User`, `Group`, `ResourceType`, `ServiceProviderConfig`, and `EnterpriseUser`.
//! - Functions for validating these resources.
//! - Functions for serializing these resources to JSON.
//! - Functions for deserializing these resources from JSON.
//!
//! Note: Validation is light because the schema is specifically flexible. We only validate required fields, not field types (like email is actually an email)
//!
//! ## Examples
//!
//! Here are some examples of how you can use this crate:
//!
//! ### Validating a User
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::validate_user;
//!
//! let user = User {
//!     // Initialize user fields here...
//!     // ...
//!     ..Default::default()
//! };
//!
//! match validate_user(&user) {
//!     Ok(_) => println!("User is valid."),
//!     Err(e) => println!("User is invalid: {}", e),
//! }
//! ```
//!
//! ### Serializing a User to JSON
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::user_to_json;
//!
//! let user = User {
//!     // Initialize user fields here...
//!     // ...
//!     ..Default::default()
//! };
//!
//! match user_to_json(&user) {
//!     Ok(json) => println!("User in JSON format: {}", json),
//!     Err(e) => println!("Error serializing user to JSON: {}", e),
//! }
//! ```
//!
//! ### Deserializing a User from JSON
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::json_to_user;
//!
//! let json = r#"{
//!     "userName": "jdoe",
//!     "name": {
//!         "formatted": "Mr. John Doe"
//!     }
//! }"#;
//!
//! match json_to_user(json) {
//!     Ok(user) => println!("User: {:?}", user),
//!     Err(e) => println!("Error deserializing JSON to User: {}", e),
//! }
//! ```
//! For more examples and usage details, refer to the documentation of each function and struct.


/// External crate imports
extern crate serde;
extern crate serde_json;
use utils::error::SCIMError;
use crate::models::user::User;
use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::service_provider_config::ServiceProviderConfig;
use crate::models::enterprise_user::EnterpriseUser;



use std::result::Result;

/// Declaring the models module which contains various submodules
pub mod models {
    pub mod user;
    pub mod group;
    pub mod resource_types;
    pub mod service_provider_config;
    pub mod enterprise_user;
    pub mod scim_schema;
}

/// Declaring the utils module which contains the error submodule
pub mod utils {
    pub mod error;
}



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
/// use scim_v2::validate_enterprise_user;
///
/// let enterprise_user = EnterpriseUser {
///     // Initialize enterprise_user fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match validate_enterprise_user(&enterprise_user) {
///     Ok(_) => println!("EnterpriseUser is valid."),
///     Err(e) => println!("EnterpriseUser is invalid: {}", e),
/// }
/// ```
pub fn validate_enterprise_user(enterprise_user: &EnterpriseUser) -> Result<(), SCIMError> {
    if enterprise_user.employee_number.is_none() {
        return Err(SCIMError::MissingRequiredField("employee_number".to_string()));
    }
    if enterprise_user.cost_center.is_none() {
        return Err(SCIMError::MissingRequiredField("cost_center".to_string()));
    }
    if enterprise_user.organization.is_none() {
        return Err(SCIMError::MissingRequiredField("organization".to_string()));
    }
    if enterprise_user.division.is_none() {
        return Err(SCIMError::MissingRequiredField("division".to_string()));
    }
    if enterprise_user.department.is_none() {
        return Err(SCIMError::MissingRequiredField("department".to_string()));
    }
    if enterprise_user.manager.is_none() {
        return Err(SCIMError::MissingRequiredField("manager".to_string()));
    }
    Ok(())
}
