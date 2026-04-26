# CHANGELOG

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
