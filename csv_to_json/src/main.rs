use std::{
    cmp,
    collections::BTreeMap,
    io::{stdout, BufWriter, Write},
    process,
};

use serde::{Deserialize, Serialize};

type MyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> MyResult {
    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    // 引数が2つでなければエラー
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    //
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // ファイルを開く
    let file = std::fs::read(filename)?;

    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
    let text = res.into_owned();

    //ファイルに書き出す
    //let mut file = std::fs::File::create("out.csv")?;
    // file.write(text.as_bytes())?;

    //CSV文字列をJSONに変換
    let mut rdr = csv::Reader::from_reader(text.as_bytes());
    let mut wtr = csv::Writer::from_writer(out);
    // csvファイルの一行目を取得する
    let headers = rdr.headers()?;
    // csvファイルの一行目をJSONのキーにする
    let keys = headers
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut json_vec = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let line = record
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let cols = split_line(&line);
        let json = to_json_obj(&keys, &cols);

        json_vec.push(json);
    }

    // json_vevを文字列に変換
    let json_str: String = json_vec
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join(",\n");

    let mut file = std::fs::File::create("address.json")?;
    file.write(json_str.as_bytes())?;

    wtr.flush()?;

    Ok(())
}

fn to_json_obj(keys: &Vec<String>, cols: &Vec<String>) -> String {
    let len = cmp::min(keys.len(), cols.len());
    let mut m = BTreeMap::new();

    for i in 0..len {
        m.insert(&keys[i], &cols[i]);
    }

    serde_json::to_string(&m).unwrap()
}

fn split_line(line: &str) -> Vec<String> {
    if line == "" {
        return vec![];
    }
    line.split(',').map(|s| s.to_string()).collect()
}
