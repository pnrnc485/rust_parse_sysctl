
# confparser

Rust で構築された、`sysctl.conf` 形式の設定ファイルをパースするライブラリです。

- ✅ `key = value` 形式に対応
- ✅ コメント行（`#`, `;`）、空行、行頭の `-` を無視
- ✅ ドット区切りのキーをネスト構造に変換
- ✅ 値の最大長（4096 文字）チェック
- ✅ JSON 形式への変換を提供
- ✅ **URL からの非同期取得とパースにも対応（`parse_url_async`）**

---

## 📦 特徴

| 機能名                     | 内容                                                       |
| -------------------------- | ---------------------------------------------------------- |
| `parse_str(&str)`          | &str から設定をパースし、`BTreeMap<String, String>` を返す |
| `parse_file(path)`         | ファイルから設定をパース                                   |
| `parse_url_async(url)`     | 非同期で URL から設定を取得してパース                      |
| `flatten_to_nested_json()` | `serde_json::Value` に変換                                 |
| `validate_with_schema()`   | スキーマに従って型や存在チェック、デフォルト補完を実行     |
| `ParseError`               | 行番号・内容・エラー種類を含んだエラー型                   |

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
│   ├── schema.rs
│   └── errors.rs
├── conf/                     # ← サンプルやスキーマファイル
│   ├── sysctl.conf
│   └── schema.conf
├── tests/
│   ├── converter_test.rs
│   ├── parser_test.rs
│   ├── parser_async_test.rs
│   └── schema_test.rs

```


# 🧩 スキーマファイル仕様（Schema File Specification）
.conf 形式のスキーマファイルで、設定ファイルの型検証・必須項目チェック・デフォルト補完が可能です。


### ✅ 対応しているデータ型

| 型名               | 説明                                                                 | 使用例                                     |
|--------------------|----------------------------------------------------------------------|--------------------------------------------|
| `string`           | 任意の文字列                                                         | `username = string`                        |
| `string(N)`        | 最大N文字までの文字列                                                | `title = string(50)`                       |
| `bool`             | 真偽値：`true` / `false`（大文字小文字どちらでも可）               | `debug = bool`                             |
| `int`              | 整数値（64bit）                                                      | `timeout = int`                            |
| `float`            | 浮動小数点数（64bit）                                                | `rate = float`                             |
| `["A", "B", ...]`  | 列挙型：指定した値のいずれかである必要あり                         | `mode = ["auto", "manual", "self"]`        |

---

### ⚙️ オプション指定（任意）

スキーマの型指定の後に括弧 `()` を使ってオプション制約を追加できます。

| オプション            | 内容                                                                             | 使用例                                           |
|-----------------------|----------------------------------------------------------------------------------|--------------------------------------------------|
| `required`            | この項目は必須。設定ファイルに存在しない場合はエラーになります                 | `log.file = string(required)`                    |
| `default=値`          | 設定ファイルに存在しない場合、このデフォルト値が自動で挿入されます              | `log.level = string(default=info)`               |
| 両方を併用可能        | カンマ区切りで複数の制約を指定可能です                                         | `timeout = int(required, default=30)`            |

---


### 📄 スキーマ記述例（`schema.conf`）

```c
log.file = string(required)
log.level = string(default=info)
debug = bool(default=false)
timeout = int
rate = float(default=0.5)
mode = ["auto", "manual", "self"]
desc = string
```


### ⚠️ バリデーションルールまとめ

| チェック内容                            | 条件・動作                                                                 |
|-----------------------------------------|----------------------------------------------------------------------------|
| **必須フィールドの欠落**                 | `required` が指定されたキーが設定ファイルに存在しない場合はエラー         |
| **デフォルト値の補完**                   | `default=値` が指定されているキーが設定ファイルにない場合は補完される     |
| **型不一致**                             | 値が `bool`, `int`, `float`, `Enum` などの型と一致しない場合はエラー       |
| **最大文字数超過**                       | `string(N)` 型で N 文字を超えている場合はエラー                            |
| **Enum 値以外の指定**                    | 例: `mode = semi` のように `["auto", "manual"]` に含まれない値はエラー     |
| **構文エラー**                           | `key = value` の形式でない行はエラー (`=` がない・左辺が空など)           |


## 📝 備考

```
このリポジトリは、sysctl.conf 互換の設定ファイルを対象としたパーサライブラリ。
構文解析・構造変換・エラー処理・非同期対応・テストカバレッジの観点で実装を行いました。
また、拡張性やメンテナンス性を意識したモジュール分割とAPI設計を行っています。
```