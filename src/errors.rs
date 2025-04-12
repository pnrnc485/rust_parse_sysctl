use std::fmt;

/// confparser のパース処理で発生しうるエラーを表現する列挙型
#[derive(Debug)]
pub enum ParseError {
    /// ファイル読み込みなどのIOエラー
    Io(String),

    /// `key=value` の形式になっていない行
    InvalidLine {
        line_number: usize,
        content: String,
    },

    /// 値が4096文字を超えている（Linuxの仕様準拠）
    ValueTooLong {
        line_number: usize,
        key: String,
        length: usize,
    },
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::Io(e.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(msg) => write!(f, "IO error: {}", msg),
            ParseError::InvalidLine { line_number, content } => {
                write!(f, "Invalid line at {}: '{}'", line_number, content)
            }
            ParseError::ValueTooLong { line_number, key, length } => {
                write!(
                    f,
                    "Value too long at line {}: key '{}' has {} characters (max 4096)",
                    line_number, key, length
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}
