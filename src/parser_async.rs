use std::collections::BTreeMap;
use crate::ParseError;
use crate::parser::parse_str;

/// 指定されたURLから設定ファイルを取得し、非同期でパースする。
pub async fn parse_url_async(url: &str) -> Result<BTreeMap<String, String>, ParseError> {
    let text = reqwest::get(url)
        .await
        .map_err(|e| ParseError::Io(e.to_string()))?
        .text()
        .await
        .map_err(|e| ParseError::Io(e.to_string()))?;

    parse_str(&text)
}
