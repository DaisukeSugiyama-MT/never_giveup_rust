//use base64ct::{Base64, Encoding};
use sha3::{Digest, Sha3_256};

pub fn encryption(password: &str) -> String {
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(password);

    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    hex_hash
    // base64　encodeの場合
    //let base64_hash = Base64::encode_string(&hash);
    //println!("hex base64 is: {}", base64_hash);
}
