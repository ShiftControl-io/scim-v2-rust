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
//! ```rust
//! use scim_v2::models::user::User;
//!
//! let user = User {
//!     user_name: "jdoe@example.com".to_string(),
//!    // other fields...
//!     ..Default::default()
//! };
//!
//! match user.validate() {
//!     Ok(_) => println!("User is valid."),
//!     Err(e) => println!("User is invalid: {}", e),
//! }
//! ```
//!
//! ### Serialize the `User` instance to a JSON string, using the custom SCIMError for error handling.
//!
//! # Examples
//! 
//! ```rust
//! use scim_v2::models::user::User;
//!
//! let user = User {
//!     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
//!     user_name: "jdoe@example.com".to_string(),
//!     // Initialize other fields as necessary...
//!     ..Default::default()
//! };
//!
//! match user.serialize() {
//!     Ok(json) => println!("Serialized User: {}", json),
//!     Err(e) => println!("Serialization error: {}", e),
//! }
//! ```
//!
//! ### Deserializing JSON to a User, using the custom SCIMError for error handling.
//!
//! # Examples
//!
//! ```rust
//! use scim_v2::models::user::User;
//!
//! let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
//! match User::try_from(user_json) {
//!     Ok(user) => println!("Successfully converted JSON to User: {:?}", user),
//!     Err(e) => println!("Error converting from JSON to User: {}", e),
//! }
//! ```
//!
//! You can also use a built-in deserialize function if you'd prefer.
//! ```
//! use scim_v2::models::user::User;
//!
//! let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
//! match User::deserialize(user_json) {
//!     Ok(user) => println!("Deserialized User: {:?}", user),
//!     Err(e) => println!("Deserialization error: {}", e),
//! }
//! ```
//! For more examples and usage details, refer to the documentation of each function and struct.


/// External crate imports
extern crate serde;
extern crate serde_json;

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




