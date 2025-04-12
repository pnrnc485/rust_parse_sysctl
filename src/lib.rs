use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ParseError {
  Io(String),
  InvalidLine {
      line_number: usize,
      content: String,
  },
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err.to_string())
    }
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          ParseError::Io(msg) => write!(f, "IO error: {}", msg),
          ParseError::InvalidLine { line_number, content } => {
              write!(f, "Invalid line at {}: '{}'", line_number, content)
          }
      }
  }
}

impl std::error::Error for ParseError {}

pub fn parse_str(input: &str) -> Result<BTreeMap<String, String>, ParseError> {
    let mut map = BTreeMap::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
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
      
        map.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(map)
}

/// ファイルパスを受け取り、ファイル内容をパースする
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, String>, ParseError> {
  let content = fs::read_to_string(&path)?;
  parse_str(&content)
}