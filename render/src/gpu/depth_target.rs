use wgpu::{CompareFunction, Device, Sampler, Texture, TextureFormat, TextureView};

#[derive(Debug)]
pub struct DepthTarget {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub format: TextureFormat,
    pub sample_count: u32,
}

impl DepthTarget {
    pub const DEFAULT_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn new(device: &Device, width: u32, height: u32, format: TextureFormat, sample_count: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render/depth-texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("render/depth-sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });
        Self { texture, view, sampler, format, sample_count }
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        *self = Self::new(device, width.max(1), height.max(1), self.format, self.sample_count);
    }
}
