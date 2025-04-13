use std::collections::BTreeMap;
use confparser::{parse_schema_str, SchemaType, ParseError};

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
