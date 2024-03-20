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

## Installation

To use \`scim_v2\` in your project, add the following to your \`Cargo.toml\`:

```toml
[dependencies]
scim_v2 = "0.2.0"
```

Then run \`cargo build\` to download and compile the \`scim_v2\` crate and all its dependencies.

## Usage

Here are some examples of how you can use this crate:

### Validating a User

```
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

```
use scim_v2::models::user::User;

let user = User {
schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
user_name: "jdoe@example.com".to_string(),
// Initialize other fields as necessary...
..Default::default ()
};

match user.serialize() {
Ok(json) => println ! ("Serialized User: {}", json),
Err(e) => println !("Serialization error: {}", e),
}
```

### Deserializing a User from JSON

```
use scim_v2::models::user::User;

let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
match User::try_from(user_json) {
Ok(user) => println!("Successfully converted JSON to User: {:?}", user),
Err(e) => println!("Error converting from JSON to User: {}", e),
}
```

You can also use a built-in deserialize function if you’d prefer.

```
use scim_v2::models::user::User;

let user_json = r#"{"schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"], "userName": "jdoe@example.com"}"#;
match User::deserialize(user_json) {
Ok(user) => println ! ("Deserialized User: {:?}", user),
Err(e) => println !("Deserialization error: {}", e),
}
```

For more examples and usage details, refer to the documentation of each function and struct.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT](https://choosealicense.com/licenses/mit/)