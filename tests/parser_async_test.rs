use confparser::parse_url_async;

const TEST_URL: &str = "https://gist.githubusercontent.com/pnrnc485/01d4b192ef7e159b7ed8cf52e87b382a/raw/43cf2bcddf5f2a961e3c4b82f04318d2ea3fb7f6/sample.conf";

#[tokio::test]
async fn test_parse_url_async_success() {
    let result = parse_url_async(TEST_URL).await;

    assert!(result.is_ok(), "URLのパースに失敗しました: {:?}", result);

    let map = result.unwrap();
    assert_eq!(map.get("net.ipv4.ip_forward"), Some(&"0".to_string()));
    assert_eq!(map.get("log.file"), Some(&"/var/log/console.log".to_string()));
}
