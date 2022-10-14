use std::{fs::File, io::Write};

fn main() {
    //write_file(10000000);

    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    // 引数が足りない場合はエラーを出力して終了
    if args.len() < 2 {
        eprintln!("引数が足りません");
        std::process::exit(1);
    }

    // 引数からファイルパスを取得
    let path = &args[1];

    // 時間計測開始
    let start = std::time::Instant::now();

    // ファイルを文字列として読み込む
    let contents = std::fs::read_to_string(path).expect("ファイルが見つかりません");

    // ファイルの内容を改行でスプリットする
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut count = 0;
    // ファイルの内容を出力
    for line in lines {
        // 文字列を逆順にする
        let rev = line.chars().rev().collect::<String>();
        //println!("{}", line);
        //count += 1;
    }

    // 時間計測終了
    let end = start.elapsed();
    println!("{}.{:03}秒", end.as_secs(), end.subsec_millis());
}

// 指定行数のファイルを作成
fn write_file(line_count: usize) {
    let path = "test.txt";
    let mut file = File::create(path).unwrap();

    let s: String = (0..line_count)
        .map(|_| "rust,samplelogs,hello,world!\n")
        .collect();

    file.write_all(s.as_bytes()).unwrap();
}
