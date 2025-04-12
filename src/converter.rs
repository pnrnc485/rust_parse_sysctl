use std::collections::BTreeMap;
use serde_json::{Map, Value};

/// フラットな BTreeMap<String, String> をネストされた JSON 構造に変換する。
///
/// 例:
/// "log.file" => "/var/log/console.log"
/// "log.name" => "default.log"
///
/// →
/// {
///   "log": {
///     "file": "/var/log/console.log",
///     "name": "default.log"
///   }
/// }
pub fn flatten_to_nested_json(map: &BTreeMap<String, String>) -> Map<String, Value> {
  let mut root = Map::new();

  for (full_key, value) in map {
      let parts: Vec<&str> = full_key.split('.').collect();
      let mut current = &mut root;

      for (i, part) in parts.iter().enumerate() {
          if i == parts.len() - 1 {
              // 最後のパート → 値を挿入
              current.insert(part.to_string(), Value::String(value.clone()));
          } else {
              // 中間ノード → Value::Object を期待
              current = current
                  .entry(part.to_string())
                  .or_insert_with(|| Value::Object(Map::new()))
                  .as_object_mut()
                  .expect("expected object while nesting JSON");
          }
      }
  }

  Value::Object(root)
}