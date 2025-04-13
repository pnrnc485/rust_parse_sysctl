## 📝 備考
```
このリポジトリは、sysctl.conf 互換の設定ファイルを対象としたパーサライブラリ。
構文解析・構造変換・エラー処理・非同期対応・テストカバレッジの観点で実装を行いました。  
また、拡張性やメンテナンス性を意識したモジュール分割とAPI設計を行っています。
```

# confparser

Rustで構築された、`sysctl.conf` 形式の設定ファイルをパースするライブラリです。

- ✅ `key = value` 形式に対応
- ✅ コメント行（`#`, `;`）、空行、行頭の `-` を無視
- ✅ ドット区切りのキーをネスト構造に変換
- ✅ 値の最大長（4096文字）チェック
- ✅ JSON形式への変換を提供
- ✅ **URL からの非同期取得とパースにも対応（`parse_url_async`）**

---

## 📦 特徴

| 機能名                     | 内容                                                         |
|----------------------------|--------------------------------------------------------------|
| `parse_str(&str)`          | &str から設定をパースし、`BTreeMap<String, String>` を返す   |
| `parse_file(path)`         | ファイルから設定をパース                                     |
| `parse_url_async(url)`     | 非同期でURLから設定を取得してパース                         |
| `flatten_to_nested_json()` | `serde_json::Value` に変換                                   |
| `ParseError`               | 行番号・内容・エラー種類を含んだエラー型                    |

---

## 🔧 インストール

```toml
# Cargo.toml
[dependencies]
confparser = { path = "./confparser" } # crates.io に公開後は適宜変更
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
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

### 2. ネスト構造に変換(JSON)

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

### 3. URL から非同期でパース

```rust
use confparser::parse_url_async;

#[tokio::main]
async fn main() -> Result<(), confparser::ParseError> {
    let url = "https://sample.com/sample.conf";
    let map = parse_url_async(url).await?;

    for (k, v) in &map {
        println!("{} = {}", k, v);
    }

    Ok(())
}

```

### 4. エラーハンドリング
```rust
use confparser::{parse_str, ParseError};

fn main() {
    let input = "invalid line without equals";

    match parse_str(input) {
        Ok(_) => println!("Parsed OK"),
        Err(ParseError::InvalidLine { line_number, content }) => {
            eprintln!("構文エラー（{}行目）: {}", line_number, content);
        }
        Err(e) => eprintln!("その他のエラー: {}", e),
    }
}

```

### 5.　ディレクトリ構成
```
confparser/
├── src/
│   ├── lib.rs
│   ├── parser.rs
│   ├── parser_async.rs    # ← URL対応の非同期パーサ
│   ├── converter.rs
│   └── error.rs
├── tests/
│   ├── sysctl_format.rs
│   ├── parse_url.rs       # ← 非同期パースのテスト
│   └── fixtures/
│       └── test_sysctl.conf

```