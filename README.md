# SCIM v2

\`scim_v2\` is a Rust crate that provides utilities for working with the System for Cross-domain Identity Management (
SCIM) version 2.0 protocol.

## Description

This crate provides functionalities for:

- Models for various SCIM resources such as \`User\`, \`Group\`, \`ResourceType\`, \`ServiceProviderConfig\`, and \`
  EnterpriseUser\`.
- Functions for validating these resources.
- Functions for serializing these resources to JSON.
- Functions for deserializing these resources from JSON.
- SCIM filter expression parsing (RFC 7644 §3.4.2.2) via the `filter` module.
  servers can return an RFC 7644 §3.12 `invalidFilter` response instead of a generic parse error.

## Installation

To use \`scim_v2\` in your project, add the following to your \`Cargo.toml\`:

```toml
[dependencies]
scim_v2 = "0.4"
```

## Usage

Here are some examples of how you can use this crate:

### Validating a User

```rust
use scim_v2::models::user::User;

let user = User {
    user_name: "jdoe@example.com".to_string(),
    // other fields...
    ..Default::default()
};

match user.validate() {
    Ok(_) => println!("User is valid."),
    Err(e) => println!("User is invalid: {}", e),
}
```

### Serializing a User to JSON

```rust
use scim_v2::models::user::User;
use scim_v2::schema_urns;

let user = User {
    schemas: vec![schema_urns::USER.to_string()],
    user_name: "jdoe@example.com".to_string(),
    // Initialize other fields as necessary...
    ..Default::default()
};

match user.serialize() {
    Ok(json) => println ! ("Serialized User: {}", json),
    Err(e) => println !("Serialization error: {}", e),
}
```

### Deserializing a User from JSON

```rust
use scim_v2::models::user::User;

let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
match serde_json::from_str::<User<String>>(user_json) {
    Ok(user) => println!("Successfully converted JSON to User: {:?}", user),
    Err(e) => println!("Error converting from JSON to User: {}", e),
}
```

You can also use a built-in deserialize function if you'd prefer.

```rust
use scim_v2::models::user::User;

let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
match User::<String>::deserialize(user_json) {
    Ok(user) => println!("Deserialized User: {:?}", user),
    Err(e) => println!("Deserialization error: {}", e),
}
```

### Parsing a SCIM filter

```rust
use std::str::FromStr;
use scim_v2::filter::Filter;

let filter = Filter::from_str(r#"userName eq "jdoe@example.com""#)
    .expect("valid filter");
println!("Parsed filter: {}", filter);
```

Filters parsed via `Filter::from_str` or deserialization are rejected if their
AST exceeds `filter::MAX_FILTER_DEPTH` (64) to prevent stack-overflow DoS from
pathologically nested input.

### Tolerant filter parsing for servers

Use `TolerantListQuery` / `TolerantSearchRequest` so a malformed `filter=` does
not fail the whole request — letting you return an RFC 7644 §3.12
`invalidFilter` response while keeping `startIndex`, `count`, etc.

The suggested pattern is to deserialize into the tolerant variant, then call
`.into_strict()` and `map_err` the resulting `InvalidFilterError` into your
SCIM error response. After the conversion you have a `StrictListQuery` /
`StrictSearchRequest` with a fully-parsed `Filter`.

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

### Parsing a SCIM PATCH operation

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

### Using custom ID types

`User`, `Group`, `Member`, `Resource`, and `ListResponse` are generic over their
ID type, defaulting to `String`. Substitute `uuid::Uuid`, `i64`, or any
`Serialize + DeserializeOwned` type.

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

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT](https://choosealicense.com/licenses/mit/)
