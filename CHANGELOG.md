# CHANGELOG

## 1.0.0

First stable release. The API is now covered by SemVer, and
`cargo-semver-checks` runs on every PR to keep it that way.

This release breaks compatibility deliberately, in the one place where doing
so is free, and every break is listed below.

### Breaking Changes

- **Multi-valued attributes are `Vec<T>`, not `Option<Vec<T>>`** — all 18 of
  them: `User`'s `emails`, `addresses`, `phoneNumbers`, `ims`, `photos`,
  `groups`, `entitlements`, `roles` and `x509Certificates`; `Group.members`;
  `ResourceType.schemaExtensions`; `Schema`'s `canonicalValues`,
  `subAttributes` and `referenceTypes`; `SearchRequest`'s `attributes` and
  `excludedAttributes`. RFC 7643 §2.5 makes an unassigned attribute, an
  explicit `null`, and an empty array equivalent in state, so the old type
  offered three representations of one thing and made every caller decide
  whether `None` and `Some(vec![])` differed. Suggested by @travipross while
  reviewing #48. All three wire forms now deserialize to an empty `Vec`,
  including explicit `null` — which needs a `deserialize_with`, since
  `#[serde(default)]` alone rejects `"roles": null`, a form providers do send.

- **`ListResponse` is generic over the resource, not the ID type.** `GET
  /Users` is now `ListResponse<User<String>>` and deserializes straight into
  `Vec<User<String>>`; the heterogeneous form RFC 7644 §3.4.3 allows at the
  root `/.search` endpoint is `ListResponse<Resource<String>>`. Previously the
  parameter was the ID type and `Resources` was always a four-way enum a
  caller had to match through even when the endpoint returned one kind. It
  also boxed every variant to keep the enum small — `User<String>` is 992
  bytes against the enum's 16 — so a 100-user page meant 100 separate
  allocations where the typed form is one contiguous `Vec`.

  Reusing an existing type parameter for a new purpose is the dangerous part:
  `ListResponse<String>` used to compile and mean "ids are strings", and would
  have kept compiling under the new meaning while failing at runtime on
  deserialize. `R` is therefore bounded by the new `ScimResource` trait, which
  has a private supertrait, so `ListResponse<String>` is a compile error
  (E0277) rather than a runtime surprise. The sealing also fixes the
  implementing set to this crate's resources, which is the intended scope.
  Note that `cargo-semver-checks` has no lint for this class of change — the
  seal is what catches it.

- **The `serialize()` and `deserialize()` methods are gone** from `User`,
  `Group`, `EnterpriseUser`, `Schema`, `ResourceType` and
  `ServiceProviderConfig`. They wrapped `serde_json::to_string` / `from_str`
  and only remapped the error type, while hiding `to_writer`, `from_slice` and
  `from_value`; an inherent method named `deserialize` beside
  `serde::Deserialize::deserialize` shadowed the trait method. Use
  `serde_json` directly.

- **`validate()` moved to the `Validate` trait** and returns `ValidationError`
  instead of `SCIMError`. Add `use scim_v2::Validate;`. The error carries the
  SCIM **wire** path (`userName`, not `user_name`) and the RFC 7644 §3.12
  `scimType` keyword, and `ValidationError::to_http_error` builds the error
  body a server should return.

- **`Schema`, `ResourceType` and `ServiceProviderConfig` gained a `schemas`
  field.** All three carry it on the wire per RFC 7643 §§5-7, but none
  modelled it, so a present value was silently dropped and never
  round-tripped. `#[serde(default)]` keeps absence legal, which §§6-7 allow.

- **`Address` gained `value`, `display` and `primary`.** RFC 7643 §2.4 defines
  all three as common sub-attributes of every multi-valued attribute, and
  gives "the preferred mailing address" as its example of `primary`; §4.1.2's
  listing for `addresses` simply omits them. JumpCloud sends
  `addresses[].primary`, and it was being dropped.

- **`MemberType` gained an `Other(String)` variant** and is
  `#[non_exhaustive]`. RFC 7643 §2.2 defines `canonicalValues` as "a
  collection of *suggested* canonical values that MAY be used", so a provider
  may send a `members.type` outside {User, Group} — which previously failed
  deserialization of the entire enclosing payload, losing a whole
  `ListResponse` to one unrecognised member. Unknown labels now round-trip
  verbatim, and matching is case-insensitive per the schema's
  `caseExact: false`. Because the enum now has a non-unit variant, a numeric
  cast like `MemberType as isize` no longer compiles.

- **`SCIMError`, `FilterActionError`, `Resource` and `MemberType` are
  `#[non_exhaustive]`.** After 1.0 an added variant is a breaking change, so
  each public enum was given a deliberate answer. The enums that mirror RFC
  7644's grammar — `Filter`, `AttrExp`, `ValFilter`, `CompareOp`, `CompValue`,
  `PatchPath`, `PatchOperation`, `OperationTarget`, `MaybeFilter` — stay
  exhaustive on purpose, because the RFC closes those sets and an exhaustive
  `match` is worth having. The model *structs* are deliberately not
  `#[non_exhaustive]`, since that would forbid `..Default::default()`
  downstream; `User` was audited against the embedded RFC 7643 §4.1 schema and
  carries every attribute.

