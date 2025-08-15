// render/src/renderer/targets/msaa.rs
use winit::dpi::PhysicalSize;
use crate::gpu::instance::GpuContext;

pub struct MsaaTargets {
    pub color: wgpu::Texture,
    pub color_view: wgpu::TextureView,
    pub color_format: wgpu::TextureFormat,
    pub sample_count: u32,
}
impl MsaaTargets {
    pub fn create(
        ctx: &GpuContext,
        size: PhysicalSize<u32>,
        color_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let desc = wgpu::TextureDescriptor {
            label: Some("render/msaa_color"),
            size: wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: color_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let color = ctx.device.create_texture(&desc);
        let color_view = color.create_view(&wgpu::TextureViewDescriptor::default());
        Self { color, color_view, color_format, sample_count }
    }
}
