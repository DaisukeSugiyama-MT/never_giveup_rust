use base64ct::{Base64, Encoding};
use sha3::{Digest, Sha3_256};

pub fn encryption(password: &str) -> String {
    println!("input password is: {}", password);
    // create a sha3-256 object
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(password);

    let hash = hasher.finalize();

    println!("hash is: {:x}", hash);
    let base64_hash = Base64::encode_string(&hash);
    println!("hex base64 is: {}", base64_hash);

    let hex_hash = base16ct::lower::encode_string(&hash);

    hex_hash
}
