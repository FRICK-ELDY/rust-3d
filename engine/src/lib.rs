//! summary: engine エントリ（プラットフォーム別に platform_* を薄ラップ）
//! path: engine/src/lib.rs

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop {
    use anyhow::Result;
    pub fn run() -> Result<()> { platform_desktop::run() }
}

#[cfg(target_arch = "wasm32")]
pub mod web {
    use anyhow::Result;
    pub fn run() -> Result<()> { platform_web::run() }
}

#[cfg(not(target_arch = "wasm32"))]
pub use desktop::run as run_desktop;
#[cfg(target_arch = "wasm32")]
pub use web::run as run_web;
