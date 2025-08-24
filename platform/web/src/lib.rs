//! summary: Web最小レンダリング（共通 Renderer を利用）+ アダプタ情報を #msg/console に表示
//! path: platform/web/src/lib.rs

#![cfg(target_arch = "wasm32")]

use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

/// AppBuilder から呼ばれる統一入口（engine から呼ばれる）
pub fn run_with(
    clear_color: [f32;4],
    _prefer_high_performance: bool, // wasm では内部で None にマップ
    initial_size: Option<(u32,u32)>,
    canvas_id: Option<String>,
) -> Result<()> {
    let document = window().context("no window")?.document().context("no document")?;
    let id = canvas_id.as_deref().unwrap_or("canvas");
    let canvas: HtmlCanvasElement = document
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("no element: #{id}"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| anyhow!("canvas dyn_into failed: {:?}", e))?;

    if let Some((w,h)) = initial_size {
        canvas.set_width(w.max(1));
        canvas.set_height(h.max(1));
    }
    let (w, h) = (canvas.width().max(1), canvas.height().max(1));

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = async {
            let instance = wgpu::Instance::default();
            let surface  = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
                .map_err(|e| anyhow!("create_surface failed: {e:?}"))?;

            let init = render::init_with_surface(
                &instance,
                surface,
                (w, h),
                render::RenderInitOptions::default(),
            ).await?;

            // 任意: アダプタ情報
            let info = init.adapter_info;
            let msg = format!(
                "Adapter: {} | backend={:?} | type={:?} | vendor=0x{:04x} | device=0x{:04x}",
                info.name, info.backend, info.device_type, info.vendor, info.device
            );
            web_sys::console::log_1(&msg.clone().into());
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id("msg") {
                    el.set_text_content(Some(&msg));
                }
            }

            let mut renderer = init.renderer;
            // ★ f32 → f64 キャスト
            let clear = wgpu::Color {
                r: clear_color[0] as f64,
                g: clear_color[1] as f64,
                b: clear_color[2] as f64,
                a: clear_color[3] as f64,
            };
            renderer.render_clear(clear)?;

            Ok::<(), anyhow::Error>(())
        }.await {
            let s = format!("render error: {e:#}");
            web_sys::console::error_1(&s.clone().into());
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id("msg") {
                    el.set_text_content(Some(&s));
                }
            }
        }
    });

    Ok(())
}

/// 既定値ショートカット（任意）
pub fn run() -> Result<()> {
    run_with([0.07,0.10,0.18,1.0], true, None, None)
}
