use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use crate::ParseError;

/// 空行・コメント・無視行かどうか判定
fn is_ignorable_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') || trimmed.starts_with('-')
}

/// 行末のコメントを削除（#以降を除去）
fn remove_inline_comment(value: &str) -> &str {
    match value.find('#') {
        Some(pos) => &value[..pos],
        None => value,
    }
}

/// 文字列をパースして BTreeMap を返す
pub fn parse_str(input: &str) -> Result<BTreeMap<String, String>, ParseError> {
    let mut map = BTreeMap::new();

    for (i, line) in input.lines().enumerate() {
        if is_ignorable_line(line) {
            continue;
        }

        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(ParseError::InvalidLine {
                line_number: i + 1,
                content: line.to_string(),
            });
        };

        let key = raw_key.trim();
        let value_content = remove_inline_comment(raw_value).trim();

        if value_content.len() > 4096 {
            return Err(ParseError::ValueTooLong {
                line_number: i + 1,
                key: key.to_string(),
                length: value_content.len(),
            });
        }

        map.insert(key.to_string(), value_content.to_string());
    }

    Ok(map)
}

/// ファイルから読み込んでパースする
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, String>, ParseError> {
    let content = fs::read_to_string(&path)?;
    parse_str(&content)
}
