# Linting Rules

**目的**: Lint による品質基準を統一し、コードスタイルや安全性のバラつきを防ぐ。

---

## 使用ツール
- **Clippy**: 静的解析（`cargo clippy -- -D warnings`）
- **rustfmt**: 自動整形（`cargo fmt --all`）

---

## 実行ルール
- PR 前に必ず以下を実行
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```
- CI（`.github/workflows/ci.yml`）でも同じチェックを実行

## Clippy 方針
- **原則**: すべての警告をエラーとして扱う（`-D warnings`）
- **許可する例外**（allow 可能だが理由必須）
  - `clippy::too_many_arguments`
    - 大きな初期化関数や設定構造体生成時など、明確な理由がある場合のみ
  - `clippy::module_name_repetitions`
    - API の一貫性維持のためにモジュール名と型名が重複する場合
- **明示的禁止**
  - `clippy::unwrap_used`（起動初期やテスト以外）
  - `clippy::expect_used`（同上）
  - `clippy::panic`（同上）

---

## rustfmt 方針
- **Edition**: 2021
- **最大行長**: 100 文字（max_width = 100）
- **インデント**: スペース 4
- **連鎖呼び出し**は改行インデントで揃える
- `use` 文はアルファベット順に整列

---

## 設定ファイル
- `rustfmt.toml` と `Clippy.toml` をリポジトリルートに配置
- 許可した例外やプロジェクト特有の設定はここに明記
- 例外を追加した場合は `docs/ai/LINTING.md` にも反映

---

## 更新ルール
- 新たに lint 警告が発生した場合、修正が第一優先
- 例外を追加する場合は **理由と期限** を `LINTING.md` に必ず記載
