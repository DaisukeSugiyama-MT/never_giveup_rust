fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// ↑と同じ whereを使って書き直したもの
fn add_where<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1.1, 2.1));
    println!("{}", add_where(1.1, 2.1));
    // エラー
    // println!("{}", add("Hello", "World"));
}
