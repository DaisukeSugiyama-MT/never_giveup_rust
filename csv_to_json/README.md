# CSVファイルをJSONファイルに変換して出力する

CSVファイル(Shift-JIS)をJSONファイルにして出力する

## 実行

`cargo run {CSVのファイルパス}`


## 

実行時、以下エラーメッセージが表示される。

```

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Utf8 { pos: Some(Position { byte: 0, line: 1, record: 0 }), err: Utf8Error { field: 3, valid_up_to: 2 } })', src\main.rs:16:2
```

恐らくUTF-8ではなく、Shift-JISのCSVファイルを読み込んだためと思われる。


## 参考

Shift_JISからUTF-8に変換する
(https://note.com/kokoronopython/n/n191c2dc1f79f)

CSVをJSONに変換する
(https://github.com/winebarrel/xjr)