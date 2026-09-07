# CHANGELOG

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
