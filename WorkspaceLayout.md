# Workspace Layout

- Generated: 2025-08-25 02:12:12
- Root: `D:\Work\FRICK-ELDY\rust-3d`
- Max Depth: none
- Excludes: `.dart_tool, .git, .github, .gitignore, .idea, .vscode, Cargo.lock, README.md, WorkspaceLayout.md, assets, bin, build, dist, docs, node_modules, out, target`

### Legend
- 0–4: ⚪ (無評価)
- 5–50: 🟢 (OK, 保持)
- 51–100: 🟡 (様子見, 早めに分割候補)
- 101–200: 🟠 (分割推奨)
- 200–: 🔴 (最優先で分割)

## Summaries

### 🕹 game
| Path | Lines | Status | Summary |
|------|------:|:------:|---------|
| _no files_ | 0 | - | - |

---

### 💻 platform/desktop
| Path | Lines | Status | Summary |
|------|------:|:------:|---------|
| [platform/desktop/src/lib.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/platform/desktop/src/lib.rs) | 139 | 🟠 | デスクトップ最小ループ（共通 Renderer を利用, AppBuilder からの引数に対応） |

---

### 🌐 platform/web
| Path | Lines | Status | Summary |
|------|------:|:------:|---------|
| [platform/web/src/lib.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/platform/web/src/lib.rs) | 86 | 🟡 | Web最小レンダリング（共通 Renderer を利用）+ アダプタ情報を #msg/console に表示 |

---

### 🎨 render
| Path | Lines | Status | Summary |
|------|------:|:------:|---------|
| [render/src/lib.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/render/src/lib.rs) | 152 | 🟠 | wgpu 初期化・リサイズ・クリア描画の共通実装（Surface を受け取って統一処理） |

---

### 🛠 xtask
| Path | Lines | Status | Summary |
|------|------:|:------:|---------|
| [xtask/src/cli.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/cli.rs) | 33 | 🟢 | clap で CLI 定義 |
| [xtask/src/config.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/config.rs) | 54 | 🟡 | 設定と既定の除外セット |
| [xtask/src/main.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/main.rs) | 27 | 🟢 | CLIエントリーポイント |
| [xtask/src/render/header.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/header.rs) | 28 | 🟢 | 先頭メタ情報とLegend出力 |
| [xtask/src/render/mod.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/mod.rs) | 26 | 🟢 | WorkspaceLayout.md の Markdown 出力 (facade) |
| [xtask/src/render/summaries/classify.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/summaries/classify.rs) | 21 | 🟢 | パスから Section を分類するユーティリティ |
| [xtask/src/render/summaries/collector.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/summaries/collector.rs) | 65 | 🟡 | Nodeツリーを走査してファイル行数・summaryを収集する処理 |
| [xtask/src/render/summaries/mod.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/summaries/mod.rs) | 28 | 🟢 | Summaries表の生成のエントリポイント（セクション収集＋Markdown出力） |
| [xtask/src/render/summaries/model.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/summaries/model.rs) | 42 | 🟢 | Summaries表で使うデータ型（Section, Row）の定義 |
| [xtask/src/render/summaries/writer.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/summaries/writer.rs) | 49 | 🟢 | SectionごとのテーブルMarkdownを組み立てて出力する処理 |
| [xtask/src/render/tree.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/tree.rs) | 51 | 🟡 | Tree 表示（コードブロック） |
| [xtask/src/render/utils.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/render/utils.rs) | 24 | 🟢 | 文字列ユーティリティ |
| [xtask/src/run.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/run.rs) | 37 | 🟢 | layout サブコマンドの実装 |
| [xtask/src/summary.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/summary.rs) | 37 | 🟢 | .rs/.wgsl の1行目から summary を抽出 |
| [xtask/src/tree/build.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/tree/build.rs) | 97 | 🟡 | 走査結果からツリー構築 |
| [xtask/src/tree/mod.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/tree/mod.rs) | 10 | 🟢 | ファイルツリー関連の公開インターフェース |
| [xtask/src/tree/model.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/tree/model.rs) | 12 | 🟢 | ツリーノードのデータ構造 |
| [xtask/src/tree/scan.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/tree/scan.rs) | 46 | 🟢 | ファイルシステム走査（ignoreを尊重） |
| [xtask/src/tree/util.rs](https://github.com/FRICK-ELDY/rust-3d/blob/main/xtask/src/tree/util.rs) | 25 | 🟢 | ツリー構築で使う小物ユーティリティ |

## Directory / File Tree

```
root/
├─ Cargo.toml
├─ LICENSE
├─ engine/
│  ├─ Cargo.toml
│  └─ src/
│     └─ lib.rs — engine の統一 API（AppBuilder）— Desktop/Web を薄くラップ
├─ examples/
│  ├─ desktop/
│  │  └─ integration_min/
│  │     ├─ Cargo.toml
│  │     └─ src/
│  │        └─ main.rs — integration_min (desktop) — AppBuilder で起動
│  └─ web/
│     └─ integration_min/
│        ├─ Cargo.toml
│        ├─ index.html
│        └─ src/
│           └─ lib.rs — integration_min (web) — AppBuilder で起動（canvas_id を受け取れる）
├─ platform/
│  ├─ desktop/
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     └─ lib.rs — デスクトップ最小ループ（共通 Renderer を利用, AppBuilder からの引数に対応）
│  └─ web/
│     ├─ Cargo.toml
│     └─ src/
│        └─ lib.rs — Web最小レンダリング（共通 Renderer を利用）+ アダプタ情報を #msg/console に表示
├─ render/
│  ├─ Cargo.toml
│  └─ src/
│     └─ lib.rs — wgpu 初期化・リサイズ・クリア描画の共通実装（Surface を受け取って統一処理）
└─ xtask/
   ├─ Cargo.toml
   └─ src/
      ├─ cli.rs — clap で CLI 定義
      ├─ config.rs — 設定と既定の除外セット
      ├─ main.rs — CLIエントリーポイント
      ├─ render/
      │  ├─ header.rs — 先頭メタ情報とLegend出力
      │  ├─ mod.rs — WorkspaceLayout.md の Markdown 出力 (facade)
      │  ├─ summaries/
      │  │  ├─ classify.rs — パスから Section を分類するユーティリティ
      │  │  ├─ collector.rs — Nodeツリーを走査してファイル行数・summaryを収集する処理
      │  │  ├─ mod.rs — Summaries表の生成のエントリポイント（セクション収集＋Markdown出力）
      │  │  ├─ model.rs — Summaries表で使うデータ型（Section, Row）の定義
      │  │  └─ writer.rs — SectionごとのテーブルMarkdownを組み立てて出力する処理
      │  ├─ tree.rs — Tree 表示（コードブロック）
      │  └─ utils.rs — 文字列ユーティリティ
      ├─ run.rs — layout サブコマンドの実装
      ├─ summary.rs — .rs/.wgsl の1行目から summary を抽出
      └─ tree/
         ├─ build.rs — 走査結果からツリー構築
         ├─ mod.rs — ファイルツリー関連の公開インターフェース
         ├─ model.rs — ツリーノードのデータ構造
         ├─ scan.rs — ファイルシステム走査（ignoreを尊重）
         └─ util.rs — ツリー構築で使う小物ユーティリティ
```

