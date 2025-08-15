# Codegen Rules for AI

**目的**: コード品質の一貫性と保守性を確保し、誰が見ても理解しやすく修正しやすい状態を保つ。

---

## 基本方針
- Rust 1.79+ / Edition 2021 を使用
- **Clippy 警告ゼロ**を維持（`cargo clippy -- -D warnings`）
- **rustfmt 適用済み**（`cargo fmt --all`）
- 可読性 > 短縮化（短いが意味が曖昧なコードは禁止）
- 依存関係は最小限に保つ（不要ライブラリを安易に追加しない）

---

## エラーハンドリング
- 失敗しうる関数は `anyhow::Result<T>` または `Result<T, E>` を返す
- エラー発生時は `context()` を付けて原因を明示
- `unwrap` / `expect` は起動初期など致命的ケースのみ許可
- 回復不能なエラーは `bail!` で早期終了

---

## ログ
- ログには `tracing` クレートを使用
- レベルの使い分け:
  - `trace!` … 毎フレームや高頻度出力
  - `debug!` … 開発時の確認
  - `info!` … 重要な状態変化
  - `warn!` … 潜在的な問題
  - `error!` … 明確な不具合

---

## 構造体とモジュール
- 1ファイル1主要構造体または密接関連構造体
- 構造体のフィールドは基本 `pub` にしない（getter/setter経由）
- 関連関数は `impl` 内にまとめ、無関係な free function は作らない

---

## 関数
- 1関数は**1画面（約50行）以内**を目安
- 引数が3つ以上になる場合は構造体にまとめる
- 命名は動詞＋目的語（例: `update_camera`, `draw_grid`）
- 副作用のない処理は `&self` または `&mut self` の明確な意味付け

---

## コメント
- 公開関数・型には必ず **doc comment (`///`)** を付ける
- 実装の意図やアルゴリズムは `//` コメントで簡潔に説明
- 複雑な処理は冒頭に処理概要コメントを記述

---

## 禁止事項
- `unsafe` の直接使用（やむを得ない場合は理由を明記）
- グローバル可変状態（`static mut`）
- 関数・型の過剰なジェネリクス化による可読性低下
- 意味不明なマジックナンバー（定数化する）

---

## テスト
- 単体テストは可能な限り `#[cfg(test)]` で同ファイル内に配置
- プラットフォーム依存コードは条件付きコンパイルで分離
- バグ修正時は必ず再発防止のテストを追加

---

## 更新ルール
- 設計変更や大きな仕様追加は `DECISIONS.md` に記録
- 規約と実装が矛盾する場合は **最新コードを優先**し、修正提案を記録

## Dependencies
- 追加するクレートは `Cargo.toml` ではなく **root の `[workspace.dependencies]` に追記**。
- 各 crate 側は `xxx = { workspace = true }` を使う（例: `winit`, `anyhow`, `pollster`, `serde`, `toml`）。

## File Placement
- GPU 周り: `render/src/gpu/`
- 新しい描画パイプライン: `render/src/pipelines/<name>.rs` + 必要なら `shaders/<name>/`
- 1 パス複数パイプラインの場合は `render/src/passes/` に統括ロジックを置く
- シーン要素は `render/src/scene/` に追加

## Winit 0.30（Desktop）
- エントリは `ApplicationHandler`（`resumed` で Window/Renderer 初期化、`window_event` で resize/close）。
- 直接 `EventLoop::run` を書かない。**`resumed`/`window_event` に分離**する。

## Shader
- WGSL ファイルは `render/shaders/<group>/` に配置。ローダから相対パスで取得できる構造を維持。
- 共通定義は `shaders/common/` に。

## Config
- ゲーム設定は `GameConfig` に追加し、**TOML から読み込める**よう `serde` で derive する。
