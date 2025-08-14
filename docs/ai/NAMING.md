# Naming Rules

**目的**: 命名の一貫性を保ち、コードの可読性を向上させる。

---

## 基本規則
- **型名**: `PascalCase`（例: `Renderer`, `CameraController`）
- **関数/変数名**: `snake_case`（例: `update_camera`, `draw_grid`）
- **モジュール/ファイル名**: `snake_case`（例: `grid_renderer.rs`）
- **定数**: `SCREAMING_SNAKE_CASE`（例: `MAX_LIGHTS`）
- **シェーダ変数(WGSL)**:
  - 関数: `vs_main`, `fs_main`
  - Uniform/Storage: `u_ViewProj`, `s_DiffuseTex`
  - 一貫したプレフィックス（`u_`=uniform, `s_`=sampler, `t_`=texture）

---

## 接尾辞・接頭辞
- **Renderer**: 描画専用コンポーネント（例: `GridRenderer`）
- **Controller**: 入力や状態管理を行うもの（例: `CameraController`）
- **State**: データ保持クラス（例: `GameState`）
- **System**: ECS風システム（例: `PhysicsSystem`）

---

## 命名アンチパターン（禁止）
- 意味があいまいな略語（例: `mgr`, `util`）
- コンテキスト不要な短縮（例: `pos` より `position`）
- 日本語ローマ字化（例: `hyouji`, `kesu`）

---

## 特記事項
- 座標系の軸や方向を表す場合は `x`, `y`, `z` をそのまま使う
- 時間は `*_secs`（秒）または `*_ms`（ミリ秒）で単位を明示
