use rand::{distributions::Alphanumeric, Rng};
use std::io::Write;

fn main() {
    println!("[start create file]");
    let mut file = std::fs::File::create("pass.txt").unwrap();

    // ランダムな文字列を生成
    let mut rng = rand::thread_rng();
    let s: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();

    file.write_all(s.as_bytes()).unwrap();
    println!("password:{}", s);
    println!("[end create file]");
}
