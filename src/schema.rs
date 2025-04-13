use std::collections::BTreeMap;
use crate::ParseError;

/// スキーマの型を表す列挙型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaType {
    String,
    Bool,
    Int,
    Float,
}

impl SchemaType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" => Some(SchemaType::String),
            "bool" => Some(SchemaType::Bool),
            "int" => Some(SchemaType::Int),
            "float" => Some(SchemaType::Float),
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

pub fn validate_with_schema(
    config: &BTreeMap<String, String>,
    schema: &BTreeMap<String, SchemaType>,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for (key, value) in config {
        if let Some(expected_type) = schema.get(key) {
            let is_valid = match expected_type {
                SchemaType::String => true,
                SchemaType::Bool => matches!(value.to_lowercase().as_str(), "true" | "false"),
                SchemaType::Int => value.parse::<i64>().is_ok(),
                SchemaType::Float => value.parse::<f64>().is_ok(),
            };

            if !is_valid {
                errors.push(format!(
                    "{}: '{}' is not a valid {:?}",
                    key, value, expected_type
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
