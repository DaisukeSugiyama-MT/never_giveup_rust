mod common;
mod sample;

fn main() {
    println!("Hello, world!");
    let count_common = common::add(1, 2);
    let count = sample::calc();
    let count_sub = sample::calc_sub();
    println!("count_common: {}", count_common);
    println!("count: {}", count);
    println!("count_sub: {}", count_sub);
}
