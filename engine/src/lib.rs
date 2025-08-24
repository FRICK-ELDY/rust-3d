//! summary: engine の統一 API（AppBuilder）— Desktop/Web を薄くラップ
//! path: engine/src/lib.rs

use anyhow::Result;

/// エンジン起動用のビルダー。どのプラットフォームでも同じコードで呼べる。
pub struct AppBuilder {
    clear_color: [f32; 4],
    prefer_high_performance: bool,
    initial_size: Option<(u32, u32)>,
    #[cfg(target_arch = "wasm32")]
    canvas_id: Option<String>,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            clear_color: [0.07, 0.10, 0.18, 1.0],
            prefer_high_performance: true,
            initial_size: None,
            #[cfg(target_arch = "wasm32")]
            canvas_id: None,
        }
    }
}

impl AppBuilder {
    pub fn new() -> Self { Self::default() }

    /// クリアカラー（RGBA, 0.0..=1.0）
    pub fn clear_color(mut self, rgba: [f32; 4]) -> Self {
        self.clear_color = rgba; self
    }

    /// 高性能GPU優先（nativeのみ実質有効、wasmは内部でNoneにマップ）
    pub fn prefer_high_performance(mut self, yes: bool) -> Self {
        self.prefer_high_performance = yes; self
    }

    /// 初期ウィンドウ/キャンバスサイズ（指定しなければ既定）
    pub fn initial_size(mut self, w: u32, h: u32) -> Self {
        self.initial_size = Some((w, h)); self
    }

    /// （wasmのみ）描画先キャンバスID
    #[cfg(target_arch = "wasm32")]
    pub fn canvas_id(mut self, id: impl Into<String>) -> Self {
        self.canvas_id = Some(id.into()); self
    }

    /// （wasmのみ）Option で渡したいとき
    #[cfg(target_arch = "wasm32")]
    pub fn canvas_id_opt(mut self, id: Option<String>) -> Self {
        self.canvas_id = id; self
    }

    /// 実行（プラットフォームに応じて分岐）
    pub fn run(self) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            platform_desktop::run_with(
                self.clear_color,
                self.prefer_high_performance,
                self.initial_size,
            )
        }

        #[cfg(target_arch = "wasm32")]
        {
            platform_web::run_with(
                self.clear_color,
                self.prefer_high_performance,
                self.initial_size,
                self.canvas_id,
            )
        }
    }
}
