use proconio::input;
use std::collections::HashMap;
fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("N".to_string(), "S".to_string());
    map.insert("E".to_string(), "W".to_string());
    map.insert("W".to_string(), "E".to_string());
    map.insert("S".to_string(), "N".to_string());
    map.insert("NE".to_string(), "SW".to_string());
    map.insert("NW".to_string(), "SE".to_string());
    map.insert("SE".to_string(), "NW".to_string());
    map.insert("SW".to_string(), "NE".to_string());

    input! {
        d: String,
    }

    println!("{}", map[&d])
}