- **`SearchRequest::excluded_attributes` is now `pub`.** It never was, so no
  caller could set it.

### Added

- **Feature flags `filter`, `models` and `schemas`, all on by default**, so an
  existing consumer sees no change. Turning off `filter` drops eight crates —
  `lalrpop-util`, `fluent-uri`, `regex-automata`, `regex-syntax`,
  `aho-corasick`, `borrow-or-share`, `ref-cast`, `ref-cast-impl` — taking the
  tree from 22 to 14 and removing a regex engine from the supply chain. For a
  SCIM server that has its own resource model and wants only the grammar:
  `default-features = false, features = ["filter"]`.
- `ScimResource`, a sealed trait naming the resources that may appear in a
  `ListResponse`, with `schema_urn()`.
- `Validate`, `ValidationError` and `ValidationErrorKind`, re-exported at the
  crate root.
- `schema_urns::ERROR`, `SERVICE_PROVIDER_CONFIG`, `BULK_REQUEST` and
  `BULK_RESPONSE`.

### Fixed

- `EnterpriseUser::validate` demanded `employeeNumber`, `costCenter`,
  `organization`, `division`, `department` and `manager`, so it rejected every
  conformant extension that left any of them unset. RFC 7643 §4.3 defines no
  REQUIRED attribute, and all six are `required: false` in the schema this
  crate embeds. The module had no tests at all, which is why nobody noticed.
- `ServiceProviderConfig::validate` failed whenever `patch`, `bulk`, `filter`,
  `changePassword`, `sort` or `etag` reported `supported: false`. §5 makes the
  `supported` *field* required, not its value true: a server without bulk
  support correctly advertises `"bulk": {"supported": false}`. It now checks
  `authenticationSchemes`, which §5 does mark REQUIRED.
- `primary` tolerated a stringified boolean only on `Role`, the one place
  #45 had patched. A provider sending `"primary": "true"` on an email hit the
  identical failure. All nine multi-valued types now share the lenient
  deserializer.
- `Bulk::default()` returned `maxOperations: 1000, maxPayloadSize: 1048576`
  and `Filter::default()` returned `maxResults: 100`, while
  `ServiceProviderConfig::default()` hand-built the same structs with zeros —
  two different defaults for one type depending on the constructor. They now
  delegate, and the limits are zero, since a positive limit beside
  `supported: false` advertises a capacity the server lacks.
- `SCIMError` implements `std::error::Error`. It had a hand-written `Display`
  and no `Error` impl, so it could not be boxed as `dyn Error` or composed
  with `anyhow`. It now derives `thiserror::Error`, like the filter errors
  already did.

### Fixed (MSRV)

- `rust-version` said `1.85`, and the crate has not built on 1.85 since the
  filter parser landed: `lalrpop-util`'s entire 0.23 line declares rustc 1.86,
  and the checked-in parser is generated by 0.23.1, so dropping to 0.22 would
  mean regenerating it. Nothing checked the claim, which is what the new MSRV
  CI job is for — it failed on its first run. Now `1.86`, verified against a
  real 1.86.0 toolchain. Without `filter` the crate still builds on 1.85,
  verified likewise, but `rust-version` has to describe the default set.

### Documentation and tooling

- README rewritten: badges, the feature table, and examples for the 1.0 API.
  Its code blocks are compiled and run as doctests, so it cannot drift. Four
  lines had shipped with escaped backticks, rendering as literal `` \` `` on
  GitHub.
- `CONTRIBUTING.md`, a PR template, and issue templates for bugs, RFC
  conformance gaps and feature requests.
- CI: clippy now runs `--all-targets --all-features` (it previously saw
  neither test code nor gated items), plus new jobs for the six-configuration
  feature matrix, the declared MSRV, `cargo doc -D warnings`,
  `cargo-semver-checks` and `cargo-deny`, and coverage published as a badge.
- A `Signed commits required` check that maps each of GitHub's
  `verification.reason` codes to its specific fix, rather than reporting the
  code alone.
- Publishing to crates.io is manual (`workflow_dispatch`), defaults to a dry
  run, requires the version input to match `Cargo.toml` and a tag on the same
  commit, and runs behind a `crates-io` environment. It was previously
  triggered by any `v*.*.*` tag push.
- Dependency requirements are stated as caret requirements at major.minor
  (`serde = "1.0"`, `thiserror = "2.0"`, `lalrpop-util = "0.23"`,
  `fluent-uri = "0.4"`). Every form is a caret requirement and cargo resolves
  to the newest compatible release either way, so the patch digit only set a
  floor — a claim about the oldest API this crate compiles against — and a
  needlessly precise floor forced consumers to upgrade for no reason.
- Line coverage 88.25% → 95.78%, functions 81.15% → 90.72%, excluding the
  generated parser. `tests/fixtures.rs` now enforces that every fixture is
  documented and read by a test, which found three dead JumpCloud fixtures
  that now have round-trip tests.
## 0.5.0

Releases the RFC 7644 conformance work from #47 and #48. A minor bump rather
than a patch, because two field types changed and `^0.4` treats that as
breaking.

### Breaking Changes

- `ListResponse.items_per_page` and `ListResponse.start_index` are now
  `Option<i64>`, and so are `SearchRequest.start_index` and
  `SearchRequest.count`. RFC 7644 §3.4.2 makes the two `ListResponse`
  pagination markers REQUIRED only "when partial results are returned due to
  pagination", and §3.4.3 marks both `SearchRequest` fields OPTIONAL — so the
  crate could not deserialize the RFC's own unpaginated `ListResponse`
  example, nor an RFC-legal `SearchRequest` that omitted them. `Option<i64>`
  rather than `#[serde(default)]` to an inert `0`, so a server that really
  sent `0` stays distinguishable from one that omitted the field. Migration is
  mechanical: wrap reads in `Option`, or `.unwrap_or(1)` / `.unwrap_or(0)` at
  the call site.

