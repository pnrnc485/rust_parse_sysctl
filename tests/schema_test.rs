use std::collections::BTreeMap;
use confparser::{parse_schema_str, validate_with_schema, SchemaType, ParseError};

#[test]
fn test_parse_valid_schema() {
    let input = r#"
        endpoint = string
        debug = bool
        log.file = string
        log.max = int
    "#;

    let parsed = parse_schema_str(input).unwrap();

    let expected = BTreeMap::from([
        ("endpoint".to_string(), SchemaType::String),
        ("debug".to_string(), SchemaType::Bool),
        ("log.file".to_string(), SchemaType::String),
        ("log.max".to_string(), SchemaType::Int),
    ]);

    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_invalid_schema_type() {
    let input = r#"
        endpoint = string
        log.level = enum
    "#;

    let result = parse_schema_str(input);
    assert!(matches!(result, Err(ParseError::InvalidLine { .. })));
}

#[test]
fn test_validate_with_schema_success() {
    let config = BTreeMap::from([
        ("endpoint".to_string(), "localhost:3000".to_string()),
        ("debug".to_string(), "true".to_string()),
        ("log.max".to_string(), "100".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("endpoint".to_string(), SchemaType::String),
        ("debug".to_string(), SchemaType::Bool),
        ("log.max".to_string(), SchemaType::Int),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_schema_failure() {
    let config = BTreeMap::from([
        ("endpoint".to_string(), "localhost:3000".to_string()),
        ("debug".to_string(), "yes".to_string()), // ❌ boolではない
        ("log.max".to_string(), "abc".to_string()), // ❌ intではない
    ]);

    let schema = BTreeMap::from([
        ("endpoint".to_string(), SchemaType::String),
        ("debug".to_string(), SchemaType::Bool),
        ("log.max".to_string(), SchemaType::Int),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.contains("debug")));
    assert!(errors.iter().any(|e| e.contains("log.max")));
}

#[test]
fn test_validate_with_schema_float() {
    let config = BTreeMap::from([
        ("rate.limit".to_string(), "3.14".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("rate.limit".to_string(), SchemaType::Float),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_schema_float_invalid() {
    let config = BTreeMap::from([
        ("rate.limit".to_string(), "abc.def".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("rate.limit".to_string(), SchemaType::Float),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_err());
}

