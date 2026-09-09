/// `"schemaExtensions": null` is the third `Vec` field the
/// null-collapse sweep covers; nothing had pinned it.
#[test]
fn schema_extensions_null_collapses_to_empty() {
    let raw = r#"{"name":"User","endpoint":"/Users","schema":"urn:x","schemaExtensions":null}"#;
    let rt: ResourceType = serde_json::from_str(raw).expect("null schemaExtensions");
    assert!(rt.schema_extensions.is_empty());
}

/// RFC 7643 §6 marks `name`, `endpoint` and `schema` REQUIRED, and each is
/// reported by its wire name so a server can echo it.
#[test]
fn validate_requires_name_endpoint_and_schema() {
    let good = ResourceType {
        name: "User".to_string(),
        endpoint: "/Users".to_string(),
        schema: crate::schema_urns::USER.to_string(),
        ..Default::default()
    };
    assert!(good.validate().is_ok());

    for (blank, expected) in [
        ("name", "name"),
        ("endpoint", "endpoint"),
        ("schema", "schema"),
    ] {
        let mut bad = ResourceType {
            name: "User".to_string(),
            endpoint: "/Users".to_string(),
            schema: crate::schema_urns::USER.to_string(),
            ..Default::default()
        };
        match blank {
            "name" => bad.name.clear(),
            "endpoint" => bad.endpoint.clear(),
            _ => bad.schema.clear(),
        }
        let err = bad
            .validate()
            .expect_err("blank required attribute must fail");
        assert_eq!(err.path(), expected);
        assert_eq!(err.scim_type_str(), "invalidValue");
    }
}

/// §6 explicitly says `id` is not required for this resource, so it is not
/// validated; `schemas` is REQUIRED by §3 and is.
#[test]
fn validate_ignores_id_but_requires_schemas() {
    let minimal = ResourceType {
        id: None,
        schemas: vec![crate::schema_urns::RESOURCE_TYPE.to_string()],
        name: "User".to_string(),
        endpoint: "/Users".to_string(),
        schema: crate::schema_urns::USER.to_string(),
        ..Default::default()
    };
    assert!(minimal.validate().is_ok());

    let without = ResourceType {
        schemas: Vec::new(),
        ..minimal
    };
    let err = without
        .validate()
        .expect_err("absent schemas must not validate");
    assert_eq!(err.path(), "schemas");
}

/// The `schemas` attribute is modelled from 1.0 on. RFC 7643 §6's example
/// carries it; before 1.0 there was no field, so it was dropped.
#[test]
fn schemas_round_trips_and_absence_is_tolerated() {
    let raw = format!(
        r#"{{"schemas":["{}"],"id":"User","name":"User","endpoint":"/Users","schema":"{}"}}"#,
        crate::schema_urns::RESOURCE_TYPE,
        crate::schema_urns::USER
    );
    let rt: ResourceType = serde_json::from_str(&raw).unwrap();
    assert_eq!(rt.schemas, vec![crate::schema_urns::RESOURCE_TYPE]);
    let back = serde_json::to_value(&rt).unwrap();
    assert_eq!(
        back["schemas"],
        serde_json::json!([crate::schema_urns::RESOURCE_TYPE])
    );

    let no_schemas = r#"{"name":"User","endpoint":"/Users","schema":"urn:x"}"#;
    let rt: ResourceType = serde_json::from_str(no_schemas).unwrap();
    assert!(rt.schemas.is_empty());
    assert_eq!(rt.validate().unwrap_err().path(), "schemas");
}

/// `Default` is constructible and carries the resource's own schema URN,
/// but is not conformant on its own — the three REQUIRED attributes are
/// blank.
#[test]
fn default_carries_its_urn_but_does_not_validate() {
    let rt = ResourceType::default();
    assert_eq!(rt.schemas, vec![crate::schema_urns::RESOURCE_TYPE]);
    assert!(rt.name.is_empty());
    assert!(rt.schema_extensions.is_empty());
    assert!(rt.validate().is_err());
    serde_json::to_string(&rt).expect("Default must serialize");
}
use pretty_assertions::assert_eq;

use super::*;

