//! summary: integration_min の npm/ESM向けエントリ（engine::run_web を呼ぶ）
//! path: examples/web/integration_min/src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas_id: Option<String>) -> Result<(), JsValue> {
    let _ = canvas_id; // 将来の複数キャンバス対応用
    engine::run_web().map_err(|e| JsValue::from_str(&format!("{e:#}")))
}
