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
//! - **Resources** — `User`, `Group`,
//!   `EnterpriseUser`,
//!   `Schema`,
//!   `ResourceType`,
//!   `ServiceProviderConfig`.
//! - **Protocol messages** — `ListResponse`,
//!   `SearchRequest`,
//!   `ListQuery`,
//!   `PatchOp`,
//!   `ScimHttpError`.
//! - **Filter and PATCH-path parsing** — `filter`, implementing the
//!   RFC 7644 §3.4.2.2 grammar and the §3.5.2 PATCH path rule.
//! - **Validation** — the [`Validate`] trait, reporting failures by SCIM wire
//!   path so a server can echo them in an RFC 7644 §3.12 response.
//!
//! ## Feature flags
//!
//! All four are on by default and all four are additive: a feature only ever
//! compiles more. Wire-behaviour choices are wrapper types, because Cargo
//! unifies features across the dependency graph and a transitive crate could
//! otherwise flip them for everyone.
//!
//! | Feature | Provides |
//! |---------|----------|
//! | `filter` | `filter` and its parser. Off: eight fewer crates (`lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`, `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl`) |
//! | `models` | every resource and protocol message |
//! | `schemas` | the embedded RFC 7643 schema definitions and the `get_schemas` lookup; ~48 KB of `include_str!` |
//! | `case-insensitive` | `utils::case`, which canonicalises attribute-name case before deserializing (RFC 7643 §2.1) |
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
//! ## Leniency on input, canonical form on output
//!
//! Real providers do not all send conformant SCIM, and a client that cannot
//! read a provider's payload is useless. So every wire parser here is
//! lenient, and every serializer emits the RFC's canonical form. A round-trip
//! therefore *canonicalizes* rather than preserving bytes.
//!
//! | Accepted on input | Emitted | Why |
//! |---|---|---|
//! | `"true"` / `"True"` for a boolean | `true` | Entra; RFC 7643 §2.3.2 defines the JSON literal |
//! | `null`, `[]` or absence for a multi-valued attribute | `[]` (or omitted via `utils::compact::Compact`) | RFC 7643 §2.5 equivalence; RFC 7644 §3.5.1 gives `[]` clear-all meaning |
//! | `Add` / `ADD` for a PATCH `op` | `add` | RFC 7644 §3.5.2 spells it lowercase; Entra does not |
//! | `Ascending`, `GROUP` for `sortOrder` / `members.type` | `ascending`, `Group` | schema `caseExact: false` |
//! | a `members.type` or `scimType` outside the RFC's list | preserved verbatim | RFC 7643 §7: canonical values are *suggested* |
//! | attribute names in any case, via `utils::case` (`case-insensitive` feature) | canonical camelCase | RFC 7643 §2.1: "Attribute names are case insensitive" |
//!
//! ## Validation — deserializing does not validate
//!
//! None of the leniency above is where conformance is enforced, and neither is
//! deserialization: `serde_json::from_str::<User>(..)` will return a `User`
//! with an empty `userName`, two `primary: true` emails, or an `id` on a create
//! request. Store that and you will later emit a non-conformant response, or
//! echo a `password` back.
//!
//! [`Validate`] holds the RFC's REQUIRED rules and names the offending
//! attribute by its **wire** path. It is direction-aware:
//! [`validate_as`](Validate::validate_as) takes a [`Context`], because RFC 7643
//! §3.1 forbids `id` on a create and requires it on a response. [`Valid<T>`]
//! is a value the type system knows has passed, and [`Strict<T, M>`] runs the
//! check inside deserialization so a bad body never becomes a `T` at all. A
//! server should take `Strict<User, CreateRequest>` in its handlers; a client
//! can usually stop at `validate()`.
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
//! assert_eq!(err.scim_type_str(), "invalidValue");
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
//! see the `filter` module docs.

/// Every `rust` block in README.md is compiled and run as a doctest, so the
/// front page cannot drift from the API. Costs nothing at build time —
/// `cfg(doctest)` is set only while rustdoc collects tests.
///
/// Gated on the full feature set. The README documents the crate as published
/// (all features default on) and its examples use `filter`, `PatchOp` and the
/// tolerant query types; the alternative would be `# #[cfg(...)] {` guards in
/// the README itself, which rustdoc hides but GitHub renders literally.
#[cfg(all(doctest, feature = "filter", feature = "models", feature = "schemas"))]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

// Include the schema files into the binary.
#[cfg(feature = "schemas")]
pub(crate) const USER_SCHEMA: &str = include_str!("schemas/user.json");
#[cfg(feature = "schemas")]
pub(crate) const GROUP_SCHEMA: &str = include_str!("schemas/group.json");
#[cfg(feature = "schemas")]
pub(crate) const ENTERPRISE_USER_SCHEMA: &str = include_str!("schemas/enterprise_user.json");

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

pub use utils::validation::{
    Context, ContextMarker, CreateRequest, ReplaceRequest, Response, Strict, Valid, Validate,
    ValidationError, ValidationErrorKind, at_most_one_primary, require_schema_urn,
};

/// Declaring the utils module which contains the error submodule
pub mod utils {
    #[cfg(feature = "case-insensitive")]
    pub mod case;
    pub mod compact;
    pub mod error;
    #[cfg(feature = "models")]
    pub(crate) mod serde;
    pub mod validation;
}
