# SCIM v2

[![CI](https://github.com/ShiftControl-io/scim-v2-rust/actions/workflows/build.yml/badge.svg)](https://github.com/ShiftControl-io/scim-v2-rust/actions/workflows/build.yml)
[![Test Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/shiftcontrol-dan/f1163c7aabc6c706464e59a0e10c861e/raw/scim_v2_coverage.json)](#)
[![crates.io](https://img.shields.io/crates/v/scim_v2.svg)](https://crates.io/crates/scim_v2)
[![docs.rs](https://img.shields.io/docsrs/scim_v2)](https://docs.rs/scim_v2)
[![MSRV](https://img.shields.io/badge/MSRV-1.86-blue)](https://blog.rust-lang.org/2025/04/03/Rust-1.86.0.html)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Models, parsers and validators for the System for Cross-domain Identity
Management (SCIM) 2.0 protocol. [RFC 7642](https://www.rfc-editor.org/rfc/rfc7642),
[RFC 7643](https://www.rfc-editor.org/rfc/rfc7643) and
[RFC 7644](https://www.rfc-editor.org/rfc/rfc7644) define the protocol.

## What this crate is for

SCIM is a protocol made of messages, and this crate is about those messages. It
reads a message that arrived and tells you whether the message conforms. It
builds a message to send, and refuses to build one that does not conform. A
client and a server need the same thing from opposite sides, so the same types
serve both, and `Context` states the direction a value travels in.

The goal is to apply the RFCs correctly, and correct is not always obvious. The
RFCs leave some questions open, and real providers send forms the RFCs never
describe. This crate decides those cases instead of passing the problem to you,
and the doc comment on each decision quotes the RFC sentence it rests on. Where
the decision is genuinely yours, the type hands it to you rather than choosing
for you. `Multi<T>` is the clearest example. RFC 7644 §3.5.1 makes an omitted
attribute a different instruction from a cleared one, so `Multi<T>` keeps the
two apart and your server decides what each one means.

## Quick start

This example shows a SCIM server that receives `POST /Users`. `Strict` does two
operations in one step. `Strict` deserializes the body, and validates the body
for the direction of travel. A non-conformant request does not become a `User`.
The error names the attribute that failed, by the wire path of the attribute.
The error also carries the RFC 7644 §3.12 `scimType` value. A server returns
that value in a `400` response. `Valid` is the type-level proof that a value
passed validation. `Context::Response` checks the rules that apply to the
response.

```rust
use scim_v2::{Context, CreateRequest, Strict, Valid};
use scim_v2::models::{scim_schema::Meta, user::User};

let body = r#"{
  "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
  "userName": "bjensen@example.com",
  "name": {"givenName": "Barbara", "familyName": "Jensen"},
  "emails": [{"value": "bjensen@example.com", "type": "work", "primary": true}]
}"#;

// Deserialize and validate as a create request. RFC 7643 §3.1 forbids `id`
// here; a blank `userName` or a second `primary: true` email would fail too.
let request: Valid<User<String>> =
    match serde_json::from_str::<Strict<User<String>, CreateRequest>>(body) {
        Ok(strict) => strict.into_valid(),
        Err(e) => return Err(e.into()), // answer 400 with `e.to_string()`
    };

// Store it, then build the representation to return. A response MUST carry
// `id` (§3.1) and MUST NOT carry `password` (§4.1); `Context::Response` checks both.
let mut user = request.into_inner();
user.id = Some("2819c223-7f76-453a-919d-413861904646".to_string());
user.meta = Some(Meta {
    resource_type: Some("User".to_string()),
    location: Some("https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string()),
    ..Default::default()
});
let response = Valid::new(user, Context::Response)?;
let json = serde_json::to_string(&response)?;
assert!(json.contains(r#""id":"2819c223-7f76-453a-919d-413861904646""#));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The sections [Usage](#usage) and [For SCIM servers](#for-scim-servers) below
show the other operations. These operations include a read of a list response,
a parse of a filter, and a PATCH operation.

## Scope

This crate is narrow by design. The crate models the wire format. The crate
parses the two grammars that the RFC defines. The crate checks the attributes
that the RFC marks REQUIRED.

- **Resources** — `User`, `Group`, `EnterpriseUser`, `Schema`, `ResourceType`,
  `ServiceProviderConfig`.
- **Protocol messages** — `ListResponse`, `SearchRequest`, `ListQuery`,
  `PatchOp`, `ScimHttpError`.
- **Filter and PATCH-path parsers** — the full RFC 7644 §3.4.2.2 filter
  grammar and the §3.5.2 PATCH path rule, with a depth guard.
- **Validation** — the `Validate` trait. The trait reports a failure by the
  SCIM wire path. A server returns that path in an RFC 7644 §3.12 response.

The crate performs no I/O. The crate evaluates no filter against storage. The
crate does not wrap `serde`. Use `serde_json` directly. A parse of a filter
gives you the AST. You map that AST onto your storage.

## Installation

```toml
[dependencies]
scim_v2 = "1"
```

### Feature flags

All three features are on by default. All three features are additive. A
feature only adds code to the compilation. A feature does not change what an
existing call does.

| Feature | What it does |
|---------|--------------|
| `filter` | the filter and PATCH-path parsers |
| `models` | every resource and protocol message |
| `schemas` | the embedded RFC 7643 schema definitions and `get_schemas` |

Lenient booleans and case folding are wire-behaviour choices.
These choices are **types**, not features: `CaseInsensitive<T>`
and `Strict<T, M>`. Cargo unifies features across the whole dependency graph.
A feature switch would let any transitive crate change what every other
consumer puts on the wire. A wrapper type is a decision that the caller makes
at the call site.

If you turn off `filter`, the build drops eight crates: `lalrpop-util`,
`fluent-uri`, `regex-automata`, `regex-syntax`, `aho-corasick`,
`borrow-or-share`, `ref-cast` and `ref-cast-impl`. The dependency tree goes
from 22 crates to 14 crates. The build also removes a regex engine from the
supply chain. The example below is for a SCIM server that has its own resource
model and wants only the grammar.

```toml
scim_v2 = { version = "1", default-features = false, features = ["filter"] }
```

`SearchRequest`, `ListQuery` and `PatchOp` each carry a parsed filter or a
parsed PATCH path. These three types need both `models` and `filter`.

The MSRV is **1.86**. `lalrpop-util` sets this floor, because the whole 0.23
line of `lalrpop-util` requires 1.86. `rust-version` is a package-wide floor.
Cargo enforces `rust-version` before it compiles anything, and features do not
change the floor. The floor of 1.86 applies to every configuration. One such
configuration is `--no-default-features`, where the *code* would compile on
1.85. Cargo has no way to express a per-feature MSRV. Therefore 1.86 is the
floor in practice.

## Usage

### Deserializing a resource

Every model is a plain `serde` type. Use `serde_json` directly.

```rust
use scim_v2::models::user::User;

let json = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"jdoe@example.com"}"#;
let user: User<String> = serde_json::from_str(json)?;
assert_eq!(user.user_name, "jdoe@example.com");
# Ok::<(), serde_json::Error>(())
```

### Validation — read this before shipping a server

**Deserialization of a model does not validate the model.** The parsers are
lenient by design. This design lets the crate read a real provider's payload
every time. Therefore `serde_json::from_str::<User>(..)` gives you a `User`
that holds an empty `userName`, or two `primary: true` emails, or an `id` that
the client must not send, or a `schemas` value that names the wrong resource.
If you store such a `User`, and you later build a response from it, you send a
non-conformant response. If the `User` holds a `password`, you can return the
`password` to the client.

Nearly every SCIM attribute is optional. Therefore `serde` alone cannot express
the REQUIRED rules of the RFC. `Validate` expresses these rules. `Validate`
names the attribute that failed by the **wire** path of the attribute, for
example `userName` and not `user_name`. A server returns that path in the RFC
7644 §3.12 error.

There are three ways to run `Validate`. The list below goes from the least
enforced way to the most enforced way.

1. `value.validate()` — the direction-agnostic checks. A caller can forget
   this call.
2. `value.validate_as(Context::CreateRequest)` — adds the rules that depend on
   the direction: a create request MUST NOT carry `id`, a response MUST carry
   `id`, and a response MUST NOT carry `password`.
3. `Valid<T>` and `Strict<T, M>` — these two types make the omission
   impossible. A handler that takes `Valid<User>` cannot receive an
   unvalidated `User`. `Strict<User, CreateRequest>` rejects a non-conformant
   body at deserialization.

For a server, use method 3. For a client that reads responses, method 1 or
method 2 is usually sufficient. The operator of a non-conformant *provider*
must fix the provider. The fix is not your responsibility.

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

The example below shows the enforced form. A server's request handler should
take this form.

```rust
use scim_v2::{Context, CreateRequest, Strict, Valid, models::user::User};

// Parse and validate in one step; a bad body never becomes a `User` at all.
// The body carries `userName`, the login name the client chooses (RFC 7643
// §4.1). It has no `id`: that is a separate attribute, the identifier the
// server assigns when it stores the resource (§3.1).
let body = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen"}"#;
let valid: Valid<User<String>> =
    serde_json::from_str::<Strict<User<String>, CreateRequest>>(body)?.into_valid();
assert_eq!(valid.user_name, "bjensen");

// A create body that carries an `id` is rejected at the door: the client
// "MUST NOT" choose the server's identifier (RFC 7643 §3.1).
let with_id = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"bjensen","id":"7"}"#;
assert!(serde_json::from_str::<Strict<User<String>, CreateRequest>>(with_id).is_err());

// Or validate a value you already hold, for a given direction.
let user = valid.into_inner();
assert!(Valid::new(user.clone(), Context::CreateRequest).is_ok());
// As a *response* the same value fails: a server's representation "MUST
// include a non-empty id value" (§3.1), and this one has none yet.
assert!(Valid::new(user, Context::Response).is_err());
# Ok::<(), serde_json::Error>(())
```

### Reading a list response

`ListResponse` is generic over the resource. An endpoint such as `GET /Users`
returns one kind of resource. Name that kind in the type. The resources then
deserialize directly, and you do not match through an enum.

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

RFC 7644 §3.4.3 allows a heterogeneous page for a query on the root `/.search`
endpoint. For such a page, use `ListResponse<Resource<String>>` and match on
the `Resource` variant.

### Multi-valued attributes

RFC 7643 §2.5 makes three wire forms equivalent in state: an unassigned
attribute, an explicit `null`, and an empty array. Therefore a multi-valued
attribute is a `Vec<T>` and not an `Option<Vec<T>>`. All three wire forms
deserialize to an empty `Vec`.

Serialization writes an empty `Vec` as `[]`. Serialization never writes `null`,
and never omits the attribute. This behaviour is deliberate. RFC 7644 §3.5.1
says that a client "MAY specify … an empty array `[]` for a multi-valued
attribute, to clear all values". An omitted attribute is only "not asserted",
and the server may keep the value or apply a default. These models are request
bodies as well as representations. `[]` keeps a conformant clear-all
expressible. An attribute that nobody assigned is absent, and `Multi<T>`
leaves it off the wire without any wrapper.

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

### Proxying between providers

A bridge is a service provider on the way in and a client on the way out:

```text
  Okta ──PUT──▶ [ you as service provider │ you as client ] ──PUT──▶ CyberArk
(client)           deserialize → mutate → serialize           (service provider)
```

Both roles use the same types. In a response, the three wire forms above are
equivalent. In a request, they are not. RFC 7644 §3.5.1 makes an omitted
readWrite attribute "not asserted by the client", and it gives the client a
deterministic alternative: clients "MAY specify "null" for a single-valued
attribute, or an empty array "[]" for a multi-valued attribute, to clear all
values".

`Multi<T>` keeps those two apart, so a bridge needs no wrapper and no extra
call. A message that arrives and leaves unchanged is unchanged:

```rust
use scim_v2::models::user::User;

// Okta clears the emails and says nothing about phone numbers.
let inbound = r#"{"schemas":["urn:ietf:params:scim:schemas:core:2.0:User"],"userName":"BJENSEN","emails":[]}"#;

let mut user: User<String> = serde_json::from_str(inbound)?;
user.user_name = user.user_name.to_lowercase();
let outbound = serde_json::to_value(&user)?;

assert_eq!(outbound["userName"], "bjensen");
assert_eq!(outbound["emails"], serde_json::json!([])); // the clear survives
assert!(outbound.get("phoneNumbers").is_none()); // never asserted, still absent
# Ok::<(), Box<dyn std::error::Error>>(())
```

| upstream sent | §3.5.1 meaning | `Multi<T>` state | goes downstream as |
|---|---|---|---|
| `"emails": null` | clear all values | cleared | `[]` |
| `"emails": []` | clear all values | cleared | `[]` |
| no `phoneNumbers` member | not asserted | absent | omitted |

A server applying a `PUT` asks the field directly:

```rust
# use scim_v2::models::user::User;
# let (mut stored, body) = (User::<String>::default(), User::<String>::default());
if body.emails.is_asserted() {
    stored.emails = body.emails; // set these, or clear all when empty
}
// absent: the client asserted nothing, so leave the stored value alone
```

Everything else reads as before. `Multi<T>` derefs to `[T]`, so `user.emails.len()`,
`user.emails.iter()`, `user.emails[0]` and `for email in &user.emails` are
unchanged. A reader that does not care about the distinction never sees it.

### Timestamps

`meta.created` and `meta.lastModified` are `ScimDateTime` values, not `String`
values. RFC 7643 §2.3.5 requires a valid `xsd:dateTime` that carries both a
date and a time. RFC 7643 §3.1 makes every `meta` sub-attribute readOnly and
provider-assigned. Therefore a server built on this crate is the party most
likely to write a malformed timestamp. The `ScimDateTime` type stops that
error. Every way to make a `ScimDateTime` validates the value.

```rust
use scim_v2::ScimDateTime;

let created: ScimDateTime = "2010-01-23T04:56:22Z".parse().unwrap();
assert_eq!(created.as_str(), "2010-01-23T04:56:22Z");

// Exactly the XSD 1.1 §3.3.7.2 lexical space, day-of-month rule included.
assert!("2000-02-29T00:00:00Z".parse::<ScimDateTime>().is_ok());
assert!("1900-02-29T00:00:00Z".parse::<ScimDateTime>().is_err());
assert!("2010-01-23".parse::<ScimDateTime>().is_err());

// The offset is optional in XSD, so check before handing it to an RFC 3339
// parser, which requires one.
assert!(created.has_offset());
```

`==` compares the text. This type carries the spelling of a timestamp. To find
whether two values name the same instant, use `xsd_equivalent`. To order two
values, use `xsd_partial_cmp`. Both methods implement XSD 1.1 §3.3.7.1.
`xsd_partial_cmp` answers `None` for the pairs that the spec calls
incomparable. For that reason, `ScimDateTime` has no `Ord`.

```rust
use scim_v2::ScimDateTime;

let z: ScimDateTime = "2010-01-23T04:56:22Z".parse().unwrap();
let pst: ScimDateTime = "2010-01-22T20:56:22-08:00".parse().unwrap();
assert!(z != pst); // different spelling
assert!(z.xsd_equivalent(&pst)); // same instant
```

`ScimDateTime` validates a lexical form and stops. The type does no arithmetic
and no time zone conversion. This limit is deliberate. XSD makes the offset
optional. `time`, `chrono` and `jiff` each split offset-bearing values from
offset-less values across two different types
(`OffsetDateTime`/`PrimitiveDateTime`, `DateTime<Tz>`/`NaiveDateTime`,
`Timestamp`/`civil::DateTime`). A field with any one of those types would
reject conformant input, or would invent an offset that the provider never
sent. You do the conversion. When `has_offset()` is true, the conversion is one
line: `OffsetDateTime::parse(created.as_str(), &Rfc3339)`. When `has_offset()`
is false, you make your own decision. The module docs carry the full list.

### Parsing a SCIM filter

`Filter` implements `FromStr`. Use `.parse()`.

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

You get the parsed expression. **You evaluate that expression against your
storage.** This split is deliberate. The grammar is the difficult part, and the
RFC binds it. The map from a filter onto SQL, LDAP or an in-memory index is
specific to your server. This crate does the grammar work for you.

The parser rejects a filter that nests deeper than `filter::MAX_FILTER_DEPTH`
(64). The parser also rejects a filter that holds more than
`filter::MAX_FILTER_TERMS` (1024) attribute expressions. Both rejections apply
to a filter from `.parse()` and to a filter from deserialization. The parser
enforces both limits *while* it parses. The parser rejects an input that
crosses a limit at that point, and does not read the rest of the input. The
limits bound the peak memory for a hostile filter, and the length of the filter
does not. Without the depth bound, pathologically nested input such as
`not (not (… (title pr) …))` builds an AST. That AST overflows the stack on the
*next* `Display`, `==`, `{:?}`, serialize or drop operation. The result is a
remote DoS, and no bad allocation is visible in the input. `and` and `or` are
n-ary: `Filter::And(Operands<Filter>)` holds at least two operands by
construction. Therefore a long flat chain, such as an `or` lookup over a
hundred ids, has a depth of 2 and parses. Only the term bound applies to such a
chain.

## For SCIM servers

### Accepting requests

Take `Strict<T, CreateRequest>` (or `ReplaceRequest`) in your handlers. The
section [Validation](#validation--read-this-before-shipping-a-server) shows
this form. A body that fails the rules of the RFC does not become a `T`. The
error names the attribute, and you return that name in
`scimType: invalidValue`. If you cannot trust the case of a peer's attribute
name, wrap the type in `CaseInsensitive<..>` first. RFC 7643 §2.1 makes an
attribute name case-insensitive. Most clients use camelCase, but not all
clients do.

### Tolerant filter parsing

A client that sends a malformed `filter=` should get the RFC 7644 §3.12
`invalidFilter` error back, and not a generic 400. But a filter that fails to
parse during deserialization makes the whole query envelope fail. You then lose
`startIndex` and `count`.

`TolerantListQuery` and `TolerantSearchRequest` keep the envelope. Deserialize
the body into the tolerant variant. Then call `.into_strict()`, and map the
`InvalidFilterError` onto your error response. You then hold a
`StrictListQuery` or a `StrictSearchRequest` with a fully-parsed `Filter`.

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

You can also examine the variant directly, for example to log the raw input and
continue. Match on `MaybeFilter` before the conversion.

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
use scim_v2::Validate;
use scim_v2::models::others::{PatchOp, PatchOperation, OperationTarget};

let body = r#"{
  "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
  "Operations": [
    { "op": "replace", "path": "userName", "value": "new@example.com" }
  ]
}"#;

let patch: PatchOp = serde_json::from_str(body).expect("valid PatchOp");
patch.validate()?; // an `add` with no `value` is rejected here (RFC 7644 §3.5.2.1)

for op in &patch.operations {
    match op {
        // `value` is `Option<Value>`: `None` is a member the body omitted,
        // `Some(Value::Null)` an explicit JSON null.
        PatchOperation::Add(OperationTarget::WithPath { path, value }) =>
            println!("add @ {path}: {value:?}"),
        PatchOperation::Replace(OperationTarget::WithPath { path, value }) =>
            println!("replace @ {path}: {value:?}"),
        PatchOperation::Remove { path, .. } =>
            println!("remove @ {path}"),
        _ => {}
    }
}
# Ok::<(), scim_v2::ValidationError>(())
```

## Using custom ID types

`User`, `Group`, `Member` and `Resource` are generic over the ID type. The
default ID type is `String`. Substitute `uuid::Uuid`, `i64`, or any type that
implements `Serialize + DeserializeOwned`.

`ListResponse` is generic over the *resource*, and not over the ID type. The ID
type travels inside the resource: `ListResponse<User<Uuid>>`.

```rust
use scim_v2::models::user::User;
use uuid::Uuid;

let user: User<Uuid> = User {
    user_name: "jdoe@example.com".to_string(),
    ..Default::default()
};
```

For more examples and usage details, read the documentation of each function
and struct.

## Regenerating the filter parser

[LALRPOP](https://github.com/lalrpop/lalrpop) generates the SCIM filter parser
(`src/filter_parser.rs`) from `src/filter_parser.lalrpop` before a build. The
repository holds the generated file. A build script is not necessary.

If you modify `src/filter_parser.lalrpop`, regenerate `src/filter_parser.rs`.
Run these commands:

```sh
cargo install lalrpop
lalrpop src/filter_parser.lalrpop
```

Commit both `src/filter_parser.lalrpop` and the updated
`src/filter_parser.rs` together.

## Contributing

Contributions are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md).
**You must sign every commit.** CI rejects an unsigned commit and an
unverifiable commit.

## License

[MIT](https://choosealicense.com/licenses/mit/)
