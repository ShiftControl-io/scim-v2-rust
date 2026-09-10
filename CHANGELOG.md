# CHANGELOG

## 1.0.0

First stable release. The public API is covered by SemVer from here and
`cargo-semver-checks` runs on every PR. Every breaking change is listed
below; 1.0 was the one moment they were free.

### Breaking changes

- **`Filter::And`/`Or` and `ValFilter::And`/`Or` are n-ary**:
  `And(Operands<Filter>)` instead of `And(Box<Filter>, Box<Filter>)`. The
  parser flattens same-operator chains, so `a or b or c` is one node and a
  hundred-id lookup no longer hits the depth cap. `Operands<T>` holds at least
  two operands by construction and derefs to a slice. Build filters with
  `Filter::and`, `or`, `all` and `any`.
- **Multi-valued attributes are `Vec<T>`, not `Option<Vec<T>>`** (all 18).
  RFC 7643 §2.5 makes absent, `null` and `[]` one state, so all three
  deserialize to an empty `Vec`. An empty attribute serializes as `[]`, since
  RFC 7644 §3.5.1 gives `[]` the meaning "clear all values" that omission
  lacks; `compact::Compact(&value)` omits them in a response. Suggested
  by @travipross in #48.
- **`ListResponse` is generic over the resource, not the ID type**:
  `ListResponse<User<String>>` deserializes straight into `Vec<User<String>>`,
  and the heterogeneous form is `ListResponse<Resource<String>>`. `R` is bound
  by the sealed `ScimResource` trait, so the old `ListResponse<String>` is a
  compile error rather than a runtime surprise.
- **`validate()` moved to the `Validate` trait** (`use scim_v2::Validate;`)
  and returns `ValidationError`, which carries the SCIM wire path and the
  RFC 7644 §3.12 `scimType`; `to_http_error()` builds the error body.
- **`User<T>` and `Group<T>` need `T: Display` to validate**, so a response's
  `id` can be checked for RFC 7643 §3.1's "non-empty". `String`,
  `uuid::Uuid` and the integers qualify.
- **`OperationTarget::WithPath.value` is `Option<Value>`**, so a PATCH
  operation that omitted `value` is distinguishable from one that sent an
  explicit `null` instead of both collapsing to null. RFC 7644 §3.5.2.1 makes
  the member REQUIRED on an `add`, which `PatchOp::validate` now reports.
- **`Compact<T>` requires the sealed `Compactable`**: resources and lists,
  never `PatchOp` or `SearchRequest`, whose `[]` means clear-all.
- **The `serialize()` / `deserialize()` wrappers are gone.** Use `serde_json`.
- **`Schema`, `ResourceType` and `ServiceProviderConfig` gained `schemas`**,
  and **`Address` gained `value`, `display` and `primary`** (RFC 7643 §2.4).
  All were dropped on the wire before.
- **`MemberType` gained `Other(String)`** and compares case-insensitively, so
  an unknown `members.type` no longer fails a whole page (RFC 7643 §7 calls
  canonical values *suggested*).
- **`SCIMError`, `FilterActionError`, `Resource` and `MemberType` are
  `#[non_exhaustive]`.** The grammar enums stay exhaustive on purpose.
- **`SearchRequest::excluded_attributes` is `pub`.** It never was.
- **`meta.created` and `meta.lastModified` are `ScimDateTime`, not `String`.**
  RFC 7643 §2.3.5 requires a valid `xsd:dateTime` with both a date and a time,
  and §3.1 makes `meta` provider-assigned, so a malformed timestamp is now
  unconstructable rather than caught by whoever consumes your responses. The
  accepted grammar is exactly XSD 1.1 §3.3.7.2, day-of-month rule included;
  no date-time dependency is taken, and the module docs say what that costs.
  Suggested by @sidrubs in #49.
- **`utils::case` is `case_insensitive` and `utils::compact` is `compact`**,
  both at the crate root, with `CaseInsensitive` and `Compact` re-exported
  there too. The old paths made a reader parse `utils` to learn nothing.
- **The `case-insensitive` feature is gone**, and the module it gated is
  always compiled. It never selected any behaviour — the choice is made per
  call site by whether you call `case_insensitive::from_str` — so the flag
  bought nothing and cost a round of reasoning about feature unification.
  Raised by @Paul-E in #51.

### Added

- **Feature flags** `filter`, `models` and `schemas`, all on by default and
  all additive. `default-features = false, features = ["filter"]` gives a
  server the grammar alone and drops eight crates.
- **Direction-aware validation**: `validate_as(Context)` with
  `CreateRequest`, `ReplaceRequest` and `Response`; `Valid<T>` as proof that
  a value passed; `Strict<T, M>` to deserialize and validate in one step.
- **Case-insensitive attribute names** (RFC 7643 §2.1) through `case_insensitive`
  and `CaseInsensitive<T>`. Two spellings of one attribute in one object are
  an `AmbiguousKey` error, and the rejected value is left untouched.
- **Filter limits**: `MAX_FILTER_DEPTH` (64) and `MAX_FILTER_TERMS` (1024),
  enforced while parsing so a hostile filter is rejected before it is
  allocated; `FilterActionError::TooManyTerms`. `tests/filter_budget_alloc.rs`
  measures the peak.
