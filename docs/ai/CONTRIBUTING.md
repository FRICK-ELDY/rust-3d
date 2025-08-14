# Contributing Guide

このプロジェクトに参加するための流れとルールをまとめています。  
詳細なコーディング規約や設計方針は AI 用ドキュメント群に記載しています。

---

## 1. 開発フロー
1. **Issue 作成**  
   - 新機能提案やバグ報告はまず Issue を作成
   - 必要に応じてラベルを付与（`feature`, `bug`, `docs` など）
2. **フォーク & ブランチ作成**  
   - ブランチ名は `feature/xxx` または `fix/xxx` 形式
3. **実装**  
   - ローカルで lint / fmt / test を実行
4. **Pull Request 作成**  
   - テンプレートに沿って説明を書く
   - 関連する Issue 番号を必ず記載

---

## 2. 規約へのリンク
詳細は以下を参照してください。
- コーディング規約: [`docs/ai/CODING_RULES.md`](CODING_RULES.md)
- 命名規則: [`docs/ai/NAMING.md`](NAMING.md)
- Lint 方針: [`docs/ai/LINTING.md`](LINTING.md)
- 設計概要: [`docs/ai/ARCHITECTURE.md`](ARCHITECTURE.md)

---

## 3. 設計方針（概要）
- 共通処理は `game/` に配置
- 描画処理は `render/` に配置
- プラットフォーム依存処理は `platform/` に配置
- 設定値やリソースは `assets/` に配置
- テスト可能な設計（依存注入・関数分割）を心がける

---

## 4. PR & Issue のルール
- コミットメッセージは短く簡潔に
- PR 説明欄に関連 Issue 番号を記載
- バグ報告時は再現手順・環境・スクリーンショットを添付

---

## 5. 注意事項
- `unsafe` の使用は極力避ける。使用時は理由を明記（`CODING_RULES.md` 参照）
- 外部クレートは Cargo.toml に明示し、バージョンを固定
