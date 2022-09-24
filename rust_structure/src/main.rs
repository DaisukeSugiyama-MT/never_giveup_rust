// 身長と体重を表す
struct Body {
    height: f64,
    weight: f64,
}

// Body構造体のメソッドを定義する
impl Body {
    fn bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn is_normal(&self) -> bool {
        let bmi = self.bmi();
        bmi >= 18.5 && bmi < 25.0
    }

    fn calc_per(&self) -> f64 {
        self.bmi() / 22.0 * 100.0
    }
}

fn main() {
    let body = Body {
        height: 168.0,
        weight: 60.0,
    };
    println!("BMI: {}", body.bmi());
    println!("isNormal: {}", body.is_normal());
    println!("乖離率: {:.1}%", body.calc_per());
}
