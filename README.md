# rust-3d

## For AI readers
このプロジェクトの設計/規約/参照先は `docs/ai/INDEX.md` に集約済み。
AIで解析する際はまず `docs/ai/PROMPT.md` を読んで回答方針を遵守してください。

## 概要

## Rust インストーラを直接実行
- 公式ページへアクセス
  - https://www.rust-lang.org/ja/tools/install
- 「Windows (64-bit)」の `rustup-init.exe` をダウンロード
- ダウンロードした rustup-init.exe を実行
  - 1) `Proceed with installation (default)` を選択
  - インストール後、PowerShell を再起動
- 動作確認
```
rustup --version
cargo --version
rustc --version
```

## 
```
// Rustで WebAssembly向けにコンパイルできるようにする
rustup target add wasm32-unknown-unknown
// WASM＋フロントエンドの開発を楽にするビルド＆サーバーツール
cargo install trunk
```


##
```
cargo new --lib game_core
```
