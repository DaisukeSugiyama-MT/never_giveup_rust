# Rust 文字列を暗号化する

任意の文字列をsha-3_256でハッシュ化する。
パスワードのハッシュ値は、「ハッシュ+ストレッチングしたパスワード」に「ハッシュ+ストレッチングしたユーザID」をソルトとして前後ではさんでいる。

## 実行

cargo run で実行する場合
引数に任意のユーザID,パスワードを入力する。
` cargo run {user_id} {password} `;

## ビルド手順

`cargo build --release`
target/releaseディレクトリが作成され、その内部にexeファイルが配置されている。