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