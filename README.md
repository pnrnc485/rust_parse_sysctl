# confparser

Rustで構築された、`sysctl.conf` 形式の設定ファイルをパースするライブラリです。

- ✅ `key = value` 形式に対応
- ✅ コメント行（`#`, `;`）、空行、行頭の `-` を無視
- ✅ ドット区切りのキーをネスト構造に変換
- ✅ 値の最大長（4096文字）チェック
- ✅ JSON形式への変換を提供

---

## 📦 特徴

| 機能名                     | 内容                                                         |
|----------------------------|--------------------------------------------------------------|
| `parse_str(&str)`          | &str から設定をパースし、`BTreeMap<String, String>` を返す   |
| `parse_file(path)`         | ファイルから設定をパース                                     |
| `flatten_to_nested_json()` | `serde_json::Value` に変換                                   |
| `ParseError`               | 行番号・内容・エラー種類を含んだエラー型                    |

---

## 🔧 インストール

```toml
# Cargo.toml
[dependencies]
confparser = { path = "./confparser" } # crates.io に公開後は適宜変更
serde_json = "1.0" # flatten_to_nested_json を使う場合に必要
```

---

## 🚀 使用例

### 1. ファイルから読み込んでパース

```rust
use confparser::parse_file;

fn main() -> Result<(), confparser::ParseError> {
    let config = parse_file("example.conf")?;
    for (k, v) in &config {
        println!("{} = {}", k, v);
    }
    Ok(())
}

```

### 2. ドット区切りのキーを JSON 構造に変換

```rust
use confparser::{parse_str, flatten_to_nested_json};
use serde_json::to_string_pretty;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
        endpoint = localhost:3000
        log.file = /var/log/app.log
        log.level = debug
    "#;

    let flat_map = parse_str(input)?;
    let nested = flatten_to_nested_json(&flat_map);

    println!("{}", to_string_pretty(&nested)?);
    Ok(())
}

```


#### 出力例：
```json
{
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/app.log",
    "level": "debug"
  }
}
```