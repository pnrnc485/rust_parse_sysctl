use confparser::parse_str;

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