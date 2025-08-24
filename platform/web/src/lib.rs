//! summary: Web最小レンダリング（共通 Renderer を利用）+ アダプタ情報を #msg/console に表示
//! path: platform/web/src/lib.rs

#![cfg(target_arch = "wasm32")]

use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

pub fn run() -> Result<()> {
    let document = window().context("no window")?.document().context("no document")?;
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .context("no element: #canvas")?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| anyhow!("canvas dyn_into failed: {:?}", e))?;

    let (w, h) = (canvas.width().max(1), canvas.height().max(1));

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = async {
            // Surface は platform 側で生成
            let instance = wgpu::Instance::default();
            let surface  = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
                .map_err(|e| anyhow!("create_surface failed: {e:?}"))?;

            // 共通初期化
            let init = render::init_with_surface(
                &instance,
                surface,
                (w, h),
                render::RenderInitOptions::default(), // wasm では内部で PowerPreference::None へ
            ).await?;

            // アダプタ情報を画面とコンソールへ
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

            // 1フレームだけクリア
            let mut renderer = init.renderer;
            let clear = wgpu::Color { r: 0.07, g: 0.10, b: 0.18, a: 1.0 };
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
