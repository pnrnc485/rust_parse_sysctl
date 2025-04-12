use confparser::{parse_str, ParseError};

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