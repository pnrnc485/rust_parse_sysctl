use confparser::{
    parse_file,
    flatten_to_nested_json,
    ParseError,
};

fn main() -> Result<(), ParseError> {
    // 任意の設定ファイルパス（テスト用）
    let path = "src/sysctl.conf";

    // ファイルを読み込む
    let flat_map = parse_file(path)?;

    println!("✅ フラットなMap:");
    for (k, v) in &flat_map {
        println!("{} = {}", k, v);
    }

    // JSON形式のネストに変換
    let json_nested = flatten_to_nested_json(&flat_map);
    println!("\n✅ ネスト構造 (JSON形式):");
    println!("{}", serde_json::to_string_pretty(&json_nested).unwrap());

    Ok(())
}
