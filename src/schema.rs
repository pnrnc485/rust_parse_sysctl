use std::collections::BTreeMap;
use crate::ParseError;

/// スキーマの型を表す列挙型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaType {
    String(Option<usize>), // 最大文字数を指定できるように
    Bool,
    Int,
    Float,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaEntry {
    pub typ: SchemaType,
    pub required: bool,
    pub default: Option<String>,
}

impl SchemaType {
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();

        // string型の処理
        // 例: string(10) のように最大文字数を指定できるようにする
        if let Some(rest) = s.strip_prefix("string(").and_then(|s| s.strip_suffix(")")) {
            let max_len = rest.parse::<usize>().ok();
            return Some(SchemaType::String(max_len));
        }

        // Enum型の処理
        if s.starts_with('[') && s.ends_with(']') {
            let inner = &s[1..s.len() - 1];
            let variants = inner
                .split(',')
                .map(|v| v.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            return Some(SchemaType::Enum(variants));
        }

        match s.to_lowercase().as_str() {
            "string" => Some(SchemaType::String(None)),
            "bool" => Some(SchemaType::Bool),
            "int" => Some(SchemaType::Int),
            "float" => Some(SchemaType::Float),
            _ => None,
        }
    }
}

/// スキーマファイルをパースして BTreeMap に変換する
pub fn parse_schema_str(input: &str) -> Result<BTreeMap<String, SchemaEntry>, ParseError> {
    let raw_map = crate::parser::parse_str(input)?;
    let mut schema = BTreeMap::new();

    for (key, value) in raw_map {
        let mut typ = value.trim();
        let mut required = false;
        let mut default = None;

        // 括弧付きのメタ情報をパース（例: string(required, default=info)）
        if let Some(idx) = value.find('(') {
            let type_part = &value[..idx].trim();
            let meta_part = &value[idx + 1..value.len().saturating_sub(1)].trim(); // 最後の ')'

            typ = type_part;

            for token in meta_part.split(',') {
                let token = token.trim();
                if token.eq_ignore_ascii_case("required") {
                    required = true;
                } else if let Some((key, value)) = token.split_once('=') {
                    if key.trim() == "default" {
                        default = Some(value.trim().to_string());
                    }
                }
            }
        }

        // 型をパース
        let schema_type = SchemaType::from_str(typ).ok_or(ParseError::InvalidLine {
            line_number: 0,
            content: format!("unknown schema type: {}", typ),
        })?;

        schema.insert(key, SchemaEntry {
            typ: schema_type,
            required,
            default,
        });
    }

    Ok(schema)
}

/// スキーマに基づいて設定を検証し、必要に応じて default 値を補完する
pub fn validate_with_schema(
    config: &mut BTreeMap<String, String>,
    schema: &BTreeMap<String, SchemaEntry>,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for (key, entry) in schema {
        match config.get(key) {
            Some(value) => {
                let is_valid = match &entry.typ {
                    SchemaType::String(None) => true,
                    SchemaType::String(Some(max)) => value.len() <= *max,
                    SchemaType::Bool => matches!(value.to_lowercase().as_str(), "true" | "false"),
                    SchemaType::Int => value.parse::<i64>().is_ok(),
                    SchemaType::Float => value.parse::<f64>().is_ok(),
                    SchemaType::Enum(variants) => variants.contains(value),
                };

                if !is_valid {
                    errors.push(format!(
                        "{}: '{}' is not a valid {:?}",
                        key, value, entry.typ
                    ));
                }
            }
            None => {
                if entry.required {
                    errors.push(format!("{}: required field is missing", key));
                } else if let Some(default_value) = &entry.default {
                    // ✅ default 値を補完
                    config.insert(key.clone(), default_value.clone());
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
