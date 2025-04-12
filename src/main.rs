use confparser::{parse_file};

fn main() {
    let result = parse_file("src/sysctl.conf");
    match result {
        Ok(config) => {
            for (key, value) in config {
                println!("{} = {}", key, value);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }    
}
