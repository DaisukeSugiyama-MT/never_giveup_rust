use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() {
    //write_file(10000000);

    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    // 引数がなければ終了
    if args.len() < 2 {
        println!("Usage: {} [file]", args[0]);
        return;
    }
    // pathを取得
    let path = &args[1];

    let mut count = 0;
    // 時間計測開始
    let start = std::time::Instant::now();

    // ファイルを開く
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    // ループ
    for line in reader.lines() {
        //line.unwrap();
        count += 1;
    }
    // 時間計測終了
    let end = start.elapsed();
    println!("{} lines read in {:?}", count, end);
}

// 指定行数のファイルを作成
fn write_file(line_count: usize) {
    let path = "test.txt";
    let mut file = File::create(path).unwrap();

    let s: String = format!("{}\n", "rust,samplelogs,hello,world!").repeat(line_count);

    file.write_all(s.as_bytes()).unwrap();
}