#[test]
fn resource_type_deserialization_user() {
    let json_data = r#"{
                "schemas":
                [
                    "urn:ietf:params:scim:schemas:core:2.0:ResourceType"
                ],
                "id": "User",
                "name": "User",
                "endpoint": "/Users",
                "description": "User Account",
                "schema": "urn:ietf:params:scim:schemas:core:2.0:User",
                "schemaExtensions":
                [
                    {
                        "schema": "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
                        "required": true
                    }
                ],
                "meta":
                {
                    "location": "https://example.com/v2/ResourceTypes/User",
                    "resourceType": "ResourceType"
                }
            }"#;

    let resource_type: Result<ResourceType, serde_json::Error> = serde_json::from_str(json_data);

    if let Err(e) = &resource_type {
        eprintln!("Deserialization failed: {:?}", e);
    }
    assert!(resource_type.is_ok());
    let resource_type = resource_type.unwrap();
    assert_eq!(resource_type.id, Some("User".to_string()));
    assert_eq!(resource_type.name, "User");
    assert_eq!(resource_type.endpoint, "/Users");
    assert_eq!(resource_type.description, Some("User Account".to_string()));
    assert_eq!(
        resource_type.schema,
        "urn:ietf:params:scim:schemas:core:2.0:User"
    );
    let schema_extensions = &resource_type.schema_extensions;
    assert_eq!(schema_extensions.len(), 1);
    let schema_extension = &schema_extensions[0];
    assert_eq!(
        schema_extension.schema,
        "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"
    );
    assert_eq!(schema_extension.required, true);
}

#[test]
fn resource_type_deserialization_group() {
    let json_data = r#"{
                "schemas":
                [
                    "urn:ietf:params:scim:schemas:core:2.0:ResourceType"


                ],
                "id": "Group",
                "name": "Group",
                "endpoint": "/Groups",
                "description": "Group",
                "schema": "urn:ietf:params:scim:schemas:core:2.0:Group",
                "meta":
                {
                    "location": "https://example.com/v2/ResourceTypes/Group",
                    "resourceType": "ResourceType"
                }
            }"#;

    let resource_type: Result<ResourceType, serde_json::Error> = serde_json::from_str(json_data);

    if let Err(e) = &resource_type {
        eprintln!("Deserialization failed: {:?}", e);
    }
    assert!(resource_type.is_ok());
    let resource_type = resource_type.unwrap();
    assert_eq!(resource_type.id, Some("Group".to_string()));
    assert_eq!(resource_type.name, "Group");
    assert_eq!(resource_type.endpoint, "/Groups");
    assert_eq!(resource_type.description, Some("Group".to_string()));
    assert_eq!(
        resource_type.schema,
        "urn:ietf:params:scim:schemas:core:2.0:Group"
    );
}

#[test]
fn test_get_resource_types() {
    let resource_type_names = vec!["user", "group", "enterprise_user"];
    let resource_types = get_resource_types(resource_type_names).unwrap();

    assert_eq!(resource_types.len(), 2);

    let user_resource_type = &resource_types[0];
    assert_eq!(user_resource_type.name, "User");
    assert_eq!(user_resource_type.endpoint, "/Users");
    assert_eq!(
        user_resource_type.schema,
        "urn:ietf:params:scim:schemas:core:2.0:User"
    );
    assert!(!user_resource_type.schema_extensions.is_empty());

    let group_resource_type = &resource_types[1];
    assert_eq!(group_resource_type.name, "Group");
    assert_eq!(group_resource_type.endpoint, "/Groups");
    assert_eq!(
        group_resource_type.schema,
        "urn:ietf:params:scim:schemas:core:2.0:Group"
    );
    assert!(group_resource_type.schema_extensions.is_empty());
}

/// RFC 7643 §6: a schema extension's `schema` "MUST be equal to the id
/// attribute of a Schema resource. REQUIRED."
#[test]
fn validate_requires_each_schema_extension_to_name_its_schema() {
    let mut rt = ResourceType {
        schemas: vec![crate::schema_urns::RESOURCE_TYPE.to_string()],
        name: "User".to_string(),
        endpoint: "/Users".to_string(),
        schema: crate::schema_urns::USER.to_string(),
        schema_extensions: vec![
            SchemaExtension {
                schema: crate::schema_urns::ENTERPRISE_USER.to_string(),
                required: true,
            },
            SchemaExtension {
                schema: String::new(),
                required: false,
            },
        ],
        ..Default::default()
    };
    let err = rt.validate().unwrap_err();
    assert_eq!(err.path(), "schemaExtensions[1].schema");
    rt.schema_extensions.pop();
    assert_eq!(rt.validate(), Ok(()));
}
