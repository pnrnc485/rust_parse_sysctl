pub mod converter;
pub mod errors;

pub use converter::flatten_to_nested_json;
use errors::ParseError;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;



pub fn parse_str(input: &str) -> Result<BTreeMap<String, String>, ParseError> {
    let mut map = BTreeMap::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') || line.starts_with('-') {
            continue;
        }

    
        // = で区切れない行はエラー
        let Some((key, value)) = line.split_once('=') else {
            return Err(ParseError::InvalidLine {
                line_number: i + 1,
                content: line.to_string(),
            });
        };

        // 行末コメント対応
        let key = key.trim();
        let value = {
            let trimmed = value.trim();
            match trimmed.find('#') {
                Some(pos) => &trimmed[..pos],
                None => trimmed,
            }
        }.trim();

        // ✅ 長さチェックを追加（4096文字以上ならエラー）
        if value.len() > 4096 {
          return Err(ParseError::ValueTooLong {
            line_number: i + 1,
            key: key.to_string(),
            length: value.len(),
          });
        }
      
        map.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(map)
}

/// ファイルパスを受け取り、ファイル内容をパースする
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, String>, ParseError> {
  let content = fs::read_to_string(&path)?;
  parse_str(&content)
}