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
//! ## Quick start
//!
//! A SCIM server receiving `POST /Users`. [`Strict`] deserializes the body and
//! validates it for the direction it is travelling in one step, so a
//! non-conformant request never becomes a `User` at all;
//! the error names the offending attribute by its wire path and carries the
//! RFC 7644 §3.12 `scimType`, ready for a `400`.
//!
//! ```rust
//! # #[cfg(feature = "models")] {
//! use scim_v2::{Context, CreateRequest, Strict, Valid};
//! use scim_v2::models::{scim_schema::Meta, user::User};
//!
//! let body = r#"{
//!   "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
//!   "userName": "bjensen@example.com",
//!   "name": {"givenName": "Barbara", "familyName": "Jensen"},
//!   "emails": [{"value": "bjensen@example.com", "type": "work", "primary": true}]
//! }"#;
//!
//! // Deserialize and validate as a create request. RFC 7643 §3.1 forbids `id`
//! // here; a blank `userName` or a second `primary: true` email would fail too.
//! let request: Valid<User<String>> =
//!     match serde_json::from_str::<Strict<User<String>, CreateRequest>>(body) {
//!         Ok(strict) => strict.into_valid(),
//!         Err(e) => return Err(e.into()), // answer 400 with `e.to_string()`
//!     };
//! assert_eq!(request.user_name, "bjensen@example.com");
//!
//! // Store it, then build the representation to return. A response MUST carry
//! // `id` (§3.1) and MUST NOT carry `password` (§4.1); `Context::Response`
//! // checks both, and `Valid` is the proof that it did.
//! let mut user = request.into_inner();
//! user.id = Some("2819c223-7f76-453a-919d-413861904646".to_string());
//! user.meta = Some(Meta {
//!     resource_type: Some("User".to_string()),
//!     location: Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string()),
//!     ..Default::default()
//! });
//! let response = Valid::new(user, Context::Response)?;
//! let json = serde_json::to_string(&response)?;
//! assert!(json.contains(r#""id":"2819c223-7f76-453a-919d-413861904646""#));
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! A SCIM client reading `GET /Users`. A page deserializes straight into the
//! resource type, the lenient parsers absorb what real providers send (here
//! Entra's `"active": "True"`), and `validate()` checks the envelope and every
//! resource on the page.
//!
//! ```rust
//! # #[cfg(feature = "models")] {
//! use scim_v2::{Validate, models::{others::ListResponse, user::User}};
//!
//! let page = r#"{
//!   "schemas": ["urn:ietf:params:scim:api:messages:2.0:ListResponse"],
//!   "totalResults": 1, "startIndex": 1, "itemsPerPage": 1,
//!   "Resources": [{
//!     "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
//!     "id": "2819c223-7f76-453a-919d-413861904646",
//!     "userName": "bjensen@example.com",
//!     "active": "True"
//!   }]
//! }"#;
//!
//! let page: ListResponse<User<String>> = serde_json::from_str(page)?;
//! page.validate()?;
//! assert_eq!(page.resources[0].active, Some(true));
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! And the query side: a `?filter=` parameter parses into an AST for you to
//! map onto your storage. A malformed filter is an error to answer with
//! `invalidFilter`, never a panic.
//!
//! ```rust
//! # #[cfg(feature = "filter")] {
//! use scim_v2::filter::{AttrExp, CompValue, CompareOp, Filter};
//!
//! let filter: Filter = r#"userName eq "bjensen@example.com""#.parse()?;
//! let Filter::Attr(AttrExp::Comparison(path, CompareOp::Eq, CompValue::Str(value))) = &filter
//! else {
//!     unreachable!()
//! };
//! assert_eq!((path.name.as_str(), value.as_str()), ("userName", "bjensen@example.com"));
//! assert!("userName garbage".parse::<Filter>().is_err());
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! For the parsed shape, precedence rules and the `invalidFilter` error path,
//! see the `filter` module docs.
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
//! ## Validation — deserializing does not validate
//!
//! Every model is a plain `serde` type, and deserializing one enforces nothing
//! beyond JSON shape: `serde_json::from_str::<User>(..)` will return a `User`
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
//! server should take `Strict<User, CreateRequest>` in its handlers, as in the
//! quick start; a client can usually stop at `validate()`.
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
//! | `null`, `[]` or absence for a multi-valued attribute | `[]` (or omitted via `compact::Compact`) | RFC 7643 §2.5 equivalence; RFC 7644 §3.5.1 gives `[]` clear-all meaning |
//! | `Add` / `ADD` for a PATCH `op` | `add` | RFC 7644 §3.5.2 spells it lowercase; Entra does not |
//! | `Ascending`, `GROUP` for `sortOrder` / `members.type` | `ascending`, `Group` | schema `caseExact: false` |
//! | a `members.type` or `scimType` outside the RFC's list | preserved verbatim | RFC 7643 §7: canonical values are *suggested* |
//! | attribute names in any case, via [`case_insensitive`] | canonical camelCase | RFC 7643 §2.1: "Attribute names are case insensitive" |
//!
//! Leniency stops at the attribute *names*, and it is never implicit: the
//! derives stay exact-match and a caller who cannot trust a peer's casing
//! reaches for [`case_insensitive`] at the call site.
//!
//! ```
//! use scim_v2::models::user::User;
//!
//! let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"USERNAME":"bjensen"}"#;
//!
//! assert!(serde_json::from_str::<User>(body).is_err()); // exact-match
//! let user: User = scim_v2::case_insensitive::from_str(body).unwrap(); // §2.1
//! assert_eq!(user.user_name, "bjensen");
//! ```
//!
//! ## Timestamps
//!
//! `meta.created` and `meta.lastModified` are [`ScimDateTime`], not `String`.
//! RFC 7643 §2.3.5 requires a valid `xsd:dateTime` including both a date and a
//! time, and §3.1 makes every `meta` sub-attribute readOnly and
//! provider-assigned — so the party most likely to write a malformed one is a
//! service provider built on this crate, and the type is what stops it.
//!
//! ```
//! use scim_v2::ScimDateTime;
//!
//! let created: ScimDateTime = "2010-01-23T04:56:22Z".parse().unwrap();
//! assert!("2010-02-30T04:56:22Z".parse::<ScimDateTime>().is_err());
//! assert_eq!(created.as_str(), "2010-01-23T04:56:22Z");
//! ```
//!
//! It validates a lexical form and nothing more: no arithmetic, no ordering,
//! no time zone conversion. The crate takes no date-time dependency because
//! none of them fits — XSD makes the offset optional, and `time`, `chrono`
//! and `jiff` each split offset-bearing and offset-less values across two
//! different types, so a field typed as one of them would reject conformant
//! input or invent an offset. [The module docs](models::datetime) give the
//! full limits and how to convert.
//!
//! ## Feature flags
//!
//! All three are on by default and all three are additive: a feature only ever
//! compiles more. Behaviour choices are never features — they are wrapper
//! types ([`CaseInsensitive`], [`Compact`]) or a call-site choice, because
//! Cargo unifies features across the dependency graph and a transitive crate
//! could otherwise flip them for everyone.
//!
//! | Feature | Provides |
//! |---------|----------|
//! | `filter` | `filter` and its parser. Off: eight fewer crates (`lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`, `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl`) |
//! | `models` | every resource and protocol message |
//! | `schemas` | the embedded RFC 7643 schema definitions and the `get_schemas` lookup; ~48 KB of `include_str!` |
//!
//! Dropping `filter` takes the dependency tree from 22 crates to 14 and removes
//! a regex engine from the supply chain, which is the point; it also shortens
//! the build, more on a serial build than a parallel one, since the crates it
//! drops overlap with `syn` on the critical path.
//!
//! `SearchRequest`, `ListQuery` and `PatchOp` need both `models` and `filter`,
//! since each carries a parsed filter or PATCH path. A filter-only consumer
//! wants:
//!
//! ```toml
//! scim_v2 = { version = "1", default-features = false, features = ["filter"] }
//! ```

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
    pub mod datetime;
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

#[cfg(feature = "models")]
pub use models::datetime::{ParseScimDateTimeError, ScimDateTime};

pub use case_insensitive::CaseInsensitive;
pub use compact::Compact;

pub use utils::validation::{
    Context, ContextMarker, CreateRequest, ReplaceRequest, Response, Strict, Valid, Validate,
    ValidationError, ValidationErrorKind, at_most_one_primary, require_schema_urn,
};

/// Case-insensitive attribute names, per RFC 7643 §2.1.
pub mod case_insensitive;
/// Compact serialization: omit unassigned multi-valued attributes.
pub mod compact;

/// Declaring the utils module which contains the error submodule
pub mod utils {
    pub mod error;
    #[cfg(feature = "models")]
    pub(crate) mod serde;
    pub mod validation;
}
