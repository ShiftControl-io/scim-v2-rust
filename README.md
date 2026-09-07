# SCIM v2

[![CI](https://github.com/ShiftControl-io/scim-v2-rust/actions/workflows/build.yml/badge.svg)](https://github.com/ShiftControl-io/scim-v2-rust/actions/workflows/build.yml)
[![Test Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/shiftcontrol-dan/f1163c7aabc6c706464e59a0e10c861e/raw/scim_v2_coverage.json)](#)
[![crates.io](https://img.shields.io/crates/v/scim_v2.svg)](https://crates.io/crates/scim_v2)
[![docs.rs](https://img.shields.io/docsrs/scim_v2)](https://docs.rs/scim_v2)
[![MSRV](https://img.shields.io/badge/MSRV-1.86-blue)](https://blog.rust-lang.org/2025/04/03/Rust-1.86.0.html)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Models, parsers and validators for the System for Cross-domain Identity
Management (SCIM) 2.0 protocol — [RFC 7642](https://www.rfc-editor.org/rfc/rfc7642),
[RFC 7643](https://www.rfc-editor.org/rfc/rfc7643) and
[RFC 7644](https://www.rfc-editor.org/rfc/rfc7644).

## Scope

This crate is deliberately narrow. It models the wire format, parses the two
grammars the RFC defines, and checks the attributes the RFC marks REQUIRED.

- **Resources** — `User`, `Group`, `EnterpriseUser`, `Schema`, `ResourceType`,
  `ServiceProviderConfig`.
- **Protocol messages** — `ListResponse`, `SearchRequest`, `ListQuery`,
  `PatchOp`, `ScimHttpError`.
- **Filter and PATCH-path parsing** — the full RFC 7644 §3.4.2.2 filter
  grammar and the §3.5.2 PATCH path rule, with a depth guard.
- **Validation** — the `Validate` trait, reporting failures by SCIM wire path
  so a server can echo them in an RFC 7644 §3.12 response.

It performs no I/O, evaluates no filter against storage, and does not wrap
`serde` — use `serde_json` directly for that. Parsing a filter gives you the
AST; mapping that AST onto your storage is your call.

## Installation

```toml
[dependencies]
scim_v2 = "1"
```

### Feature flags

All three are on by default.

| Feature | Provides |
|---------|----------|
| `filter` | the filter and PATCH-path parsers |
| `models` | every resource and protocol message |
| `schemas` | the embedded RFC 7643 schema definitions and `get_schemas` |

Turning off `filter` drops eight crates — `lalrpop-util`, `fluent-uri`,
`regex-automata`, `regex-syntax`, `aho-corasick`, `borrow-or-share`,
`ref-cast`, `ref-cast-impl` — taking the tree from 22 to 14 and removing a
regex engine from the supply chain. A SCIM server that has its own resource
model and wants only the grammar:

```toml
scim_v2 = { version = "1", default-features = false, features = ["filter"] }
```

`SearchRequest`, `ListQuery` and `PatchOp` each carry a parsed filter or PATCH
path, so they need both `models` and `filter`.

The MSRV is **1.86**, set by `lalrpop-util`, whose whole 0.23 line requires it.
With `filter` off the crate builds on 1.85 (edition 2024's own floor), but
`rust-version` has to describe the default feature set.

## Usage

### Deserializing a resource

Every model is a plain `serde` type, so use `serde_json` directly.

```rust
use scim_v2::models::user::User;

let json = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"jdoe@example.com"}"#;
let user: User<String> = serde_json::from_str(json)?;
assert_eq!(user.user_name, "jdoe@example.com");
# Ok::<(), serde_json::Error>(())
```

### Checking RFC-required attributes

Nearly every SCIM attribute is optional, so the models make almost everything
`Option` or defaulted. `Validate` carries the checks `serde` cannot express,
and names the offending attribute by its wire path — `userName`, not
`user_name` — which is what a server needs for its error response.

```rust
use scim_v2::{Validate, models::user::User};

let user = User::<String> {
    schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
    user_name: String::new(),
    ..Default::default()
};

let err = user.validate().unwrap_err();
assert_eq!(err.path(), "userName");
assert_eq!(err.scim_type(), "invalidValue");

// And the RFC 7644 §3.12 body to return:
let body = err.to_http_error("400");
assert_eq!(body.scim_type.as_deref(), Some("invalidValue"));
```

### Reading a list response

`ListResponse` is generic over the resource. When the endpoint returns one
kind — as `GET /Users` does — name it, and the resources deserialize
directly with no enum to match through.

```rust
use scim_v2::{Validate, models::{others::ListResponse, user::User}};

let body = r#"{
  "schemas": ["urn:ietf:params:scim:api:messages:2.0:ListResponse"],
  "totalResults": 1,
  "startIndex": 1,
  "itemsPerPage": 1,
  "Resources": [
    {"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"}
  ]
}"#;

let list: ListResponse<User<String>> = serde_json::from_str(body)?;
list.validate()?;
assert_eq!(list.resources[0].user_name, "bjensen");
# Ok::<(), Box<dyn std::error::Error>>(())
```

For a heterogeneous page — which RFC 7644 §3.4.3 allows when querying the root
`/.search` endpoint — use `ListResponse<Resource<String>>` and match on the
`Resource` variant.

### Multi-valued attributes

RFC 7643 §2.5 makes an unassigned attribute, an explicit `null`, and an empty
array equivalent in state, so multi-valued attributes are `Vec<T>` rather than
`Option<Vec<T>>`. All three wire forms deserialize to an empty `Vec`, and an
empty `Vec` is omitted on serialize rather than emitted as `null` or `[]`.

```rust
use scim_v2::models::user::User;

for body in [
    r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"b"}"#,
    r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"b","emails":null}"#,
    r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"b","emails":[]}"#,
] {
    let user: User<String> = serde_json::from_str(body)?;
    assert!(user.emails.is_empty());
}
# Ok::<(), serde_json::Error>(())
```

### Parsing a SCIM filter

`Filter` implements `FromStr`, so `.parse()` is all you need.

```rust
use scim_v2::filter::Filter;

let filter: Filter = r#"userName eq "bjensen" and title pr"#.parse()?;

// The AST is yours to walk. Rendering it back gives an equivalent filter.
assert!(matches!(filter, Filter::And(_, _)));
println!("{filter}");

// A malformed filter is an error, not a panic.
assert!("userName garbage".parse::<Filter>().is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

You get the parsed expression; **evaluating it against your storage is your
job**. That split is deliberate — the grammar is the fiddly, spec-bound part,
and how a filter maps onto SQL, LDAP or an in-memory index is specific to your
server. What this crate saves you is the grammar.

Filters are rejected if their AST exceeds `filter::MAX_FILTER_DEPTH` (64),
whether they arrive via `.parse()` or deserialization. Without that bound,
pathologically nested input like `not (not (… (title pr) …))` builds an AST
that overflows the stack on the *next* `Display`, `==`, `{:?}`, serialize or
drop — a remote DoS with no bad allocation in sight.

## For SCIM servers

### Tolerant filter parsing

A client that sends a malformed `filter=` should get RFC 7644 §3.12
`invalidFilter` back, not a generic 400 — but if the filter fails to parse
during deserialization, the whole query envelope fails with it and you lose
`startIndex` and `count` along the way.

`TolerantListQuery` and `TolerantSearchRequest` keep the envelope. Deserialize
into the tolerant variant, then `.into_strict()` and map the resulting
`InvalidFilterError` onto your error response; what you hold afterwards is a
`StrictListQuery` / `StrictSearchRequest` with a fully-parsed `Filter`.

```rust
use scim_v2::models::others::{StrictListQuery, TolerantListQuery};

fn handle(body: &str) -> Result<StrictListQuery, MyScimError> {
    let tolerant: TolerantListQuery = serde_json::from_str(body)
        .map_err(MyScimError::bad_request)?;

    tolerant.into_strict().map_err(|e| {
        // e: scim_v2::filter::InvalidFilterError { raw, error }
        MyScimError::invalid_filter(e.raw, e.error.to_string())
    })
}
# struct MyScimError;
# impl MyScimError {
#     fn bad_request<E: std::error::Error>(_: E) -> Self { MyScimError }
#     fn invalid_filter(_raw: String, _detail: String) -> Self { MyScimError }
# }
```

If you need to inspect the variant directly (for example, to log the raw input
without aborting), match on `MaybeFilter` before converting:

```rust
use scim_v2::filter::MaybeFilter;
use scim_v2::models::others::TolerantListQuery;

let query: TolerantListQuery =
    serde_json::from_str(r#"{"filter": "userName garbage", "count": 10}"#)
        .expect("query envelope still parses");

match &query.filter {
    Some(MaybeFilter::Valid(f)) => println!("filter = {}", f),
    Some(MaybeFilter::Invalid(e)) => println!("return invalidFilter for {:?}: {}", e.raw, e.error),
    None => println!("no filter supplied"),
}
```

### Handling a SCIM PATCH

```rust
use scim_v2::models::others::{PatchOp, PatchOperation, OperationTarget};

let body = r#"{
  "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
  "Operations": [
    { "op": "replace", "path": "userName", "value": "new@example.com" }
  ]
}"#;

let patch: PatchOp = serde_json::from_str(body).expect("valid PatchOp");
for op in &patch.operations {
    match op {
        PatchOperation::Add(OperationTarget::WithPath { path, value }) =>
            println!("add @ {path}: {value}"),
        PatchOperation::Replace(OperationTarget::WithPath { path, value }) =>
            println!("replace @ {path}: {value}"),
        PatchOperation::Remove { path, .. } =>
            println!("remove @ {path}"),
        _ => {}
    }
}
```

## Using custom ID types

`User`, `Group`, `Member` and `Resource` are generic over their ID type,
defaulting to `String`. Substitute `uuid::Uuid`, `i64`, or any
`Serialize + DeserializeOwned` type.

`ListResponse` is generic over the *resource*, not the ID type, so the ID type
travels inside it: `ListResponse<User<Uuid>>`.

```rust
use scim_v2::models::user::User;
use uuid::Uuid;

let user: User<Uuid> = User {
    user_name: "jdoe@example.com".to_string(),
    ..Default::default()
};
```

For more examples and usage details, refer to the documentation of each function and struct.

## Regenerating the filter parser

The SCIM filter parser (`src/filter_parser.rs`) is pre-generated from `src/filter_parser.lalrpop`
using [LALRPOP](https://github.com/lalrpop/lalrpop). The generated file is committed
to the repository so no build script is required.

If you modify `src/filter_parser.lalrpop`, regenerate `src/filter_parser.rs` by running:

```sh
cargo install lalrpop
lalrpop src/filter_parser.lalrpop
```

Commit both `src/filter_parser.lalrpop` and the updated `src/filter_parser.rs` together.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) — note that
**all commits must be signed**; CI rejects unsigned or unverifiable commits.

## License

[MIT](https://choosealicense.com/licenses/mit/)
