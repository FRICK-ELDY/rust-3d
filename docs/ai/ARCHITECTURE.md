# Project Architecture (for AI)

**目的**: このドキュメントは、リポジトリ全体の構成、各レイヤーの責務、データフロー、依存関係ルールを明確化する。

---

## Workspace Layout
- `game/` … ゲーム状態と設定（`GameConfig`）、ロジック
- `render/` … WGPU レンダラ本体
  - `gpu/` … Surface/Device/Queue/Buffer/Texture/BindGroup/Shader/PipelineCache
  - `io/` … mesh/shader/texture ローダ
  - `math/` … Transform 等
  - `passes/` … レンダリングパス（例: `main_opaque`）
  - `pipelines/` … パイプライン単位（`grid.rs`, `gizmo.rs` など）
  - `scene/` … カメラ・マテリアル・メッシュ等のシーン表現
  - `platform/` … `desktop.rs` / `web.rs` のブリッジ
  - `shaders/` … `common/`, `grid/`, `gizmo/` 等に配置
- `platform/desktop/` … `winit 0.30` の `ApplicationHandler` ベースで起動
- `platform/web/` … WASM/Trunk 用の web エントリ

## Cross-Crate Rules
- 依存は **`[workspace.dependencies]` に統一**。各 crate 側では `foo = { workspace = true }` を使う。
- `render` はプラットフォーム非依存。Window 由来のリソースは `render::platform::{desktop,web}` を経由。

---

## データフロー概要
入力デバイス → Platform 層(winit等) → Game 層(InputState更新) → Game Systems(update) → Render 層(draw) → 出力（画面）

---

## 座標系・基本前提
- 座標系: 右手座標系
- 上方向: Y+
- 前方向: Z-
- カラースペース: sRGB
- 時間管理: 可変Δt（将来固定Δtサポート予定）

---

## 将来の拡張予定
- RenderGraph 化によるレンダリングパス柔軟化
- ECS ライブラリ導入検討（`hecs` or `bevy_ecs`）
- Phoenix などによる Web 配信統合
