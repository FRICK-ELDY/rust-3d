use anyhow::Result;
use winit::dpi::PhysicalSize;
use crate::config::RenderConfig;
use crate::gpu::{instance::GpuContext, surface::SurfaceState};
use crate::scene::Scene;

pub struct Renderer {
    ctx: GpuContext,
    surface: SurfaceState,
    pub config: RenderConfig,
    clear_color: wgpu::Color,
}

impl Renderer {
    pub async fn new(
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface  = instance.create_surface(window)?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await?; 

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

        let info = ctx.adapter.get_info();
        println!("[wgpu] adapter={} backend={:?}", info.name, info.backend);

        Ok(Self {
            ctx,
            surface: surface_state,
            config: RenderConfig::default(),
            clear_color: wgpu::Color { r: 0.06, g: 0.07, b: 0.09, a: 1.0 },
        })
    }

    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.clear_color = wgpu::Color {
            r: rgba[0] as f64,
            g: rgba[1] as f64,
            b: rgba[2] as f64,
            a: rgba[3] as f64,
        };
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface.resize(&self.ctx, new_size);
    }

    pub fn render(&mut self, _scene: &Scene) -> Result<()> {
        let frame = self.surface.surface.get_current_texture()?;
        let view  = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render/encoder"),
        });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render/clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
