use std::collections::BTreeMap;
use confparser::converter::flatten_to_nested_json;
use serde_json::json;

#[test]
fn test_flatten_to_nested_json_basic() {
  let mut flat_map = BTreeMap::new();
  flat_map.insert("endpoint".to_string(), "localhost:3000".to_string());
  flat_map.insert("log.file".to_string(), "/var/log/console.log".to_string());
  flat_map.insert("log.name".to_string(), "default.log".to_string());

  let nested = flatten_to_nested_json(&flat_map);

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
fn test_nested_with_multiple_levels() {
    let mut flat_map = BTreeMap::new();
    flat_map.insert("db.mysql.username".to_string(), "root".to_string());
    flat_map.insert("db.mysql.password".to_string(), "secret".to_string());
    flat_map.insert("db.postgres.port".to_string(), "5432".to_string());

    let nested = flatten_to_nested_json(&flat_map);

    let expected = json!({
        "db": {
            "mysql": {
                "username": "root",
                "password": "secret"
            },
            "postgres": {
                "port": "5432"
            }
        }
    });

    assert_eq!(nested, expected);
}
