//! The crate holds two representations of RFC 7643: the Rust structs, and the
//! §8.7 schema definitions it serves from `get_schemas`. Nothing else checks
//! that they agree.
//!
//! Each attribute characteristic the schema declares has one correct Rust
//! shape, so the mapping is mechanical:
//!
//! | the schema says | the field must be |
//! |---|---|
//! | `"required": true` | a bare type |
//! | `"multiValued": true` | `Asserted<Vec<T>>` |
//! | `"mutability": "readOnly"` | `Option<T>`, because RFC 7644 §3.5.1 says of readOnly, "Any values provided SHALL be ignored" |
//! | `"mutability": "readWrite"` or `"writeOnly"` | `Asserted<T>`, because §3.5.1 gives the client a clear to express |
//!
//! A readOnly **multi-valued** attribute takes a plain `Vec<T>`, which is the
//! natural Rust type with no assertion state, exactly as `Option<T>` is for a
//! readOnly scalar.
//!
//! A field added without `Asserted`, or a schema that drifts from the struct,
//! fails here.

#![cfg(all(feature = "models", feature = "schemas"))]

use serde_json::Value;

/// One `pub` field of a struct, as written in the source.
struct Field {
    wire_name: String,
    ty: String,
}

/// Reads the `pub` fields of one struct out of a source file.
///
/// Rust has no field reflection, and the distinction this test exists to
/// check is `Option` versus `Asserted`, which no single serialized sample
/// shows. So the source is the only place the answer is written down.
fn fields_of(source: &str, struct_name: &str) -> Vec<Field> {
    let start = source
        .find(&format!("pub struct {struct_name}"))
        .unwrap_or_else(|| panic!("no struct {struct_name}"));
    let body = &source[start..];
    let end = body.find("\n}").expect("struct has a closing brace");
    let body = &body[..end];

    let mut fields = Vec::new();
    let mut rename: Option<String> = None;
    for line in body.lines() {
        let line = line.trim();
        if let Some(i) = line.find("rename = \"") {
            let rest = &line[i + 10..];
            rename = rest.find('"').map(|j| rest[..j].to_string());
        }
        let Some(rest) = line.strip_prefix("pub ") else {
            continue;
        };
        let Some((name, ty)) = rest.split_once(": ") else {
            continue;
        };
        let name = name.trim_start_matches("r#");
        fields.push(Field {
            wire_name: rename.take().unwrap_or_else(|| camel_case(name)),
            ty: ty.trim_end_matches(',').to_string(),
        });
    }
    fields
}

fn camel_case(snake: &str) -> String {
    let mut out = String::with_capacity(snake.len());
    let mut upper = false;
    for c in snake.chars() {
        match c {
            '_' => upper = true,
            c if upper => {
                out.extend(c.to_uppercase());
                upper = false;
            }
            c => out.push(c),
        }
    }
    out
}

