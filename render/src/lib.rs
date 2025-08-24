//! summary: wgpu 初期化・リサイズ・クリア描画の共通実装（Surface を受け取って統一処理）
//! path: render/src/lib.rs

use anyhow::{Context, Result};

/// 初期化オプション
#[derive(Clone, Copy, Debug)]
pub struct RenderInitOptions {
    /// 高性能GPUを優先するか（wasm では常に None にマップ）
    pub prefer_high_performance: bool,
}

impl Default for RenderInitOptions {
    fn default() -> Self {
        Self { prefer_high_performance: true }
    }
}

/// 初期化の結果（Renderer と、ログなどに使える AdapterInfo を返す）
pub struct InitOutput<'s> {
    pub renderer: Renderer<'s>,
    pub adapter_info: wgpu::AdapterInfo,
}

/// 共通レンダラ
pub struct Renderer<'s> {
    surface: wgpu::Surface<'s>,
    device:  wgpu::Device,
    queue:   wgpu::Queue,
    config:  wgpu::SurfaceConfiguration,
}

impl<'s> Renderer<'s> {
    /// ビューサイズの変更
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; }
        self.config.width  = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// 画面を指定色でクリア
    pub fn render_clear(&mut self, color: wgpu::Color) -> Result<()> {
        let frame = self.surface.get_current_texture().context("get_current_texture")?;
        let view  = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear encoder"),
        });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(color), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    /// 内部の現在サイズを取得（必要なら）
    pub fn size(&self) -> (u32, u32) { (self.config.width, self.config.height) }
}

/// Surface を渡して共通初期化を行う
pub async fn init_with_surface<'s>(
    instance: &wgpu::Instance,
    surface:  wgpu::Surface<'s>,
    initial_size: (u32, u32),
    opts: RenderInitOptions,
) -> Result<InitOutput<'s>> {
    // Windows の Chromium で noisy なので、wasm では常に None、native は設定に従う
    #[cfg(target_arch = "wasm32")]
    let power_pref = wgpu::PowerPreference::None;

    #[cfg(not(target_arch = "wasm32"))]
    let power_pref = if opts.prefer_high_performance {
        wgpu::PowerPreference::HighPerformance
    } else {
        wgpu::PowerPreference::LowPower
    };

    // アダプタ
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: power_pref,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .context("request_adapter failed")?;

    // デバイス
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        })
        .await
        .context("request_device failed")?;

    // サーフェス設定
    let caps   = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(caps.formats[0]);

    #[cfg(target_arch = "wasm32")]
    let present_mode = wgpu::PresentMode::Fifo; // WebGPUは基本Fifo

    #[cfg(not(target_arch = "wasm32"))]
    let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
        wgpu::PresentMode::Mailbox
    } else {
        wgpu::PresentMode::Fifo
    };

    let (w, h) = (initial_size.0.max(1), initial_size.1.max(1));

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width:  w,
        height: h,
        present_mode,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    Ok(InitOutput {
        renderer: Renderer { surface, device, queue, config },
        adapter_info: adapter.get_info(),
    })
}
