use super::*;

#[test]
fn get_schemas_returns_correct_schemas_for_valid_input() {
    let schemas = get_schemas(vec!["user"]).unwrap();
    assert_eq!(schemas.len(), 1);
    assert_eq!(schemas[0].id, "urn:ietf:params:scim:schemas:core:2.0:User");
    assert_eq!(schemas[0].name, "User");
    assert_eq!(schemas[0].description, "User Account");
    assert_eq!(schemas[0].attributes.len(), 21);
    assert_eq!(
        schemas[0].meta.resource_type.as_ref(),
        Some(&"Schema".to_string())
    );
    assert_eq!(
        schemas[0].meta.location.as_ref(),
        Some(&"/v2/Schemas/urn:ietf:params:scim:schemas:core:2.0:User".to_string())
    );
}

#[test]
fn get_schemas_returns_error_for_invalid_input() {
    let result = get_schemas(vec!["invalid"]);
    assert!(result.is_err());
}

#[test]
fn get_schemas_returns_error_for_missing_file() {
    let result = get_schemas(vec!["missing"]);
    assert!(result.is_err());
}
