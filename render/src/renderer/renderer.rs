// render/src/renderer/renderer.rs
use anyhow::{anyhow, Result};
use winit::dpi::PhysicalSize;

use crate::config::RenderConfig;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};
use crate::scene::Scene;
use crate::passes::Pass;

use super::frame::{acquire_frame, begin_main_pass};
use super::pass_manager::PassManager;
use super::state::RenderState;
use super::targets::RenderTargets;

pub struct Renderer {
    ctx: GpuContext,
    surface: SurfaceState,
    pub config: RenderConfig,

    state: RenderState,
    targets: RenderTargets,
    passes: PassManager,
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

        let msaa4 = cfg!(feature = "msaa4");
        let state = RenderState::new(size, msaa4);
        let targets = RenderTargets::new(&ctx, size, surface_state.config.format, state.sample_count);

        let info = ctx.adapter.get_info();
        println!("[wgpu] adapter={} backend={:?}", info.name, info.backend);

        Ok(Self {
            ctx,
            surface: surface_state,
            config: RenderConfig::default(),
            state,
            targets,
            passes: PassManager::default(),
        })
    }

    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.state.set_clear_color(rgba);
    }

    pub fn add_pass<P: Pass + Send + Sync + 'static>(&mut self, pass: P) {
        self.passes.add(pass);
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }
        self.state.size = new_size;
        self.surface.resize(&self.ctx, new_size);
        let fmt = self.surface.config.format;
        self.targets.resize(&self.ctx, self.state.size, fmt, self.state.sample_count);
    }

    pub fn render(&mut self, _scene: &Scene) -> Result<()> {
        let Some(frame) = acquire_frame(&mut self.surface, self.state.size, &self.ctx)? else { return Ok(()); };
        let swap_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render/encoder"),
        });

        let (color_view, resolve_target) = if let Some(msaa) = &self.targets.msaa {
            (&msaa.color_view, Some(&swap_view))
        } else {
            (&swap_view, None)
        };

        {
            let mut rpass = begin_main_pass(
                &mut encoder,
                color_view,
                resolve_target,
                &self.targets.depth.view,
                self.state.clear_color,
            );
            self.passes.draw_all(&mut rpass);
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
