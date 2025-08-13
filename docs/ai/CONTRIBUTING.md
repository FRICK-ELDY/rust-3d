## 1. コーディング規約

- 公式Rustスタイルガイド（[Rust公式スタイル](https://doc.rust-lang.org/1.0.0/style/)）に従う
- インデントはスペース2つ
- 1行は80〜120文字以内を目安にする
- 変数や関数には型推論を活用しつつ、可読性を優先する
- コメントは英語で簡潔に書く
- `unsafe`は極力使わない。使う場合は理由を明記する
- 外部クレートはCargo.tomlで明示し、バージョンを固定する

## 2. 設計方針

- モジュール分割を意識し、責任範囲ごとにcrateやmoduleを分ける
- 共通処理は`game`、描画は`render`、プラットフォーム依存は`platform`に配置
- 設定値やリソースは`assets`ディレクトリで一元管理
- エラー処理は`Result`型や`Option`型を活用し、パニックを避ける
- テスト可能な設計（関数の分割、依存の注入など）を心がける

## 3. 命名規則

- 変数・関数：`snake_case`（例: `player_position`、`update_score`）
- 構造体・列挙体・トレイト：`CamelCase`（例: `GameState`、`RenderEngine`）
- 定数・静的変数：`SCREAMING_SNAKE_CASE`（例: `MAX_PLAYER_COUNT`）
- モジュール・ファイル名：`snake_case`
- テスト関数：`test_`で始める（例: `test_player_move`）
