use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    //　引数を取得
    let args: Vec<String> = std::env::args().collect();
    let _json = &args[1];

    let json_data: Value = serde_json::from_str(_json).unwrap();

    // get keys
    let keys = json_data
        .as_object()
        .unwrap()
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // get values
    let values = json_data
        .as_object()
        .unwrap()
        .values()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // print out keys and values
    println!("{}", keys);
    println!("{}", values);

    println!("{:?}", json_data);
}
