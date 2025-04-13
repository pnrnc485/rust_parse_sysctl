use confparser::{
    parse_file,
    parse_url_async,
    flatten_to_nested_json,
    ParseError,
};

#[tokio::main]
async fn main() -> Result<(), ParseError> {
    // ---- ファイルから読み込み ----
    let file_path = "src/sysctl.conf";
    println!("📂 ファイル読み込み中: {}", file_path);
    let flat_file = parse_file(file_path)?;

    println!("✅ ファイルのフラットMap:");
    for (k, v) in &flat_file {
        println!("{} = {}", k, v);
    }

    let json_file = flatten_to_nested_json(&flat_file);
    println!("\n✅ ファイルのネスト構造 (JSON形式):");
    println!("{}", serde_json::to_string_pretty(&json_file).unwrap());

    // ---- URLから読み込み ----
    let url = "https://gist.githubusercontent.com/pnrnc485/01d4b192ef7e159b7ed8cf52e87b382a/raw/43cf2bcddf5f2a961e3c4b82f04318d2ea3fb7f6/sample.conf";
    println!("\n🌐 URLから読み込み中: {}", url);
    match parse_url_async(url).await {
        Ok(flat_url) => {
            println!("✅ URLのフラットMap:");
            for (k, v) in &flat_url {
                println!("{} = {}", k, v);
            }

            let json_url = flatten_to_nested_json(&flat_url);
            println!("\n✅ URLのネスト構造 (JSON形式):");
            println!("{}", serde_json::to_string_pretty(&json_url).unwrap());
        }
        Err(e) => {
            eprintln!("❌ URLの読み込みに失敗: {}", e);
        }
    }

    Ok(())
}