/// Checks one embedded schema against the struct that models it.
fn assert_agrees(schema_json: &str, source: &str, struct_name: &str) {
    let schema: Value = serde_json::from_str(schema_json).expect("embedded schema parses");
    let fields = fields_of(source, struct_name);

    for attr in schema["attributes"].as_array().expect("attributes") {
        let name = attr["name"].as_str().expect("name");
        let field = fields
            .iter()
            .find(|f| f.wire_name == name)
            .unwrap_or_else(|| {
                panic!("{struct_name} has no field for the §8.7 attribute {name:?}")
            });
        let ty = &field.ty;

        // RFC 7643 contradicts itself on one attribute. §4.2 says of
        // `Group.displayName`, "A human-readable name for the Group.
        // REQUIRED.", and §8.7.1 copies that same sentence into the
        // attribute's own `description` while declaring `"required": false`.
        // The prose is normative, so the struct follows it and this test
        // records the defect rather than the schema's `false`.
        let required = match (struct_name, name) {
            // RFC 7643 contradicts itself twice, in opposite directions, and
            // this test records both rather than bending the struct to a flag
            // the RFC's own text disagrees with.
            //
            // §4.2 says of `Group.displayName`, "A human-readable name for the
            // Group.  REQUIRED.", and §8.7.1 copies that same sentence into
            // the attribute's own `description` while declaring
            // `"required": false`. The prose is normative, so the field is a
            // bare `String`.
            ("Group<T = String>", "displayName") => true,
            // §7 declares `ResourceType.schemaExtensions` `"required": true`,
            // and Figure 8, the RFC's own example, omits it entirely from the
            // Group resource type. A provider that copies the figure would be
            // rejected, so the field stays omittable.
            ("ResourceType", "schemaExtensions") => false,
            _ => attr["required"].as_bool().unwrap_or(false),
        };
        let multi = attr["multiValued"].as_bool().unwrap_or(false);
        let mutability = attr["mutability"].as_str().unwrap_or("readWrite");

        if required {
            assert!(
                !ty.starts_with("Option<") && !ty.starts_with("Asserted<"),
                "{struct_name}.{name} is REQUIRED in §8.7, so it is a bare type, not {ty}"
            );
            continue;
        }

        if multi && mutability != "readOnly" && name != "schemaExtensions" {
            assert!(
                ty.starts_with("Asserted<Vec<"),
                "{struct_name}.{name} is multiValued in §8.7, so it is Asserted<Vec<_>>, not {ty}"
            );
        }

        match mutability {
            // The third contradiction, and the clearest. §7 declares
            // `ResourceType.schemaExtensions` `"multiValued": false` while its
            // own `description` in the same object reads "A list of URIs of
            // the resource type's schema extensions", and Figure 7 shows a
            // JSON array. It is a list, so the field holds one, and it is an
            // `Asserted` rather than a bare `Vec` because Figure 8 omits the
            // attribute on the Group resource type.
            "readOnly" if (struct_name, name) == ("ResourceType", "schemaExtensions") => {
                assert!(
                    ty.starts_with("Asserted<Vec<"),
                    "{struct_name}.{name} is a list the RFC lets a provider omit, not {ty}"
                )
            }
            "readOnly" if multi => assert!(
                ty.starts_with("Vec<"),
                "{struct_name}.{name} is a readOnly multiValued attribute in §8.7, so it carries \
                 no assertion state and is a plain Vec<_>, not {ty}"
            ),
            "readOnly" => assert!(
                ty.starts_with("Option<"),
                "{struct_name}.{name} is readOnly in §8.7. RFC 7644 §3.5.1: \"Any values provided \
                 SHALL be ignored.\" A client cannot assert it, so it is Option<_>, not {ty}"
            ),
            "readWrite" | "writeOnly" => assert!(
                ty.starts_with("Asserted<"),
                "{struct_name}.{name} is {mutability} in §8.7, so RFC 7644 §3.5.1 gives the client \
                 a clear to express and the field is Asserted<_>, not {ty}"
            ),
            other => panic!("{struct_name}.{name} has an unhandled mutability {other:?}"),
        }
    }
}

#[test]
fn the_user_struct_matches_the_user_schema() {
    assert_agrees(
        include_str!("../src/schemas/user.json"),
        include_str!("../src/models/user.rs"),
        "User<T = String>",
    );
}

#[test]
fn the_group_struct_matches_the_group_schema() {
    assert_agrees(
        include_str!("../src/schemas/group.json"),
        include_str!("../src/models/group.rs"),
        "Group<T = String>",
    );
}

#[test]
fn the_enterprise_user_struct_matches_its_schema() {
    assert_agrees(
        include_str!("../src/schemas/enterprise_user.json"),
        include_str!("../src/models/enterprise_user.rs"),
        "EnterpriseUser",
    );
}

#[test]
fn the_schema_struct_matches_the_schema_schema() {
    assert_agrees(
        include_str!("../src/schemas/scim_schema.json"),
        include_str!("../src/models/scim_schema.rs"),
        "Schema",
    );
}

#[test]
fn the_resource_type_struct_matches_its_schema() {
    assert_agrees(
        include_str!("../src/schemas/resource_type.json"),
        include_str!("../src/models/resource_types.rs"),
        "ResourceType",
    );
}

#[test]
fn the_service_provider_config_struct_matches_its_schema() {
    assert_agrees(
        include_str!("../src/schemas/service_provider_config.json"),
        include_str!("../src/models/service_provider_config.rs"),
        "ServiceProviderConfig",
    );
}
