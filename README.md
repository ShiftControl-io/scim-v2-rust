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

All four are on by default, and all four are additive — a feature only ever
compiles more, never changes what an existing call does.

| Feature | What it does |
|---------|--------------|
| `filter` | the filter and PATCH-path parsers |
| `models` | every resource and protocol message |
| `schemas` | the embedded RFC 7643 schema definitions and `get_schemas` |
| `case-insensitive` | `utils::case`: canonicalise attribute-name case before deserializing, per RFC 7643 §2.1 |

Wire-behaviour choices — lenient booleans, compact output, case folding — are
**types**, not features: `Compact<&T>`, `CaseInsensitive<T>`, `Strict<T, M>`.
Cargo unifies features across the whole dependency graph, so a feature switch
would let any transitive crate change what every other consumer puts on the
wire; a wrapper type is a decision the caller makes at the call site.

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
`rust-version` is a package-wide floor that Cargo enforces before compiling
anything and is not conditional on features, so 1.86 applies to every
configuration — including `--no-default-features`, where the *code* would
compile on 1.85. Cargo has no way to express a per-feature MSRV, so 1.86 is
the floor in practice.

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

### Validation — read this before shipping a server

**Deserializing a model does not validate it.** The parsers are deliberately
lenient so that a real provider's payload can always be read, and that means
`serde_json::from_str::<User>(..)` will happily give you a `User` with an empty
`userName`, two `primary: true` emails, an `id` the client had no business
sending, or a `schemas` naming the wrong resource. If you store that and later
build a response from it, you ship a non-conformant response; if it carried a
`password`, you may echo it back.

Nearly every SCIM attribute is optional, so `serde` cannot express the
RFC's REQUIRED rules on its own. `Validate` does, and it names the offending
attribute by its **wire** path — `userName`, not `user_name` — so a server can
return it in the RFC 7644 §3.12 error.

There are three ways to run it, from least to most enforced:

1. `value.validate()` — the direction-agnostic checks. Easy to forget.
2. `value.validate_as(Context::CreateRequest)` — adds the rules that depend on
   direction: `id` MUST NOT be on a create, MUST be on a response, `password`
   MUST NOT be on a response.
3. `Valid<T>` and `Strict<T, M>` — make forgetting impossible. A handler that
   takes `Valid<User>` cannot receive an unvalidated one, and `Strict<User,
   CreateRequest>` rejects a non-conformant body at deserialization.

For a server, use 3. For a client reading responses, 1 or 2 is usually enough,
since a non-conformant *provider* is theirs to fix, not yours.

```rust
use scim_v2::{Validate, models::{errors::ScimType, user::User}};

let user = User::<String> {
    schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
    user_name: String::new(),
    ..Default::default()
};

let err = user.validate().unwrap_err();
assert_eq!(err.path(), "userName");
assert_eq!(err.scim_type_str(), "invalidValue");

// And the RFC 7644 §3.12 body to return:
let body = err.to_http_error();
assert_eq!(body.scim_type, Some(ScimType::InvalidValue));
```

The enforced form, which is what a server's request handler should take:

```rust
use scim_v2::{Context, CreateRequest, Strict, Valid, models::user::User};

// Parse and validate in one step; a bad body never becomes a `User` at all.
let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"}"#;
let valid: Valid<User<String>> =
    serde_json::from_str::<Strict<User<String>, CreateRequest>>(body)?.into_valid();
assert_eq!(valid.user_name, "bjensen");

// A create body that carries an id is rejected at the door (RFC 7643 §3.1).
let with_id = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen","id":"7"}"#;
assert!(serde_json::from_str::<Strict<User<String>, CreateRequest>>(with_id).is_err());

// Or validate a value you already hold, for a given direction.
let user = valid.into_inner();
assert!(Valid::new(user.clone(), Context::CreateRequest).is_ok());
assert!(Valid::new(user, Context::Response).is_err()); // a response needs an id
# Ok::<(), serde_json::Error>(())
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
`Option<Vec<T>>`, and all three wire forms deserialize to an empty `Vec`.

On the way out an empty `Vec` is emitted as `[]`, never as `null` and not by
omission. That is deliberate: RFC 7644 §3.5.1 says a client "MAY specify … an
empty array `[]` for a multi-valued attribute, to clear all values", while an
omitted attribute is merely "not asserted" and the server may keep or default
it. Since these models are request bodies as well as representations, `[]` is
what keeps a conformant clear-all expressible. A server serializing a response
that wants the compact form wraps the value in `utils::compact::Compact(&user)`
— a wrapper rather than a feature, because Cargo features unify across the
dependency graph and a transitive crate could otherwise change what you put on
the wire.

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

// The AST is yours to walk; `and`/`or` are n-ary, so a chain is one node.
// Rendering it back gives an equivalent filter.
assert!(matches!(&filter, Filter::And(items) if items.len() == 2));
println!("{filter}");

// A malformed filter is an error, not a panic.
assert!("userName garbage".parse::<Filter>().is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

You get the parsed expression; **evaluating it against your storage is your
job**. That split is deliberate — the grammar is the fiddly, spec-bound part,
and how a filter maps onto SQL, LDAP or an in-memory index is specific to your
server. What this crate saves you is the grammar.

Filters are rejected if they nest deeper than `filter::MAX_FILTER_DEPTH` (64)
or hold more than `filter::MAX_FILTER_TERMS` (1024) attribute expressions,
whether they arrive via `.parse()` or deserialization. Both limits are enforced
*while* parsing, so an input that crosses one is rejected at that point with the
rest unread, and peak memory for a hostile filter is bounded by the limits rather
than by its length. Without the depth bound, pathologically nested input like
`not (not (… (title pr) …))` builds an AST that overflows the stack on the *next*
`Display`, `==`, `{:?}`, serialize or drop — a remote DoS with no bad allocation
in sight. `and`/`or` are n-ary (`Filter::And(Operands<Filter>)`, at least two
operands by construction), so a long flat chain such as a hundred-id `or` lookup
is depth 2 and parses; only the term bound applies to it.

## For SCIM servers

### Accepting requests

Take `Strict<T, CreateRequest>` (or `ReplaceRequest`) in your handlers, as
shown under [Validation](#validation--read-this-before-shipping-a-server).
A body that fails the RFC's rules never becomes a `T`, and the error names the
attribute so you can return it in `scimType: invalidValue`. If a peer's
attribute-name casing cannot be trusted, wrap in `CaseInsensitive<..>` first
(RFC 7643 §2.1 makes names case-insensitive; most clients are camelCase, not
all).

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
