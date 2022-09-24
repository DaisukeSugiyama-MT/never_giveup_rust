mod encryption;

use encryption::encryption;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_id = &args[1];
    let password_str = &args[2];

    // 1. user_idをハッシュ化
    let mut user_id_hash = encryption(user_id);

    // 2.stretching
    for _n in 0..100000 {
        user_id_hash = encryption(&user_id_hash);
    }

    // 前後にsolt付ける
    let mut password_with_solt = String::from(password_str) + &user_id_hash;
    password_with_solt = String::from(&user_id_hash) + &password_with_solt;

    // 3. passwordをハッシュ化
    let mut hash = encryption(&password_with_solt);
    // stretching
    for _n in 0..100000 {
        hash = encryption(&hash);
    }
    println!("hash is: {}", hash);
}
