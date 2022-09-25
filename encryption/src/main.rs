mod encryption;

use std::fmt::Error;

use encryption::encryption;
fn main() -> Result<(), std::fmt::Error> {
    let args: Vec<String> = std::env::args().collect();
    let solt = &args[1];
    let password_str = &args[2];

    if solt.len() <= 16 {
        println!("[error] solt is not 16 characters");
        return Err(*Box::new(Error));
    }
    if password_str.len() < 8 {
        println!("[error] password is not 8 characters");
        return Err(*Box::new(Error));
    }

    // 1.soltを結合
    let password_with_solt = String::from(password_str) + &solt;

    // 2. passwordをハッシュ化
    let mut hash = encryption(&password_with_solt);

    // 3.stretching
    for _n in 0..100000 {
        hash = encryption(&hash);
    }
    println!("hash is:{}", hash);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let password = "password";
        let hash = encryption(password);
        assert_eq!(
            hash,
            "c0067d4af4e87f00dbac63b6156828237059172d1bbeac67427345d6a9fda484"
        );
    }
}
