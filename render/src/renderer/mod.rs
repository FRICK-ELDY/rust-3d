use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

use crate::config::RenderConfig;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};
use crate::scene::Scene;
use crate::passes::Pass;

mod frame;
mod targets;

use frame::{acquire_frame, begin_main_pass};
use targets::RenderTargets;

// パス
pub mod prelude {
    pub use crate::passes::Pass;
}

pub struct Renderer {
    ctx: GpuContext,
    surface: SurfaceState,
    pub config: RenderConfig,
    clear_color: wgpu::Color,

    size: PhysicalSize<u32>,
    targets: RenderTargets,

    // 描画パスのリスト（順序はここで管理）
    passes: Vec<Box<dyn Pass + Send + Sync>>,
}

impl Renderer {
    pub async fn new(window: &'static winit::window::Window, size: PhysicalSize<u32>) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface  = instance.create_surface(window)?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .map_err(|e| anyhow!("wgpu: request_adapter failed: {e:?}"))?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("render/device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                ..Default::default()
            }
        ).await?;

        let ctx = GpuContext { instance, adapter, device, queue };
        let surface_state = SurfaceState::new(&ctx, window, size)?;

        // MSAA の切替（feature）
        let sample_count: u32 = if cfg!(feature = "msaa4") { 4 } else { 1 };
        let targets = RenderTargets::new(&ctx, size, surface_state.config.format, sample_count);

        let info = ctx.adapter.get_info();
        println!("[wgpu] adapter={} backend={:?}", info.name, info.backend);

        Ok(Self {
            ctx,
            surface: surface_state,
            config: RenderConfig::default(),
            clear_color: wgpu::Color { r: 0.06, g: 0.07, b: 0.09, a: 1.0 },
            size,
            targets,
            passes: Vec::new(), // ここに好きなパスを push していく
        })
    }

    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.clear_color = wgpu::Color { r: rgba[0] as f64, g: rgba[1] as f64, b: rgba[2] as f64, a: rgba[3] as f64 };
    }

    /// 起動時やシーン準備後に呼び出し、描画パスを登録
    pub fn add_pass<P: Pass + Send + Sync + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }
        self.size = new_size;
        self.surface.resize(&self.ctx, new_size);

        let fmt = self.surface.config.format;
        self.targets.resize(&self.ctx, self.size, fmt);
    }

    pub fn render(&mut self, _scene: &Scene) -> Result<()> {
        let Some(frame) = acquire_frame(&mut self.surface, self.size, &self.ctx)? else { return Ok(()); };
        let swap_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render/encoder"),
        });

        let (color_view, resolve_target) = if let Some(msaa) = &self.targets.msaa {
            (&msaa.color_view, Some(&swap_view))
        } else {
            (&swap_view, None)
        };

        // メインパス開始
        {
            let mut rpass = begin_main_pass(
                &mut encoder,
                color_view,
                resolve_target,
                &self.targets.depth.view,
                self.clear_color,
            );

            // ここで順番通りに描画
            for p in &self.passes {
                p.draw(&mut rpass);
            }
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
