# rust-3d

## For AI readers
- [WorkspaceLayout.md](./WorkspaceLayout.md)を読んで、階層と役割を理解してください。

# Rust で 3D ゲーム作成（wgpu × Phoenix）

Unity のように UI からシーン編集でき、Web/デスクトップ両対応のランタイムを目指すプロジェクトです。
将来的には 複数ユーザーがアバターで同じ 3D 空間に入り、ボイスチャットしながら共同編集できる世界を作ります。

---
## 目標（Goals）
- エディタ UI（Elixir/Phoenix） で Hierarchy/Inspector/Shader を編集 → 即 wgpu 描画に反映
- 共通レンダラ（render）を核に、Desktop(WebGPU/Native) / Web(WebGPU/wasm) で同一結果
- シーンは SceneDoc(JSON) と Patch（差分）で管理し、ホットリロード可能
- 将来：パッケージ（.r3pkg） と プラグイン（Rust crate / WASM）に対応
- 将来：マルチユーザー（Presence + Voice：WebRTC SFU）＆ 分散処理（Phoenix クラスタ）

## リポジトリ構成
```
engine/                 # 起動API・将来RunnerやPackageManagerもここ
render/                 # wgpu共通層（初期化/resize/クリア/今後: pipeline等）
platform/
  desktop/              # winit + native wgpu surface（ランナー）
  web/                  # wasm_bindgen + WebGPU surface（ランナー）
examples/
  desktop/*/            # 最小デスクトップ例
  web/*/                # 最小Web例（wasm-packでビルド）
xtask/                  # 開発ツール（今後: summary/path検査, pkg CLI など）
```
**依存の向き**: `examples → engine → platform_{desktop,web} → render`（循環なし）

## いまの状態（Status）
- 共通 Renderer（render）で 初期化 / リサイズ / クリア描画 が両プラットフォーム一致
- `engine::AppBuilder` で **起動API統一**（Desktop/Web 同じ呼び出しでOK）
- Web は `wasm-pack --target web` でビルドし、そのまま `<script type="module">` で動作
- `//! summary` / `//! path` をファイル先頭に徹底（xtask で検査予定）

## ビルド & 起動
```
# Desktop
cargo run -p integration_min_desktop

# Web（開発用に簡易サーバ）
wasm-pack build examples/web/integration_min --release --target web --out-dir pkg
cd examples/web/integration_min
python -m http.server 8080
```

## アーキテクチャ概要
- engine
  - `AppBuilder`（共通の起動口）
  - 将来: `Runner<Game>`（固定更新 + 毎フレーム描画）、`PackageManager`、`Vfs`
- render
  - `init_with_surface` / `Renderer::resize` / `Renderer::render_clear`
  - 将来: `create_pipeline`, `draw_mesh`, `hot_reload_shader`
- `platform_desktop` / `platform_web`
  - Surface の生成とイベントループだけを担当（描画ロジックは `render` に集約）

## SceneDoc & Patch（最小仕様の草案）
SceneDoc（JSON）例
```
{
  "nodes": {
    "root": { "children": ["cam", "cube"] },
    "cam":  { "components": { "Transform": { "t":[0,1,5] }, "Camera": { "fov":60 } } },
    "cube": { "components": {
      "Transform": { "t":[0,0,0], "r":[0,0,0], "s":[1,1,1] },
      "Mesh": { "src":"asset://meshes/cube.glb#0" },
      "Material": { "shader":"asset://shaders/pbr.wgsl", "params": { "baseColor":[1,0.8,0.7,1] } }
    }}
  }
}
```
Patch（差分）例
```
{ "op":"upsert_node", "id":"cube", "components": { "Transform": { "t":[0,0,1] } } }
{ "op":"set_parent", "id":"cube", "parent":"root", "index":1 }
{ "op":"set_shader", "id":"cube", "shader":"asset://shaders/pbr.wgsl" }
{ "op":"set_material_param", "id":"cube", "param":"baseColor", "value":[0.7,0.9,1,1] }
{ "op":"remove_node", "id":"cube" }
```
**Asset URI** は `asset://...`。実体は Phoenix から配信し、クライアント側（ブラウザ/ネイティブ）でキャッシュ・解決。

## Phoenix（エディタ UI）との連携（MVP）
- UI：Hierarchy / Inspector / Shader エディタ / `<canvas id="canvas">`
- JS/LiveView → wasm ブリッジ関数を呼ぶ：
  - editor_init(canvas_id)
  - editor_load_scene(scene_json)
  - editor_apply_patch(patch_json)
  - editor_register_asset(uri, url_or_inline)
  - editor_try_compile_shader(wgsl) → エラーJSON（line/column/message）
> 将来は同じ SceneDoc/Patch をネイティブにも送って遠隔レンダリング可能（UIは共通のまま）。


## 将来計画（Roadmap）
### 基本描画の拡張
- `Renderer` に **三角形描画パイプライン**（WGSL + 頂点バッファ）を追加
- `Renderer::render_triangle()` を examples から呼び出し
- リサイズ時の 深度バッファ（将来の3D向け）を整備
### エディタ連携（ブラウザMVP）
- `editor_*` wasm ブリッジ追加（上記関数群）
- SceneDoc ロード/保存（Phoenix API）
- WGSL ホットリロード（try→OKなら差し替え→失敗時は前状態）
### パッケージ（Unity 的な取り回し）
- `.r3pkg（zip）`仕様策定：`package.toml` + `assets/` + `prefabs/` + `scenes/`
- `engine::Vfs` / `PackageManager` の最小実装（ローカルZip import→マウント）
- `xtask pkg pack/publish/install/import` の下ごしらえ
- Phoenix に `/api/packages`（メタ＋署名付きURL）を用意
### 入力/ランナー/ゲーム層
- `engine::game_api::{Game, EngineContext, Time}` の導入
- `Runner<Game>` の固定更新＋毎フレーム描画
- `input` 抽象と platform のイベントマッピング（W/A/S/D, マウス等）
### マルチユーザー（Presence ＆ Voice）
- Phoenix Presence で アバターの入退出・座標同期（まずは WebSocket）
- 補間（送信10–30Hz / 表示60fps）
- WebRTC SFU（Membrane/LiveKitなど）でボイスチャット
- クライアント側 空間音響（Web: WebAudio Panner / Native: rodio+HRTF）
### 分散処理（任意）
- 重い処理（ライトベイク等）を Phoenix クラスタへジョブ投入（Oban）
- 成果物を .r3pkg として返し、インポート→SceneDocに反映
