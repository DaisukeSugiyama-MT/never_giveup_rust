use rayon::prelude::*;
use std::{fs::File, io::Write};

fn main() {
    //write_file(10000000);

    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    // 引数がなければ終了
    if args.len() < 2 {
        println!("引数がありません");
        return;
    }
    // pathを取得
    let path = &args[1];

    // 時間計測開始
    let start = std::time::Instant::now();

    // ファイルを文字列で読み込む
    let contents = std::fs::read_to_string(path).unwrap();
    // 改行で分割
    let lines: Vec<&str> = contents.split("\n").collect();

    // 並列処理
    lines.par_iter().for_each(|line| {
        // 文字列を逆順にする
        let rev = line.chars().rev().collect::<String>();
    });
    // 時間計測終了
    let end = start.elapsed();
    println!("{}.{:03}秒", end.as_secs(), end.subsec_millis());
}

// 指定行数のファイルを作成する
fn write_file(line: usize) {
    let path = "test.txt";
    let mut file = File::create(path).unwrap();

    let s: String = (0..line).map(|_| "rust,samplelogs,hello,world\n").collect();

    file.write_all(s.as_bytes()).unwrap();
}
