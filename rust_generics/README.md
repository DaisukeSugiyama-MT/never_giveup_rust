# Rust ジェネリクスについて

Rustのジェネリクスに関するプログラム

## ジェネリクスとは

ジェネリクスとは、抽象的な型を指定して、同じ操作を可能にする機能。
RustではVecやHashMapでジェネリクスを利用するケースがよくある。

## 書式

```
fn 関数名 <T: トレイト>　(引数1: T,引数2: T,...) -> 戻り値の型 {
    ...
}
```

## 参考

The Rust Programming Language 日本語版 ジェネリックなデータ型
(<https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html>)

『手を動かして考えればよくわかる 高効率言語 Rust 書き方・作り方』(著/クジラ飛行机)
(<https://www.socym.co.jp/book/1351>)
