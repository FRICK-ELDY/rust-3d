//! summary: integration_min (web) — AppBuilder で起動（canvas_id を受け取れる）
//! path: examples/web/integration_min/src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas_id: Option<String>) -> Result<(), JsValue> {
    engine::AppBuilder::new()
        .clear_color([0.07, 0.10, 0.18, 1.0])
        .canvas_id_opt(canvas_id)
        .run()
        .map_err(|e| JsValue::from_str(&format!("{e:#}")))
}
