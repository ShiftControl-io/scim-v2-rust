//! # SCIM v2
//!
//! Models, parsers and validators for the System for Cross-domain Identity
//! Management (SCIM) 2.0 protocol. RFC 7642, RFC 7643 and RFC 7644 define the
//! protocol.
//!
//! ## What this crate is for
//!
//! SCIM is a protocol made of messages, and this crate is about those
//! messages. It reads a message that arrived and tells you whether the
//! message conforms. It builds a message to send, and refuses to build one
//! that does not conform. A client and a server need the same thing from
//! opposite sides, so the same types serve both, and [`Context`] states the
//! direction a value travels in.
//!
//! The goal is to apply the RFCs correctly, and correct is not always
//! obvious. The RFCs leave some questions open, and real providers send forms
//! the RFCs never describe. This crate decides those cases instead of passing
//! the problem to you, and the doc comment on each decision quotes the RFC
//! sentence it rests on. Where the decision is genuinely yours, the type
//! hands it to you rather than choosing for you. [`Multi<T>`](Multi) is the
//! clearest example. RFC 7644 §3.5.1 makes an omitted attribute a different
//! instruction from a cleared one, so [`Multi<T>`](Multi) keeps the two apart
//! and your server decides what each one means.
//!
//! The scope is narrow. This crate models the wire format, parses the two
//! grammars the RFC defines, and checks the attributes the RFC marks
//! REQUIRED. It performs no I/O and evaluates no filter against storage. It
//! does not wrap `serde`, so use `serde_json` directly.
//!
//! ## Quick start
//!
//! A SCIM server answers `POST /Users`. [`Strict`] deserializes the body and
//! validates the body for its direction in one step. A non-conformant request
//! therefore never becomes a `User`. The error names the attribute at fault
//! by its wire path. The error also carries the RFC 7644 §3.12 `scimType`,
//! ready for a `400`.
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
//! A SCIM client reads `GET /Users`. A page deserializes directly into the
//! resource type. The lenient parsers accept what real providers send, such as
//! Entra's `"active": "True"`. `validate()` checks the envelope and every
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
//! The query side comes next. A `?filter=` parameter parses into an AST. Map
//! the AST onto your storage. A malformed filter gives an error to answer
//! with `invalidFilter`. A malformed filter never causes a panic.
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
//! See the `filter` module docs for the parsed shape, the precedence rules
//! and the `invalidFilter` error path.
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
//! - **Filter and PATCH-path parsers** — `filter`. The module implements the
//!   RFC 7644 §3.4.2.2 grammar and the §3.5.2 PATCH path rule.
//! - **Validation** — the [`Validate`] trait. The trait reports a failure by
//!   its SCIM wire path, so a server can echo the path in an RFC 7644 §3.12
//!   response.
//!
//! ## Validation — deserialization does not validate
//!
//! Every model is a plain `serde` type. Deserialization enforces nothing
//! beyond the JSON shape. `serde_json::from_str::<User>(..)` returns a `User`
//! with an empty `userName`, two `primary: true` emails, or an `id` on a
//! create request. Store such a `User`, and you later emit a non-conformant
//! response or echo a `password` back.
//!
//! [`Validate`] holds the RFC's REQUIRED rules. The trait names the attribute
//! at fault by its **wire** path. The trait is direction-aware:
//! [`validate_as`](Validate::validate_as) takes a [`Context`]. RFC 7643 §3.1
//! forbids `id` on a create request and requires `id` on a response.
//! [`Valid<T>`] is a value the type system knows has passed the check.
//! [`Strict<T, M>`] runs the check inside deserialization, so a bad body never
//! becomes a `T`. A server should take `Strict<User, CreateRequest>` in its
//! handlers, as the quick start shows. A client can usually stop at
//! `validate()`.
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
//! Real providers do not all send conformant SCIM. A client that cannot read
//! a provider's payload is useless. Every wire parser here is therefore
//! lenient, and every serializer emits the RFC's canonical form. A round-trip
//! therefore *canonicalizes* the payload. A round-trip does not preserve the
//! bytes.
//!
//! | Accepted on input | Emitted | Why |
//! |---|---|---|
//! | `"true"` / `"True"` for a boolean | `true` | Entra; RFC 7643 §2.3.2 defines the JSON literal |
//! | `null` or `[]` for a multi-valued attribute | `[]` | RFC 7644 §3.5.1 gives `[]` clear-all meaning |
//! | an absent multi-valued attribute | omitted | RFC 7644 §3.5.1 makes it "not asserted by the client" |
//! | `Add` / `ADD` for a PATCH `op` | `add` | RFC 7644 §3.5.2 spells it lowercase; Entra does not |
//! | `Ascending`, `GROUP` for `sortOrder` / `members.type` | `ascending`, `Group` | schema `caseExact: false` |
//! | a `members.type` or `scimType` outside the RFC's list | preserved verbatim | RFC 7643 §7: canonical values are *suggested* |
//! | attribute names in any case, via [`case_insensitive`] | canonical camelCase | RFC 7643 §2.1: "Attribute names are case insensitive" |
//!
//! Leniency stops at the attribute *names*, and the leniency is never
//! implicit. The derives stay exact-match. A caller who cannot trust a peer's
//! letter case uses [`case_insensitive`] at the call site.
//!
//! ```
//! # #[cfg(feature = "models")] {
//! use scim_v2::models::user::User;
//!
//! let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"USERNAME":"bjensen"}"#;
//!
//! assert!(serde_json::from_str::<User>(body).is_err()); // exact-match
//! let user: User = scim_v2::case_insensitive::from_str(body).unwrap(); // §2.1
//! assert_eq!(user.user_name, "bjensen");
//! # }
//! ```
//!
//! ## Timestamps
//!
//! `meta.created` and `meta.lastModified` are `ScimDateTime`, not `String`.
//! RFC 7643 §2.3.5 requires a valid `xsd:dateTime` with both a date and a
//! time. §3.1 makes every `meta` sub-attribute readOnly and provider-assigned.
//! The party most likely to write a malformed timestamp is therefore a service
//! provider built on this crate. The `ScimDateTime` type stops that service
//! provider.
//!
//! ```
//! # #[cfg(feature = "models")] {
//! use scim_v2::ScimDateTime;
//!
//! let created: ScimDateTime = "2010-01-23T04:56:22Z".parse().unwrap();
//! assert!("2010-02-30T04:56:22Z".parse::<ScimDateTime>().is_err());
//! assert_eq!(created.as_str(), "2010-01-23T04:56:22Z");
//! # }
//! ```
//!
//! The type validates a lexical form. The type does no arithmetic, no time
//! zone conversion and no normalization. `==` is textual. Ask about instants
//! with `ScimDateTime::xsd_equivalent` and `ScimDateTime::xsd_partial_cmp`.
//! Both functions implement §3.3.7.1. Both answer `None` where the spec says
//! two values are ·incomparable·. The crate takes no date-time dependency,
//! because no date-time crate fits. XSD makes the offset optional. `time`,
//! `chrono` and `jiff` each split values with an offset and values without an
//! offset across two different types. A field with one of those types would
//! reject conformant input or invent an offset.
//! [The module docs](models::datetime) give the full limits and show how to
//! convert (`models::datetime`).
//!
//! ## Feature flags
//!
//! All three features are on by default. All three are additive: a feature
//! only ever compiles more code. Behaviour choices are never features. A
//! behaviour choice is a wrapper type ([`CaseInsensitive`]) or a
//! call-site choice. Cargo unifies features across the dependency graph, and a
//! transitive crate could otherwise flip a behaviour choice for everyone.
//!
//! | Feature | Provides |
//! |---------|----------|
//! | `filter` | `filter` and its parser. Off: eight fewer crates (`lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`, `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl`) |
//! | `models` | every resource and protocol message |
//! | `schemas` | the embedded RFC 7643 schema definitions and the `get_schemas` lookup; ~48 KB of `include_str!` |
//!
//! Without `filter` the dependency tree falls from 22 crates to 14, and the
//! supply chain loses a regex engine. The lost regex engine is the point. The
//! build also gets shorter. The gain is larger on a serial build than on a
//! parallel build. The dropped crates overlap with `syn` on the critical
//! path.
//!
//! `SearchRequest`, `ListQuery` and `PatchOp` need both `models` and `filter`.
//! Each of the three carries a parsed filter or a parsed PATCH path. A
//! filter-only consumer wants this dependency declaration:
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
pub use multi::Multi;

pub use utils::validation::{
    Context, ContextMarker, CreateRequest, ReplaceRequest, Response, Strict, Valid, Validate,
    ValidationError, ValidationErrorKind, at_most_one_primary, require_schema_urn,
};

pub mod case_insensitive;
pub mod multi;

/// Declaring the utils module which contains the error submodule
pub mod utils {
    pub mod error;
    #[cfg(feature = "models")]
    pub(crate) mod serde;
    pub mod validation;
}
