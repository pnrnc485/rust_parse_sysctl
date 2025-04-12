
use confparser::{
    ParseError,
    parse_str,
    flatten_to_nested_json,
};
use serde_json::json;

#[test]
fn test_flat_config_parses_correctly() {
    let input = r#"
        endpoint = localhost:3000
        debug = true
        log.file = /var/log/console.log
    "#;

    let flat = parse_str(input).unwrap();
    assert_eq!(flat.get("endpoint"), Some(&"localhost:3000".to_string()));
    assert_eq!(flat.get("debug"), Some(&"true".to_string()));
    assert_eq!(flat.get("log.file"), Some(&"/var/log/console.log".to_string()));
}

#[test]
fn test_flatten_to_nested_json_input1() {
    let input = r#"
        endpoint = localhost:3000
        debug = true
        log.file = /var/log/console.log
    "#;

    let flat = parse_str(input).unwrap();
    let nested = flatten_to_nested_json(&flat);

    let expected = json!({
        "endpoint": "localhost:3000",
        "debug": "true",
        "log": {
            "file": "/var/log/console.log"
        }
    });

    assert_eq!(nested, expected);
}

#[test]
fn test_flatten_to_nested_json_input2() {
    let input = r#"
        endpoint = localhost:3000
        # debug = true
        log.file = /var/log/console.log
        log.name = default.log
    "#;

    let flat = parse_str(input).unwrap();
    let nested = flatten_to_nested_json(&flat);

    let expected = json!({
        "endpoint": "localhost:3000",
        "log": {
            "file": "/var/log/console.log",
            "name": "default.log"
        }
    });

    assert_eq!(nested, expected);
}

#[test]
fn test_semicolon_comment_line_only() {
    let input = "
        ; this is a comment
        kernel.threads-max = 12345
    ";
    let result = parse_str(input).unwrap();
    assert_eq!(result.get("kernel.threads-max").unwrap(), "12345");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_dash_prefix_line_is_ignored() {
    let input = "
        -net.ipv4.conf.all.rp_filter = 1
        kernel.pid_max = 65535
    ";

    let result = parse_str(input).unwrap();

    assert!(result.get("net.ipv4.conf.all.rp_filter").is_none());
    assert_eq!(result.get("kernel.pid_max").unwrap(), "65535");
}

#[test]
fn test_value_too_long_error() {
    let long_value = "a".repeat(4097);
    let input = format!("kernel.long_value = {}", long_value);

    let result = parse_str(&input);
    assert!(matches!(result, Err(ParseError::ValueTooLong { .. })));
}