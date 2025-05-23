use std::collections::BTreeMap;
use confparser::{parse_schema_str, validate_with_schema, parse_str, schema::{SchemaType, SchemaEntry}, ParseError};

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
        ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false, default: None }),
        ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false, default: None }),
        ("log.file".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false, default: None }),
        ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false, default: None }),
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
  let mut config = BTreeMap::from([
      ("endpoint".to_string(), "localhost:3000".to_string()),
      ("debug".to_string(), "true".to_string()),
      ("log.max".to_string(), "100".to_string()),
  ]);

  let schema = BTreeMap::from([
      ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false, default: None }),
      ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false, default: None }),
      ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false, default: None }),
  ]);

  let result = validate_with_schema(&mut config, &schema);
  assert!(result.is_ok());
}


#[test]
fn test_validate_with_schema_failure() {
    let mut config = BTreeMap::from([
        ("endpoint".to_string(), "localhost:3000".to_string()),
        ("debug".to_string(), "yes".to_string()), // ❌ boolではない
        ("log.max".to_string(), "abc".to_string()), // ❌ intではない
    ]);

    let schema = BTreeMap::from([
        ("endpoint".to_string(), SchemaEntry { typ: SchemaType::String(None), required: false, default: None }),
        ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false, default: None }),
        ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false, default: None }),
        ("debug".to_string(), SchemaEntry { typ: SchemaType::Bool, required: false, default: None }),
        ("log.max".to_string(), SchemaEntry { typ: SchemaType::Int, required: false, default: None }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.contains("debug")));
    assert!(errors.iter().any(|e| e.contains("log.max")));
}

#[test]
fn test_validate_with_schema_float() {
    let mut config = BTreeMap::from([
        ("rate.limit".to_string(), "3.14".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("rate.limit".to_string(), SchemaEntry { typ: SchemaType::Float, required: false, default: None }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_schema_float_invalid() {
    let mut config = BTreeMap::from([
        ("rate.limit".to_string(), "abc.def".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("rate.limit".to_string(), SchemaEntry { typ: SchemaType::Float, required: false, default: None }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_err());
}

#[test]
fn test_string_with_max_length_passes() {
    let mut config = BTreeMap::from([
        ("log.message".to_string(), "short message".to_string())
    ]);

    let schema = BTreeMap::from([
        ("log.message".to_string(), SchemaEntry { typ: SchemaType::String(Some(20)), required: false, default: None }),
    ]);

    assert!(validate_with_schema(&mut config, &schema).is_ok());
}

#[test]
fn test_string_with_max_length_fails() {
    let mut config = BTreeMap::from([
        ("log.message".to_string(), "x".repeat(101))
    ]);

    let schema = BTreeMap::from([
        ("log.message".to_string(), SchemaEntry { typ: SchemaType::String(Some(100)), required: false, default: None }),
    ]);

    assert!(validate_with_schema(&mut config, &schema).is_err());
}

#[test]
fn test_enum_type_valid() {
    let mut config = BTreeMap::from([
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
            default: None
        })
    ]);

    assert!(validate_with_schema(&mut config, &schema).is_ok());
}

#[test]
fn test_enum_type_invalid() {
    let mut config = BTreeMap::from([
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
            default: None
        })
    ]);

    assert!(validate_with_schema(&mut config, &schema).is_err());
}

#[test]
fn test_required_field_present() {
    let mut config = BTreeMap::from([
        ("log.file".to_string(), "/var/log/app.log".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("log.file".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: true,
            default: None,
        }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_ok());
}

#[test]
fn test_required_field_missing() {
    let mut config = BTreeMap::new(); // 空の設定

    let schema = BTreeMap::from([
        ("log.file".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: true,
            default: None,
        }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("log.file"));
    assert!(errors[0].contains("required"));
}

#[test]
fn test_default_value_is_inserted() {
    let mut config = BTreeMap::from([
        ("log.file".to_string(), "/var/log/app.log".to_string()),
    ]);

    let schema = BTreeMap::from([
        ("log.file".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: true,
            default: None,
        }),
        ("log.level".to_string(), SchemaEntry {
            typ: SchemaType::String(None),
            required: false,
            default: Some("info".to_string()),
        }),
        ("timeout".to_string(), SchemaEntry {
            typ: SchemaType::Int,
            required: false,
            default: Some("30".to_string()),
        }),
    ]);

    let result = validate_with_schema(&mut config, &schema);
    assert!(result.is_ok());

    // ✅ default が補完されていることを確認
    assert_eq!(config.get("log.level"), Some(&"info".to_string()));
    assert_eq!(config.get("timeout"), Some(&"30".to_string()));
}