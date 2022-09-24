mod encryption;

use encryption::encryption;
fn main() {
    let password = "Hello, world!";

    let hash = encryption(password);
    println!("hash is: {}", hash);
}
