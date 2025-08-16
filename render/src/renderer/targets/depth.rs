// render/src/renderer/targets/depth.rs
use crate::gpu::instance::GpuContext;
use winit::dpi::PhysicalSize;

pub struct DepthTarget {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
    pub sample_count: u32,
}
impl DepthTarget {
    pub fn create(ctx: &GpuContext, size: PhysicalSize<u32>, sample_count: u32) -> Self {
        let format = wgpu::TextureFormat::Depth24Plus;
        let desc = wgpu::TextureDescriptor {
            label: Some("render/depth_tex"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let texture = ctx.device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            format,
            sample_count,
        }
    }
}
