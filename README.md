# SCIM v2

\`scim_v2\` is a Rust crate that provides utilities for working with the System for Cross-domain Identity Management (SCIM) version 2.0 protocol.

## Description

This crate provides functionalities for:

- Models for various SCIM resources such as \`User\`, \`Group\`, \`ResourceType\`, \`ServiceProviderConfig\`, and \`EnterpriseUser\`.
- Functions for validating these resources.
- Functions for serializing these resources to JSON.
- Functions for deserializing these resources from JSON.

## Installation

To use \`scim_v2\` in your project, add the following to your \`Cargo.toml\`:

```toml
[dependencies]
scim_v2 = "0.1.0"
```

Then run \`cargo build\` to download and compile the \`scim_v2\` crate and all its dependencies.

## Usage

Here are some examples of how you can use this crate:

### Validating a User

```rust
use scim_v2::models::user::User;
use scim_v2::validate_user;

let user = User {
    // Initialize user fields here...
    // ...
    ..Default::default()
};

match validate_user(&user) {
    Ok(_) => println!("User is valid."),
    Err(e) => println!("User is invalid: {}", e),
}
```

### Serializing a User to JSON

```rust
use scim_v2::models::user::User;
use scim_v2::user_to_json;

let user = User {
    // Initialize user fields here...
    // ...
    ..Default::default()
};

match user_to_json(&user) {
    Ok(json) => println!("User in JSON format: {}", json),
    Err(e) => println!("Error serializing user to JSON: {}", e),
}
```

### Deserializing a User from JSON

```rust
use scim_v2::models::user::User;
use scim_v2::json_to_user;

let json = r#"{
    "userName": "jdoe",
    "name": {
        "formatted": "Mr. John Doe"
    }
}"#;

match json_to_user(json) {
    Ok(user) => println!("User: {:?}", user),
    Err(e) => println!("Error deserializing JSON to User: {}", e),
}
```

For more examples and usage details, refer to the documentation of each function and struct.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT](https://choosealicense.com/licenses/mit/)