- `sortBy` and `sortOrder` on `SearchRequest` and `ListQuery` (RFC 7644
  §3.4.2.3), plus `effective_count()` / `effective_start_index()` applying
  §3.4.2.4 Table 6. `ListQuery` validates the same sort and §3.9 selection
  rules as `SearchRequest`, so a `GET` and a `POST /.search` agree on what a
  client may ask for, and its `Default` omits both attribute selectors rather
  than sending each as an empty string.
- `ScimType`, the RFC 7644 §3.12 keywords; `ScimHttpError.scim_type` is an
  `Option<ScimType>`.
- `Validate` for `SearchRequest`, `PatchOp`, `ScimHttpError`, `Schema`,
  `AuthenticationScheme`, and `ListResponse` (the envelope and every
  resource on the page).
- `FilterActionError::InvalidAttrName`, naming the RFC 7644 §3.4.2.2
  `ATTRNAME` rule a malformed path segment broke.
- `ValidationError::under(parent)` for nested errors; `require_schema_urn`
  and `at_most_one_primary` helpers, re-exported at the root.
- `schema_urns::ERROR`, `SERVICE_PROVIDER_CONFIG`, `BULK_REQUEST` and
  `BULK_RESPONSE`.
- Property-based round-trip tests over generated resources.

### Fixed

Conformance, each checked against the RFC text in `docs/rfcs/`:

- `EnterpriseUser::validate` demanded six attributes RFC 7643 §4.3 leaves
  optional; `ServiceProviderConfig::validate` rejected `"supported": false`,
  which §5 permits.
- `schemas` must be non-empty, unique and name the resource's URN (§3), and
  a present enterprise extension needs its URN declared. `Schema` alone may
  omit `schemas`, as the RFC's own §8.7 representations do.
- `primary: true` at most once per multi-valued attribute (§2.4), on all nine
  that carry it; `AuthenticationScheme.type`, `name` and `description`
  REQUIRED (§5); `PatchOp` needs one or more operations (RFC 7644 §3.5.2);
  `SearchRequest` rejects `sortOrder` without `sortBy`, `attributes`
  together with `excludedAttributes` (§3.9), and a `sortBy` or any
  attribute-selection name that is not in standard attribute notation
  (§§3.4.2.5, 3.4.3, 3.10); `ResourceType.schemaExtensions[].schema`
  REQUIRED (RFC 7643 §6).
- A negative `count` or a `startIndex` below 1 is interpreted, not rejected
  (RFC 7644 §3.4.2.4 Table 6).
- An `add` or `replace` operation with a path and no `value` passed
  validation; §3.5.2.1 makes the member REQUIRED and §3.5.2.3 requires it of
  a `replace` twice over.
- `ListResponse::validate` checks `totalResults` against the page, both
  pagination markers on a short page, each resource's declared schema
  against the type it was parsed as, and each resource's own rules. A
  `Resources` member absent while `totalResults` is non-zero is refused
  during deserialization, where the RFC's REQUIRED can still be seen.

Robustness:

- Stringified booleans such as `"True"` are accepted on `active` and every
  `primary`; `null` is accepted for every multi-valued attribute, including
  `Resources` and `authenticationSchemes`.
- A deep `not (` chain with one trailing token aborted the process, and a
  long flat `or` chain was rejected as too deep. Both are gone with the n-ary
  AST and the parse-time limits.
- `case_insensitive` rewrote the RFC's own `Resources` and `Operations` keys, and
  resolved colliding keys last-write-wins.
- `Bulk` and `Filter` had different defaults depending on the constructor.
- `SCIMError` implements `std::error::Error`.
- `rust-version` is `1.86`, the real floor set by `lalrpop-util` 0.23; it
  claimed 1.85 and nothing checked it.

### Security and release pipeline

- `publish.yml` interpolated `${{ inputs.version }}` into a `run:` body in
  the job holding `CRATES_TOKEN`. Inputs now arrive through `env:` and are
  shape-checked. Publishing is manual, dry-run by default, behind a
  `crates-io` environment, and requires the version to match `Cargo.toml`
  and a tag on the same commit.
- The tag guard, the publish-time verification, `cargo-deny`'s `[bans]` and
  the signed-commits check each had a way to pass without checking anything.

### Documentation and tooling

- `lib.rs` and the README open with a quick start (a server handling
  `POST /Users`, a client reading `GET /Users`, a filter parse), and a
  "deserializing does not validate" section says when to validate and what
  happens if you do not.
- CI runs clippy `--all-targets --all-features`, a nine-configuration
  feature matrix, the MSRV, `cargo doc -D warnings`, `cargo-semver-checks`,
  `cargo-deny`, a coverage badge and a signed-commits check; a weekly job
  catches upstream drift.
- `CONTRIBUTING.md` carries the review rules this release taught, with PR
  and issue templates. Unit tests live in `src/<module>/tests.rs`. Line
  coverage 88% → 95%. Dependencies are caret requirements at major.minor.

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
