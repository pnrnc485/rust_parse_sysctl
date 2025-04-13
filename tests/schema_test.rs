use std::collections::BTreeMap;
use confparser::{parse_schema_str, validate_with_schema, schema::{SchemaType, SchemaEntry}, ParseError};

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
        ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false }),
        ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false }),
        ("log.file".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false }),
        ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false }),
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
      ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false }),
      ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false }),
      ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false }),
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
        ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false }),
        ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false }),
        ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false }),
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
        ("rate.limit".to_string(), SchemaEntry { typ: SchemaType::Float, required: false }),
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
        ("rate.limit".to_string(), SchemaEntry { typ: SchemaType::Float, required: false }),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_err());
}

#[test]
fn test_string_with_max_length_passes() {
    let config = BTreeMap::from([
        ("log.message".to_string(), "short message".to_string())
    ]);

    let schema = BTreeMap::from([
        ("log.message".to_string(), SchemaEntry { typ: SchemaType::String(Some(20)), required: false }),
    ]);

    assert!(validate_with_schema(&config, &schema).is_ok());
}

#[test]
fn test_string_with_max_length_fails() {
    let config = BTreeMap::from([
        ("log.message".to_string(), "x".repeat(101))
    ]);

    let schema = BTreeMap::from([
        ("log.message".to_string(), SchemaEntry { typ: SchemaType::String(Some(100)), required: false }),
    ]);

    assert!(validate_with_schema(&config, &schema).is_err());
}

#[test]
fn test_enum_type_valid() {
    let config = BTreeMap::from([
        ("log.type".to_string(), "auto".to_string())
    ]);

    let schema = BTreeMap::from([
        ("log.type".to_string(), SchemaEntry {
            typ: SchemaType::Enum(vec![
                "auto".to_string(),
                "manual".to_string(),
                "self".to_string(),
            ]),
            required: false,
        })
    ]);

    assert!(validate_with_schema(&config, &schema).is_ok());
}

#[test]
fn test_enum_type_invalid() {
    let config = BTreeMap::from([
        ("log.type".to_string(), "invalid".to_string())
    ]);

    let schema = BTreeMap::from([
        ("log.type".to_string(), SchemaEntry {
            typ: SchemaType::Enum(vec![
                "auto".to_string(),
                "manual".to_string(),
                "self".to_string(),
            ]),
            required: false,
        })
    ]);

    assert!(validate_with_schema(&config, &schema).is_err());
}

#[test]
fn test_required_field_present() {
    let config = BTreeMap::from([
        ("log.file".to_string(), "/var/log/app.log".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("log.file".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: true,
        }),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_ok());
}

#[test]
fn test_required_field_missing() {
    let config = BTreeMap::new(); // 空の設定

    let schema = BTreeMap::from([
        ("log.file".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: true,
        }),
    ]);

    let result = validate_with_schema(&config, &schema);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("log.file"));
    assert!(errors[0].contains("required"));
}