mod random_str;

pub use crate::random_str::genarete_rnd_str;
fn main() {
    println!("genarate random string");
    let rnd_str = genarete_rnd_str(10);
    println!("{}", rnd_str);
}
