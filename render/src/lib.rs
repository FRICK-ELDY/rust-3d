//! summary: renderクレートのモジュール公開
//! path: render/src/lib.rs

// デスクトップ側は wasm32 以外でだけ有効
#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;

// Web側は wasm32 のときだけ有効
#[cfg(target_arch = "wasm32")]
pub mod web;
