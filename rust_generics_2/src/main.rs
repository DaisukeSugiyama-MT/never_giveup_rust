// 構造体のフィールドの各値をprintln!で出力できるようになる
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    // コンストラクタ
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // 加算
    fn add(&mut self, other: Point<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let mut float = Point { x: 3.0, y: 4.0 };
    let int = Point { x: 3, y: 4 };
    let str = Point {
        x: "16th",
        y: "alphabet",
    };

    println!("float p is {:?}", float);
    println!("int p is {:?}", int);
    println!("str p is {:?}", str);

    float.add(Point { x: 1.0, y: 2.0 });
    println!("after add float p is {:?}", float);
}