### Added

- `ListResponse::validate()`, matching the per-model convention. Checks that
  `schemas` is present, and that a partial page — fewer entries in `Resources`
  than `totalResults` — carries both pagination markers, which §3.4.2 requires
  in exactly that case. The `skip_serializing_if` on those fields otherwise
  makes a non-conformant response expressible.
- A corpus of 22 RFC 7644 sample payloads and 5 sanitized provider responses
  under `src/test_data/`, each documented with its RFC section in
  `src/test_data/README.md`, plus round-trip tests over them.

### Fixed

- Unassigned `Option` fields are consistently omitted on serialize rather than
  emitted as `null`: `Name.formatted`, and four fields on `EnterpriseUser`.
  RFC 7643 §2.5 treats `null` and omitted as equivalent in state and permits
  the compact form, and the rest of the crate already did this. A fully-unset
  `Name` now serializes to `{}` rather than `{"formatted":null}`. Thanks to
  @travipross for both #47 and #48.

## 0.4.2

### Fixed
- `Role.primary` may again be omitted on the wire and deserializes to `None`. The lenient bool deserializer added in 0.4 suppressed serde's implicit `Option` default, so a `Role` without a `primary` key (as sent by GitHub Enterprise, e.g. `{"value": "enterprise_owner"}`) failed with a `missing field` error. Pairing `deserialize_with` with `#[serde(default)]` restores the 0.3 behaviour while keeping the stringified-bool leniency Entra requires.

## 0.4.1

### Fixed
- `ListResponse.Resources` may now be omitted on the wire and deserializes to an empty list. Per RFC 7644 §3.4.2, `Resources` is REQUIRED only when `totalResults` is non-zero, so responses for empty result sets no longer fail to deserialize.

## 0.4

### Security
- Reject SCIM filter and PATCH-path strings whose parsed AST exceeds `filter::MAX_FILTER_DEPTH` (64). Previously, deeply nested inputs like `not (not (… (title pr) …))` or long `and`/`or` chains would produce an AST that overflowed the stack on subsequent `Display` / `==` / `{:?}` / serde / drop, allowing a remote DoS. Over-deep inputs now return `ParseError::User(FilterActionError::DepthExceeded(_))`.

### Breaking Changes
- Refactor `PatchOperation` into a tagged enum (`Add`, `Remove`, `Replace`) with `OperationTarget` variants (`WithPath`, `WithoutPath`), replacing the old `PatchOperations` struct. Paths are now parsed as `PatchPath` filter expressions instead of raw strings.
- Parameterize ID types on `User`, `Group`, `Member`, `Resource`, and `ListResponse` (default type parameter is `String`, so unparameterized usage is unchanged)
- `Member.type` changed from `Option<String>` to `Option<MemberType>` enum
- `SearchRequest` and `ListQuery` are now generic over the filter field type (`SearchRequest<F = Filter>` / `ListQuery<F = Filter>`). Unparameterized usage is source-compatible; explicit turbofish on a bare generic name (e.g. `ListQuery::<_>::default()`) may need adjustment.
- Remove `TryFrom<&str>` impls on `User` and `Group`; use `serde_json::from_str` directly
- Remove `Default` impls on `Group`, `ListResponse`, and `PatchOp`

### Added
- SCIM filter expression parser (`filter` module, RFC 7644 §3.4.2.2); `SearchRequest` and `ListQuery` now include an optional `filter` field
- `filter::MaybeFilter` deserialize-only wrapper plus `TolerantListQuery` / `TolerantSearchRequest` aliases (= `ListQuery<MaybeFilter>` / `SearchRequest<MaybeFilter>`) so servers can recover malformed filter input and return an RFC 7644 §3.12 `invalidFilter` response without losing `start_index`, `count`, and other fields
- Case-insensitive deserialization of `PatchOperation` ops and `MemberType` for Entra compatibility
- Lenient `bool` deserialization (accepts `"true"`/`"false"` strings) on fields like `Role.primary`

## 0.3

- Add `externalId` to user and group entities
- **Breaking** - Uses raw identifiers, converting `type_` to `r#type`
