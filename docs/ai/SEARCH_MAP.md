# Search Map – ファイル逆引き辞典

**目的**: 特定の機能や実装箇所を素早く見つけるための逆引きマップ。  
AI・開発者が迷わず該当ファイルへジャンプできるようにする。

---

## 初期化・起動処理
- **デスクトップ起動**: `platform/desktop/src/main.rs`
- **Web起動（WASM）**: `platform/web/src/lib.rs`
- **レンダラー初期化**: `render/src/renderer.rs::new()`
- **ゲームアプリ初期化**: `game/src/app.rs::new()`

---

## 入力処理
- **入力イベント受け取り**: `platform/*` の winit イベントループ
- **入力状態管理**: `game/src/input.rs`
- **カメラ操作入力**: `game/src/systems/camera_control.rs`

---

## 描画
- **グリッド描画**: `render/src/grid.rs`
- **カメラ行列計算**: `render/src/camera.rs`
- **シェーダパイプライン**: `render/src/pipeline.rs`
- **メッシュ読み込み/描画**: `render/src/mesh.rs`

---

## ゲームロジック
- **シーン構造**: `game/src/scene.rs`
- **ECS風システム**: `game/src/app.rs`（`systems` 登録部分）
- **時間管理**: `game/src/time.rs`

---

## アセット
- **シェーダ**: `assets/shaders/*.wgsl`
- **サンプルモデル**: `assets/models/*`
- **テクスチャ**: `assets/textures/*`

---

## ビルド・実行
- **Web 実行**: `bin/win/run.bat web` または `trunk serve`
- **デスクトップ実行**: `cargo run -p app_desktop`
- **CI 設定**: `.github/workflows/ci.yml`

---

## ドキュメント
- **設計概要**: `docs/ai/ARCHITECTURE.md`
- **命名規則**: `docs/ai/NAMING.md`
- **コーディング規約**: `docs/ai/CODING_RULES.md`
- **Lint方針**: `docs/ai/LINTING.md`
- **よくある誤解と正解**: `docs/ai/QA_HINTS.md`

---

## 追加方法
- 新しい機能を追加したら必ずここに記載
- エントリは「機能名 → ファイルパス」の形式で、なるべく粒度を揃える
