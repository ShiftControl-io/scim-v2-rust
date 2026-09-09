use super::*;

/// RFC 7643 §7 — `id` MUST be specified; `schemas`, when present,
/// must name this resource type; absent `schemas` is tolerated for
/// `Schema` alone, after the RFC's own §8.7 representations.
#[test]
fn validate_requires_id_and_checks_the_urn() {
    use crate::Validate;
    let base = Schema {
        schemas: Vec::new(),
        id: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
        name: "User".to_string(),
        description: String::new(),
        attributes: Vec::new(),
        meta: Meta {
            resource_type: None,
            created: None,
            last_modified: None,
            version: None,
            location: None,
        },
    };
    assert!(base.validate().is_ok(), "absent schemas is legal");
    assert_eq!(
        Schema {
            id: String::new(),
            ..base.clone()
        }
        .validate()
        .expect_err("id")
        .path(),
        "id"
    );
    assert_eq!(
        Schema {
            schemas: vec![crate::schema_urns::GROUP.to_string()],
            ..base.clone()
        }
        .validate()
        .expect_err("wrong URN")
        .path(),
        "schemas"
    );
    assert!(
        Schema {
            schemas: vec![crate::schema_urns::SCHEMA.to_string()],
            ..base
        }
        .validate()
        .is_ok()
    );
}

/// The `schemas` attribute is modelled from 1.0 on. RFC 7643 §7 lets a
/// schema resource be served without it, so absence must be legal, but a
/// present value has to round-trip rather than being dropped.
#[test]
fn schemas_round_trips_and_absence_is_tolerated() {
    let with = format!(
        r#"{{"schemas":["{}"],"id":"urn:x","name":"X","description":"d",
                 "attributes":[],"meta":{{}}}}"#,
        crate::schema_urns::SCHEMA
    );
    let schema: Schema = serde_json::from_str(&with).expect("must deserialize");
    assert_eq!(schema.schemas, vec![crate::schema_urns::SCHEMA]);
    let back = serde_json::to_value(&schema).unwrap();
    assert_eq!(
        back["schemas"],
        serde_json::json!([crate::schema_urns::SCHEMA])
    );

    let without = r#"{"id":"urn:x","name":"X","description":"d","attributes":[],"meta":{}}"#;
    let schema: Schema = serde_json::from_str(without)
        .expect("RFC 7643 §7 allows a schema resource without `schemas`");
    assert!(schema.schemas.is_empty());
}

/// `canonicalValues`, `subAttributes` and `referenceTypes` are
/// multi-valued, so RFC 7643 §2.5 applies: absent, `null` and `[]` are one
/// state, and unassigned is omitted on the way out.
#[test]
fn multi_valued_sub_attributes_treat_absent_null_and_empty_alike() {
    for raw in [
        r#"{"name":"a","type":"string","multiValued":false,"description":"d","required":false,"caseExact":false,"mutability":"readWrite","returned":"default","uniqueness":"none"}"#,
        r#"{"name":"a","type":"string","multiValued":false,"description":"d","required":false,"caseExact":false,"mutability":"readWrite","returned":"default","uniqueness":"none","canonicalValues":null,"referenceTypes":null}"#,
        r#"{"name":"a","type":"string","multiValued":false,"description":"d","required":false,"caseExact":false,"mutability":"readWrite","returned":"default","uniqueness":"none","canonicalValues":[],"referenceTypes":[]}"#,
    ] {
        let attr: Attributes = serde_json::from_str(raw).unwrap_or_else(|e| panic!("{raw}: {e}"));
        assert!(attr.canonical_values.is_empty());
        assert!(attr.reference_types.is_empty());
        let out = serde_json::to_value(&attr).unwrap();
        assert_eq!(out["canonicalValues"], serde_json::json!([]));
        assert_eq!(out["referenceTypes"], serde_json::json!([]));
    }
}
