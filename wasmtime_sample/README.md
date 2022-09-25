# Wasmtime を使ってみる

WebAssembly、WASI用のスタンドアロンのwasm専用最適化ランタイムのWasmtimeを使ってみる。

## wasmファイル作成・実行手順

1. main.rsを作成

2. `rustup target add wasm32-wasi` で　ターゲットプラットフォームをインストールする

3. `rustc .\src\main.rs --target wasm32-wasi` で wasmにコンパイルする
プロジェクトルートディレクトリにmain.wasmが作成される。

4. `wasmtime .\main.wasm` で wasmを実行する

## 参考

[Wasmtime document](<https://docs.wasmtime.dev/introduction.html>)

[rustup クロスコンパイル](https://rust-lang.github.io/rustup/cross-compilation.html?highlight=target%20add#cross-compilation)