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