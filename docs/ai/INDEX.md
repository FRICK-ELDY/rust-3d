# AI Reading Index

**目的**: このリポジトリの構成・規約・参照先をAIと人間が最短で把握するための総目次。

---

## 重要ドキュメント
- 設計全体: [ARCHITECTURE.md](../ARCHITECTURE.md)
- 命名規約: [NAMING.md](../NAMING.md)
- コーディング規約: [CODING_RULES.md](../CODING_RULES.md)
- Lint方針: [LINTING.md](../LINTING.md)
- 描画パイプライン: [RENDER_PIPELINE.md](../RENDER_PIPELINE.md)
- シェーダ規約: [SHADER_GUIDE.md](../SHADER_GUIDE.md)
- よくある誤解と正解: [QA_HINTS.md](../QA_HINTS.md)
- ファイル検索マップ: [SEARCH_MAP.md](../SEARCH_MAP.md)
- 意思決定ログ: [DECISIONS.md](../DECISIONS.md), [ADR/](../ADR/)

---

## コードの入口ポイント
- **レンダ初期化**: `render/src/renderer.rs`
- **カメラ制御**: `render/src/camera.rs`, [../CAMERA.md](../CAMERA.md)
- **Web起動**: `platform/web/src/lib.rs`
- **デスクトップ起動**: `platform/desktop/src/main.rs`
- **最小サンプル**: `examples/draw_grid.rs`, `examples/cube_basic.rs`

---

## 座標系と基本前提
- 座標系: **右手座標系**
- Y+: 上方向
- Z-: 前方向
- カラースペース: sRGB
- 基本タイムステップ: 可変Δt（将来固定Δt対応予定）
