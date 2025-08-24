//! summary: ゲームエンジン統合のエントリポイント（render をラップ）
//! path: engine/src/lib.rs

use anyhow::Result;

/// いまは render を薄くラップするだけ。将来ここにゲーム状態や各種サブシステムを統合。
pub mod desktop {
    use anyhow::Result;
    /// デスクトップで最小のループを走らせる
    pub fn run() -> Result<()> {
        render::desktop::run()
    }
}

// re-export (必要に応じて)
pub use desktop::run as run_desktop;
