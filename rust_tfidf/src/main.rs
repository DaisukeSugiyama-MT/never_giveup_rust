use encoding::all::ISO_8859_1;
use encoding::DecoderTrap::Strict;
use linfa_preprocessing::tf_idf_vectorization::TfIdfVectorizer;

fn main() {
    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    // 引数がなければ終了
    if args.len() < 2 {
        println!("Usage: {} [file]", args[0]);
        return;
    }
    // フォルダ内のファイル一覧を取得
    let files = std::fs::read_dir(&args[1]).unwrap();

    // ファイル一覧をループして中身を取得
    let text: Vec<String> = files
        .map(|file| {
            //パスの文字列を取得
            let path = file.unwrap().path();
            path.into_os_string().into_string().unwrap()
        })
        .collect();

    // ベクトル化
    let vectorizer = TfIdfVectorizer::default()
        .fit_files(&text, ISO_8859_1, Strict)
        .unwrap();
    println!(
        "We obtain a vocabulary with {} entries",
        vectorizer.nentries()
    );

    // ベクトル化した結果を表示
    let training_records = vectorizer
        .transform_files(&text, ISO_8859_1, Strict)
        .to_dense();

    println!(
        "We obtain a {}x{} matrix of counts for the vocabulary entries",
        training_records.dim().0,
        training_records.dim().1
    );
}
