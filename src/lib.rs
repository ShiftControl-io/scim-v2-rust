//! # SCIM v2
//!
//! Models, parsers and validators for the System for Cross-domain Identity
//! Management (SCIM) 2.0 protocol — RFC 7642, RFC 7643 and RFC 7644.
//!
//! The crate is deliberately narrow: it models the wire format, parses the two
//! grammars the RFC defines, and checks the attributes the RFC marks REQUIRED.
//! It performs no I/O, evaluates no filters against storage, and does not wrap
//! `serde` — use `serde_json` directly for that.
//!
//! ## What is here
//!
//! - **Resources** — [`User`](models::user::User), [`Group`](models::group::Group),
//!   [`EnterpriseUser`](models::enterprise_user::EnterpriseUser),
//!   [`Schema`](models::scim_schema::Schema),
//!   [`ResourceType`](models::resource_types::ResourceType),
//!   [`ServiceProviderConfig`](models::service_provider_config::ServiceProviderConfig).
//! - **Protocol messages** — [`ListResponse`](models::others::ListResponse),
//!   [`SearchRequest`](models::others::SearchRequest),
//!   [`ListQuery`](models::others::ListQuery),
//!   [`PatchOp`](models::others::PatchOp),
//!   [`ScimHttpError`](models::errors::ScimHttpError).
//! - **Filter and PATCH-path parsing** — [`filter`], implementing the
//!   RFC 7644 §3.4.2.2 grammar and the §3.5.2 PATCH path rule.
//! - **Validation** — the [`Validate`] trait, reporting failures by SCIM wire
//!   path so a server can echo them in an RFC 7644 §3.12 response.
//!
//! ## Feature flags
//!
//! All three are enabled by default. Turn them off to shrink the build.
//!
//! | Feature | Provides | Cost when off |
//! |---------|----------|---------------|
//! | `filter` | [`filter`] and its parser | eight fewer crates: `lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`, `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl` |
//! | `models` | every resource and protocol message | |
//! | `schemas` | the embedded RFC 7643 schema definitions and the [`get_schemas`](models::scim_schema::get_schemas) lookup | ~48 KB of `include_str!` data |
//!
//! Dropping `filter` takes the dependency tree from 22 crates to 14, and the
//! total compile work from 35s to 14s measured serially (`-j1`, release). On a
//! parallel build the saving is smaller, because the crates it drops overlap
//! with `syn` on the critical path — the point is the smaller supply-chain
//! surface, which now excludes a regex engine.
//!
//! `SearchRequest`, `ListQuery` and `PatchOp` need both `models` and `filter`,
//! since each carries a parsed filter or PATCH path. A filter-only consumer
//! wants:
//!
//! ```toml
//! scim_v2 = { version = "1", default-features = false, features = ["filter"] }
//! ```
//!
//! ## Deserializing a resource
//!
//! Use `serde_json` directly. Every model is a plain `serde` type.
//!
//! ```rust
//! # #[cfg(feature = "models")] {
//! use scim_v2::models::user::User;
//!
//! let json = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"jdoe@example.com"}"#;
//! let user: User<String> = serde_json::from_str(json).unwrap();
//! assert_eq!(user.user_name, "jdoe@example.com");
//! # }
//! ```
//!
//! ## Checking RFC-required attributes
//!
//! Nearly every SCIM attribute is optional, so the models make almost
//! everything `Option`. [`Validate`] carries the handful of checks `serde`
//! cannot express, and names the offending attribute by its **wire** path.
//!
//! ```rust
//! # #[cfg(feature = "models")] {
//! use scim_v2::{Validate, models::user::User};
//!
//! let user = User::<String> {
//!     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
//!     user_name: String::new(),
//!     ..Default::default()
//! };
//!
//! let err = user.validate().unwrap_err();
//! assert_eq!(err.path(), "userName");
//! assert_eq!(err.scim_type(), "invalidValue");
//! # }
//! ```
//!
//! ## Parsing a filter
//!
//! ```rust
//! # #[cfg(feature = "filter")] {
//! use scim_v2::filter::Filter;
//!
//! let filter: Filter = r#"userName eq "bjensen" and title pr"#.parse().unwrap();
//! println!("{filter}");
//! # }
//! ```
//!
//! For the parsed shape, precedence rules and the `invalidFilter` error path,
//! see the [`filter`] module docs.

// Include the schema files into the binary.
#[cfg(feature = "schemas")]
const USER_SCHEMA: &str = include_str!("schemas/user.json");
#[cfg(feature = "schemas")]
const GROUP_SCHEMA: &str = include_str!("schemas/group.json");
#[cfg(feature = "schemas")]
const ENTERPRISE_USER_SCHEMA: &str = include_str!("schemas/enterprise_user.json");

/// The RFC 7643 resource models and RFC 7644 protocol messages.
#[cfg(feature = "models")]
pub mod models {
    pub mod enterprise_user;
    pub mod errors;
    pub mod group;
    pub mod others;
    pub mod resource_types;
    pub mod scim_schema;
    pub mod service_provider_config;
    pub mod user;
}

#[cfg(feature = "filter")]
#[rustfmt::skip]
#[allow(clippy::all)]
pub(crate) mod filter_parser;
#[cfg(feature = "filter")]
pub mod filter;
pub mod schema_urns;

pub use utils::validation::{Validate, ValidationError, ValidationErrorKind};

/// Declaring the utils module which contains the error submodule
pub mod utils {
    pub mod error;
    pub(crate) mod serde;
    pub mod validation;
}
