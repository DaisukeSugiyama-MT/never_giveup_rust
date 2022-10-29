mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, search-address!");
}

#[derive(Serialize, Deserialize)]
struct Address {
    address1: String,
    address2: String,
    address3: String,
    kana1: String,
    kana2: String,
    kana3: String,
    n1: String,
    n2: String,
    n3: String,
    n4: String,
    n5: String,
    n6: String,
    pref: String,
    zip: String,
    zipcode: String,
}

#[derive(Serialize, Deserialize)]
struct AddressList {
    address_list: Vec<Address>,
}
const DATA: &str = include_str!("../data/address.json");

#[wasm_bindgen]
pub fn search_address(s: &str) -> JsValue {
    //use web_sys::console;
    //時間計測
    let start = std::time::Instant::now();
    //console::log_1(&JsValue::from_str(s));
    // 住所JSONの読み込み
    let address: AddressList = serde_json::from_str::<AddressList>(&DATA).unwrap();
    // 計測終了
    let end = start.elapsed();
    //計測結果
    println!("1 elapsed: {:?}", end);

    // 計測開始
    let start = std::time::Instant::now();
    // 検索
    let filterd: Vec<Address> = address
        .address_list
        .into_iter()
        .filter(|x| x.zipcode.contains(s))
        .collect();
    // 計測終了
    let end = start.elapsed();
    println!("2 elapsed: {:?}", end);
    /*console::log_1(&JsValue::from_str(&format!(
        "検索時間: {}.{:03}秒",
        end.as_secs(),
        end.subsec_millis()
    )));
    */
    if filterd.len() < 10 {
        return serde_wasm_bindgen::to_value(&filterd).unwrap();
    }
    let sliced = &filterd[0..10];
    serde_wasm_bindgen::to_value(&sliced).unwrap()
    //filterdをreturnする
}

#[test]
fn test_search_address() {
    let result = search_address("64");
    println!("{:?}", result);
}
