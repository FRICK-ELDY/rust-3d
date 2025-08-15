use anyhow::Result;
use winit::dpi::PhysicalSize;
use super::instance::GpuContext;

pub struct SurfaceState {
    pub surface: wgpu::Surface<'static>,
    pub config:  wgpu::SurfaceConfiguration,
    pub format:  wgpu::TextureFormat,
}

impl SurfaceState {
    pub fn new(
        ctx: &GpuContext,
        window: &'static winit::window::Window,
        size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let surface = ctx.instance.create_surface(window)?;

        let caps = surface.get_capabilities(&ctx.adapter);
        let format = caps.formats.iter().copied().find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width:  size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&ctx.device, &config);

        Ok(Self { surface, config, format })
    }

    pub fn resize(&mut self, ctx: &GpuContext, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }
        self.config.width  = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&ctx.device, &self.config);
    }
}
