//! summary: Web向け最小レンダリング（#canvas をクリア）+ アダプタ情報ログ
//! path: render/src/web.rs

use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

pub fn run() -> Result<()> {
    // DOM 取得
    let document = window().context("no window")?.document().context("no document")?;
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .context("no element: #canvas")?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| anyhow!("canvas dyn_into failed: {:?}", e))?;

    // ムーブ前に一度だけ読む
    let (w, h) = (canvas.width().max(1), canvas.height().max(1));

    // 非同期初期化 + 1フレーム描画
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = async {
            let instance = wgpu::Instance::default();

            // canvas をムーブして Surface 作成
            let surface = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
                .map_err(|e| anyhow!("create_surface failed: {e:?}"))?;

            // Windows の警告ノイズを避けるため None（未指定）
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::None,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .context("request_adapter failed")?;

            // ★ アダプタ情報ログ（DevTools & 画面）
            let info = adapter.get_info();
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

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                })
                .await
                .context("request_device failed")?;

            let caps = surface.get_capabilities(&adapter);
            let format = caps.formats[0];

            surface.configure(
                &device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width: w,
                    height: h,
                    present_mode: wgpu::PresentMode::Fifo,
                    alpha_mode: caps.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );

            let frame = surface
                .get_current_texture()
                .context("get_current_texture failed")?;
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

            {
                let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("clear pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.07,
                                g: 0.10,
                                b: 0.18,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            queue.submit(Some(encoder.finish()));
            frame.present();

            Ok::<(), anyhow::Error>(())
        }
        .await
        {
            // 画面にも出す
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
