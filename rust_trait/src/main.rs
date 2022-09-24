// ダイアログトレイト
trait Dialog {
    fn show(&self);

    // ↓はデフォルトメソッドで、各メソッドの実装には自動で追加される
    fn close(&self) {
        println!("{}", "close");
    }
}

// アラートダイアログの構造体
struct AlertDialog {
    title: String,
    message: String,
}

// ダイアログトレイトの実装(アラートダイアログ)
impl Dialog for AlertDialog {
    fn show(&self) {
        println!("[Alert!]{}: {}", self.title, self.message);
    }
}

// インフォダイアログの構造体
struct InfoDialog {
    title: String,
    message: String,
}

// ダイアログトレイトの実装(インフォダイアログ)
impl Dialog for InfoDialog {
    fn show(&self) {
        println!("[Info]{}: {}", self.title, self.message);
    }
}

fn main() {
    println!("Hello, world!");
    let alert = AlertDialog {
        title: "Alert".to_string(),
        message: "Alert Message".to_string(),
    };
    let info = InfoDialog {
        title: "Info".to_string(),
        message: "Info Message".to_string(),
    };
    alert.show();
    alert.close();

    info.show();
    info.close();
}
