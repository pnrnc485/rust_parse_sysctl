use std::collections::BTreeMap;
use crate::ParseError;

/// スキーマの型を表す列挙型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaType {
    String,
    Bool,
    Int,
}

impl SchemaType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" => Some(SchemaType::String),
            "bool" => Some(SchemaType::Bool),
            "int" => Some(SchemaType::Int),
            _ => None,
        }
    }
}

pub fn parse_schema_str(input: &str) -> Result<BTreeMap<String, SchemaType>, ParseError> {
    let raw_map = crate::parser::parse_str(input)?;
    let mut schema = BTreeMap::new();

    for (key, type_str) in raw_map {
        let type_str = type_str.to_string();
        let Some(schema_type) = SchemaType::from_str(&type_str) else {
            return Err(ParseError::InvalidLine {
                line_number: 0,
                content: format!("unknown schema type: {}", type_str),
            });
        };
        schema.insert(key, schema_type);
    }

    Ok(schema)
}